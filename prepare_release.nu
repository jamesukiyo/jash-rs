#!/usr/bin/env nu

# workspace members with their crate names
let workspace_crates = {
    "common": "jash-common",
    "config": "jash-config", 
    "commands/cd": "jash-cd",
    "commands/ls": "jash-ls",
    "commands/pwd": "jash-pwd",
    "main": "jash"
}

print "Current versions:"
for member in ($workspace_crates | columns) {
    let current_version = (open $"($member)/Cargo.toml" | get package.version)
    let crate_name = ($workspace_crates | get $member)
    print $"  ($crate_name): ($current_version)"
}

print ""
let selected_crate = (input "Enter crate name to release (e.g., jash-ls): ")

# find the directory path for the selected crate
let crate_dir = ($workspace_crates | items | where {|item| $item.value == $selected_crate} | get 0.key)

if ($crate_dir | is-empty) {
    print $"Error: Crate '($selected_crate)' not found"
    print $"Available crates: (($workspace_crates | values) | str join ', ')"
    exit 1
}

let current_version = (open $"($crate_dir)/Cargo.toml" | get package.version)
print $"Current version of ($selected_crate): ($current_version)"

let new_version = (input "Enter new version: ")

if not ($new_version =~ '^[0-9]+\.[0-9]+\.[0-9]+$') {
    print "Error: Version must follow semantic versioning"
    exit 1
}

let tag_name = $"($selected_crate)-v($new_version)"
let existing_tags = (git tag -l | lines)
if ($existing_tags | any {|tag| $tag == $tag_name}) {
    print $"Error: Tag ($tag_name) already exists"
    exit 1
}

print $"Updating ($selected_crate) from ($current_version) to ($new_version)"

# update version in the selected crate
let cargo_path = $"($crate_dir)/Cargo.toml"
let cargo_content = (open $cargo_path --raw | str replace $'version = "($current_version)"' $'version = "($new_version)"')
$cargo_content | save -f $cargo_path

# check if any other crates depend on this one and offer to update their dependencies
let dependent_crates = []
for member in ($workspace_crates | columns) {
    let dep_cargo_path = $"($member)/Cargo.toml"
    let dep_content = (open $dep_cargo_path --raw)
    
    if ($dep_content | str contains $selected_crate) {
        $dependent_crates = ($dependent_crates | append $member)
    }
}

if not ($dependent_crates | is-empty) {
    print $"The following crates depend on ($selected_crate):"
    for dep in $dependent_crates {
        print $"  - (($workspace_crates | get $dep))"
    }
    
    let update_deps = (input "Update their dependency versions? (y/N): ")
    if ($update_deps | str downcase) == "y" {
        for dep in $dependent_crates {
            let dep_cargo_path = $"($dep)/Cargo.toml"
            let dep_content = (open $dep_cargo_path --raw)
            
            # update the dependency version
            let pattern = $'($selected_crate) = \\{ path = "[^"]*"(, version = "[^"]*")?'
            let replacement = $'($selected_crate) = { path = "' + (if $dep == "main" { "../" } else { "../../" }) + $crate_dir + $'", version = "($new_version)"'
            
            let updated_content = ($dep_content | str replace $pattern $replacement)
            $updated_content | save -f $dep_cargo_path
            print $"  Updated dependency in (($workspace_crates | get $dep))"
        }
    }
}

cargo check --quiet

git add .
git commit -m $"chore(($selected_crate)): release v($new_version)"
git tag $tag_name
git push origin HEAD
git push origin $tag_name

print $"Released ($selected_crate) v($new_version) with tag ($tag_name)"
