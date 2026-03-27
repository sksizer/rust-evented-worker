sync-crates-out:
    rsync --archive -z --verbose --delete --exclude='.idea' --exclude='target' --exclude='.git' crates/cmd-spec/ ${CMD_SPEC_PATH}
sync-crates-in:
    rsync --archive -z --verbose --delete --exclude='.idea' --exclude='target' --exclude='.git' ${CMD_SPEC_PATH}/ crates/cmd-spec/