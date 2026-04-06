set allow-duplicate-recipes := true
set allow-duplicate-variables := true
set shell := ["bash", "-euo", "pipefail", "-c"]

# ---------------------------------------------------------------------------- #
#                                 DEPENDENCIES                                 #
# ---------------------------------------------------------------------------- #

# Rust: https://rust-lang.org/tools/install
cargo := require("cargo")
rustc := require("rustc")

# ---------------------------------------------------------------------------- #
#                                    RECIPES                                   #
# ---------------------------------------------------------------------------- #

# Show available commands
default:
    @just --list

# Build the program
build:
    cargo build

# Run the program
run:
    cargo run

# Run all code checks
full-check:
    cargo fmt --all --check
    cargo clippy -- --deny warnings
alias fc := full-check

full-write:
    cargo fmt --all
alias fw := full-write

# Run tests
test:
    cargo test

# ---------------------------------------------------------------------------- #
#                                   RELEASE                                    #
# ---------------------------------------------------------------------------- #

# Generate changelog from conventional commits
changelog:
    git-cliff --output CHANGELOG.md

# Check for semver violations against the latest git tag
semver-check:
    cargo semver-checks --baseline-rev "$(git describe --tags --abbrev=0)"

# Dry-run a release (default: patch bump)
release-dry-run level="patch":
    cargo release {{level}} --no-confirm

# Perform a release (patch, minor, or major)
release level="patch":
    cargo release {{level}} --execute

# ---------------------------------------------------------------------------- #
#                                  TEMPLATE                                    #
# ---------------------------------------------------------------------------- #

# Bring repo up to date with upstream template (dry-run by default; --execute to run, optional target dir)
bring-up-to-date *args:
    bash scripts/bring_up_to_date.sh {{args}}
alias butd := bring-up-to-date

# ---------------------------------------------------------------------------- #
#                              PROJECT-SPECIFIC                                #
# ---------------------------------------------------------------------------- #

sync-crates-out:
    rsync --archive -z --verbose --delete --exclude='.idea' --exclude='target' --exclude='.git' crates/cmd-spec/ ${CMD_SPEC_PATH}
    rsync --archive -z --verbose --delete --exclude='.idea' --exclude='target' --exclude='.git' crates/fluent-git/ ${FLUENT_GIT_PATH}
sync-crates-in:
    rsync --archive -z --verbose --delete --exclude='.idea' --exclude='target' --exclude='.git' ${CMD_SPEC_PATH}/ crates/cmd-spec/
    rsync --archive -z --verbose --delete --exclude='.idea' --exclude='target' --exclude='.git' ${FLUENT_GIT_PATH}/ crates/fluent-git/
