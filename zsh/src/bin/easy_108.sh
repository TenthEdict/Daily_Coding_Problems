#!/bin/zsh

# Daily Coding Problem: 108
# Difficulty: Easy
# Asked by: Google
# Authored by: TenthEdict

A="$1"
B="$2"

if [[ ${#A} -ne ${#B} ]]; then
    echo "false"
elif [[ "$A$A" == *"$B"* ]]; then
    echo "true"
else
    echo "false"
fi
