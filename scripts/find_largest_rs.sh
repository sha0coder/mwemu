#!/bin/bash

# Find the top 20 largest .rs files by lines of code in the crates/ directory
find crates/ -name "*.rs" -type f -exec wc -l {} + | sort -rn | head -21 | tail -20