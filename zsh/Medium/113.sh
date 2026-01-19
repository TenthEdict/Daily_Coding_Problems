#!/bin/zsh

# Daily Coding Problem: 113
# Difficulty: Medium
# Asked by: Google
# Authored by: TenthEdict

echo "$1" | tr ' ' '\n' | tac | awk '{printf "%s%s", (NR==1 ? "" : " "), $0} END {printf "\n"}' 