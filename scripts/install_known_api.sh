#!/usr/bin/env bash
# set -euo pipefail

##############################################################
# INSTALL KNOWN API SKILLS
#
# Install all the API skills from known_openapi.csv under
# the .claude/skills directory.
# ############################################################

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/known_api_csv.sh"

output_dir() {
    skill_dir "$1"
}

install_api() {
    local name="$1"
    local token_env_var="$2"
    local url="$3"
    local -a args=("$url" --output-dir "$(output_dir "$name")")
    [[ -n "$token_env_var" ]] && args+=(--token-env-var "$token_env_var")
    openapi2skill -f "${args[@]}"
    echo "Installed $name"
}

run_parallel() {
    local -a names=() token_vars=() urls=() pids=()

    parse_csv names token_vars urls

    for i in "${!names[@]}"; do
        install_api "${names[$i]}" "${token_vars[$i]}" "${urls[$i]}" &
        pids+=($!)
    done

    for pid in "${pids[@]}"; do
        wait "$pid"
    done
}

run_parallel
