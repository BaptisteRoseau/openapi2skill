#!/usr/bin/env bash
set -uo pipefail

##############################################################
# SET KNOWN API SECRETS
#
# Walk through known_openapi.csv and register, for each API,
# the token of its `token_env_var` as a global sbx secret:
#
#   sbx secret set-custom --host <host> -g --env <VAR> -t <token>
#
# The host is prefilled from the API server declared in the
# generated skill, falling back to the spec URL host.
# ############################################################

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/known_api_csv.sh"

# Host part of a URL: drops the scheme, userinfo and path.
url_host() {
    local url="$1"
    url="${url#*://}"
    url="${url#*@}"
    url="${url%%/*}"
    echo "${url%%\?*}"
}

# First absolute server URL listed in the generated SKILL.md.
skill_server_url() {
    local skill_md="$(skill_dir "$1")/SKILL.md"
    [[ -f "$skill_md" ]] || return 1
    awk '
        /^\*\*Servers:\*\*/ { in_servers = 1; next }
        in_servers && /^- .*:\/\// {
            sub(/^- /, "")
            sub(/ (—|--) .*$/, "")
            print
            exit
        }
        in_servers && /^$/ { exit }
    ' "$skill_md" | head -n1
}

default_host() {
    local name="$1"
    local spec_url="$2"
    local server_url host
    server_url="$(skill_server_url "$name")"
    if [[ -n "$server_url" ]]; then
        url_host "$server_url"
        return
    fi
    host="$(url_host "$spec_url")"
}

set_secret() {
    local name="$1"
    local token_env_var="$2"
    local spec_url="$3"
    local host token

    if [[ -z "$token_env_var" ]]; then
        echo "== $name: no token_env_var in $CSV_FILE, skipped"
        return
    fi

    echo "== $name ($token_env_var)"
    read -r -e -i "$(default_host "$name" "$spec_url")" -p "   Host: " host
    read -r -s -p "   Token (empty to skip): " token
    echo

    if [[ -z "$token" ]]; then
        echo "   skipped"
        return
    fi
    if [[ -z "$host" ]]; then
        echo "   no host given, skipped"
        return
    fi

    if sbx secret set-custom --host "$host" -g --env "$token_env_var" -t "$token"; then
        echo "   secret set for $host"
    else
        echo "   sbx failed for $host" >&2
    fi
}

run() {
    local -a names=() token_vars=() urls=()

    if [[ ! -t 0 ]]; then
        echo "This script is interactive, run it from a terminal." >&2
        exit 1
    fi
    if ! command -v sbx > /dev/null; then
        echo "sbx not found in PATH, run this script on the host." >&2
        exit 1
    fi

    parse_csv names token_vars urls

    for i in "${!names[@]}"; do
        set_secret "${names[$i]}" "${token_vars[$i]}" "${urls[$i]}"
    done
}

run
