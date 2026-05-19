#!/usr/bin/env bash
set -euo pipefail

##############################################################
# INSTALL KNOWN API SKILLS
# 
# Install all the API skills from known_openapi.csv under
# the .claude/skills directory.
# ############################################################

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_ROOT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)
CSV_FILE="$GIT_ROOT/known_openapi.csv"

output_dir() {
    local name="$1"
    echo "$GIT_ROOT/.claude/skills/api_$(echo "$name" | tr '[:upper:]' '[:lower:]')"
}

install_api() {
    local name="$1"
    local url="$2"
    openapi2skill "$url" --output-dir "$(output_dir "$name")"
    echo "Installed $name"
}

parse_csv() {
    local -n _names=$1
    local -n _urls=$2
    while IFS=',' read -r name url; do
        name="${name//$'\r'/}"
        url="${url//$'\r'/}"
        [[ "$name" == "name" || -z "$name" ]] && continue
        _names+=("$name")
        _urls+=("$url")
    done < "$CSV_FILE"
}

run_parallel() {
    local -a names=() urls=() pids=()

    parse_csv names urls

    for i in "${!names[@]}"; do
        install_api "${names[$i]}" "${urls[$i]}" &
        pids+=($!)
    done

    for pid in "${pids[@]}"; do
        wait "$pid"
    done
}

run_parallel
