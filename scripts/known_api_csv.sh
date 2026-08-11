#!/usr/bin/env bash

##############################################################
# KNOWN API CSV PARSING
#
# Shared by install_known_api.sh and set_known_api_secrets.sh.
# ############################################################

KNOWN_API_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_ROOT=$(git -C "$KNOWN_API_SCRIPT_DIR" rev-parse --show-toplevel)
CSV_FILE="$GIT_ROOT/known_openapi.csv"

# parse_csv NAMES_ARRAY TOKEN_VARS_ARRAY URLS_ARRAY
parse_csv() {
    local -n _names=$1
    local -n _token_vars=$2
    local -n _urls=$3
    local name token_var url
    while IFS=',' read -r name token_var url || [[ -n "$name" ]]; do
        name="${name//$'\r'/}"
        token_var="${token_var//$'\r'/}"
        url="${url//$'\r'/}"
        [[ "$name" == "name" || -z "$name" ]] && continue
        _names+=("$name")
        _token_vars+=("$token_var")
        _urls+=("$url")
    done < "$CSV_FILE"
}

skill_dir() {
    local name="$1"
    echo "$GIT_ROOT/.claude/skills/$(echo "$name" | tr '[:upper:]' '[:lower:]')"
}
