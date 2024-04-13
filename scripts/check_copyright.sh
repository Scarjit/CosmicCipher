#!/usr/bin/env bash

# This script checks if $1 has a valid SPDX license identifier.
if [ -z "$1" ]; then
    echo "Usage: $0 <file>"
    exit 1
fi

if [ ! -f "$1" ]; then
    echo "File not found: $1"
    exit 1
fi

# Each file must contain: SPDX-License-Identifier: Apache-2.0 OR MIT
if ! grep -q "SPDX-License-Identifier: Apache-2.0 OR MIT" "$1"; then
    echo "File $1 does not contain a valid SPDX license identifier."
    exit 1
fi

# Each file must also contain: Copyright (c) <current_year> Ferdinand Linnenberg
if ! grep -q "Copyright (c) $(date +%Y) Ferdinand Linnenberg" "$1"; then
    echo "File $1 does not contain a valid copyright."
    exit 1
fi
