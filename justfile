sync-crates-out:
    rsync --archive -z --verbose --delete --exclude='.idea' --exclude='target' --exclude='.git' crates/serde-command/ ${SERDE_COMMAND_PATH}/

sync-crates-in:
    rsync --archive -z --verbose --delete --exclude='.idea' --exclude='target' --exclude='.git' ${SERDE_COMMAND_PATH}/ crates/serde-command/