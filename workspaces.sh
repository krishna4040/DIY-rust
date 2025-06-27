#!/bin/bash

# Usage:
# ./workspace.sh my_workspace api core db

set -e

WORKSPACE_NAME=$1
shift
CRATES=("$@")

if [ -z "$WORKSPACE_NAME" ] || [ "${#CRATES[@]}" -eq 0 ]; then
    echo "Usage: $0 <workspace_name> <crate1> <crate2> ..."
    exit 1
fi

# Create workspace root folder
mkdir "$WORKSPACE_NAME"
cd "$WORKSPACE_NAME"

# Create root Cargo.toml with [workspace]
cat > Cargo.toml <<EOF
[workspace]
members = [
$(for crate in "${CRATES[@]}"; do echo "    \"$crate\","; done)
]
EOF

# Create each crate
for crate in "${CRATES[@]}"; do
    cargo new "$crate"
done

echo "✅ Workspace '$WORKSPACE_NAME' created with crates: ${CRATES[*]}"
