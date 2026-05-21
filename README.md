# openapi2skill

Convert an OpenAPI 3.x specification into an **AI Agent Skill** — a tree of index-linked markdown files an AI agent can navigate efficiently.

Built in Rust. Runs in milliseconds. Works on any spec reachable by file path or URL.

## Overview

`openapi2skill` parses an OpenAPI 3.x spec and writes a structured set of markdown files that Claude Code can use as a Skill. The output is designed for AI agents, not humans:

- **Token-efficient** — each category and schema has its own file; agents load only what they need.
- **Index-driven** — every directory has an `index.md` with bullet links, so agents can navigate the tree without scanning all files at once.
- **Fast** — file I/O is parallelised with Tokio; generating a full Petstore skill takes under 10 ms.

## Installation

**Download a pre-built binary** from the [latest GitHub release](https://github.com/BaptisteRoseau/openapi2skill/releases/latest) for your platform, then put it on your `PATH`.

**Or install from crates.io:**

```bash
cargo install openapi2skill
```

## Usage

```
Convert an OpenAPI 3.x specification into an AI Agent Skill — a tree of markdown files an AI agent can navigate efficiently.

Usage: openapi2skill [OPTIONS] <PATH_OR_URL>

Arguments:
  <PATH_OR_URL>  Open API source (URL or file path)

Options:
  -o, --output-dir <OUTPUT_DIR>  Output directory
  -v, --verbose                  Enable output
  -f, --force                    Remove existing output directory
  -h, --help                     Print help
  -V, --version                  Print version
```

**Examples:**

```bash
# From a local file
openapi2skill petstore.yaml

# From a URL
openapi2skill https://petstore3.swagger.io/api/v3/openapi.json

# Custom output directory
openapi2skill petstore.yaml --output-dir my_skill

# Make it directly available to Claude Code
openapi2skill https://petstore3.swagger.io/api/v3/openapi.json --output-dir .claude/skills/api_petstore --force
```

## Skill architecture

The generated skill is a self-contained directory tree. Agents start at `SKILL.md` and follow links — they never need to load the whole tree at once.

```
SKILL.md                         # frontmatter (name, description, allowed-tools) + links to indexes
│
├── authentication/
│   ├── index.md                 # bullet links to each auth scheme file
│   ├── api_key.md               # one file per security scheme
│   └── oauth.md
│
├── {tag}/                       # one directory per operation tag (snake_case); "general" if untagged
│   ├── index.md                 # bullet links to each endpoint file
│   ├── get-pets.md              # one file per operation: method + path slug
│   ├── post-pets.md
│   └── get-pets-pet-id.md
│
└── schemas/
    ├── index.md                 # bullet links to each schema file
    ├── Pet.md                   # one file per component schema
    └── Error.md
```

Each endpoint file includes the HTTP method, URL, auth requirements, path/query parameters, a typed request body example, and typed response examples — all in a compact markdown + jsonc format.

## Trubleshooting

If you encounter a warning or an error, feel free to open an issue in this repository, but always include the openapi.json link or file.

## License

MIT
