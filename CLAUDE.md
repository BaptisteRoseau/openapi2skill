# openapi2skill

Rust CLI that converts an OpenAPI 3.x specification into a tree of markdown files formatted as a Claude Code **Skill** — a self-contained, index-linked documentation set an AI agent can navigate efficiently.

## Usage

```
openapi2skill <path-or-url> [--output-dir <dir>]
```

`<path-or-url>` accepts a file path (`.json`, `.yaml`, `.yml`) or an HTTP/HTTPS URL. The output directory defaults to the snake_case title of the API (e.g. `swagger_petstore/`).

## Output format

The `example/` directory contains hand-written reference files that define the exact format the writer must produce. **Always consult these before modifying writer logic.**

### File tree

```
SKILL.md                          ← frontmatter (name, description, allowed-tools) + links to indexes
authentication/
  index.md                        ← bullet links to auth method files
  api_key.md / oauth.md / …       ← one file per security scheme
{category}/                       ← one dir per operation tag (snake_case), "general" if untagged
  index.md                        ← bullet links to endpoint files
  {method}-{path-slug}.md         ← one file per operation
schemas/
  index.md                        ← bullet links to schema files
  {schema-name}.md                ← one file per component schema
```

### Path slug convention

`/pet/findByStatus/{petId}` → `pet-find-by-status-pet-id`
- Split on `/`, strip `{}`  braces from path params, convert camelCase segments to kebab-case, join with `-`.

### Endpoint file format

```markdown
# {METHOD} {path}

| | |
|--|--|
| **Method** | `{METHOD}` |
| **URL** | `{path}` |
| **Auth** | {scheme names and scopes, or "None"} |
| **Content-Type** | `application/json` |   ← only when request body exists

## Input

### Path Parameters
| Parameter | Type | Required | Description |

### Query Parameters
| Parameter | Type | Required | Description |

### Payload
```jsonc
{ ... }  ← jsonc with inline type comments
```

## Response {status_code}

{description}

```jsonc
{ ... }
```
```

### Schema / payload jsonc format

Fields are rendered with trailing `// type, required/optional` comments:
```jsonc
{
  "id": 0,            // integer (int64), optional
  "name": "doggie",   // string, required
  "category": {
    "id": 0,          // integer (int64), optional
    "name": "string"  // string, optional
  },
  "tags": [           // array of Tag, optional
    {
      "id": 0,        // integer (int64), optional
      "name": "string"
    }
  ],
  "status": "available"  // string, optional, enum: "available", "pending", "sold"
}
```

Rules:
- Scalars: value + comment on same line.
- Objects: opening `{` on the property line, no comment on that line, comments inside.
- Arrays: `[  // array of {item_type}, {req}` on the opening line, item example indented, `]` closing.
- Example values: use `example` field → first `enum` value → `default` → type-based fallback (`0`, `"string"`, `false`).
- `$ref` arrays: use the schema name as item type label (e.g. `array of Tag`).

### 422 responses always include a jsonc block

```jsonc
{
  "code": 422,        // integer
  "type": "string",   // string
  "message": "string" // string
}
```

## Architecture

### Crate structure

`src/lib.rs` re-exports all modules publicly so integration tests in `tests/` can access them via `openapi2skill::writer::openapi2skill`.

### Writer pipeline (`src/writer/`)

`openapi2skill()` in `mod.rs`:
1. Calls `{skill,auth,endpoint,schema}::collect_writes()` — each appends `(PathBuf, String)` pairs synchronously.
2. Spawns one `tokio::task` per pair for parallel async file I/O.

`schema::render_schema_jsonc()` is shared between `schema.rs` (schema files) and `endpoint.rs` (request/response bodies).

### Key oas3 types

```rust
use oas3::spec::{
    ObjectOrReference,   // Ref { ref_path, .. } | Object(T)
    ObjectSchema,        // schema_type: Option<SchemaTypeSet>, properties, required, items, enum_values, format, example
    Schema,              // Boolean(BooleanSchema) | Object(Box<ObjectOrReference<ObjectSchema>>)
    SchemaType,          // Boolean | Integer | Number | String | Array | Object | Null
    SchemaTypeSet,       // Single(SchemaType) | Multiple(Vec<SchemaType>)  — has is_array_or_nullable_array()
    ParameterIn,         // Path | Query | Header | Cookie
    SecurityRequirement, // newtype: SecurityRequirement(pub Map<String, Vec<String>>), access inner map via .0
    SecurityScheme,      // ApiKey { name, location } | Http { scheme, bearer_format } | OAuth2 { flows } | …
};
```

`Spec::operations()` → `impl Iterator<Item = (String, http::Method, &Operation)>` — use `method.as_str()` for "GET" etc.

`Schema::resolve(&spec)` recursively dereferences `$ref`s; result is always `Boolean` or `Object(Object(...))`, never `Object(Ref(...))`.

`SecurityRequirement.0` to iterate scheme → scopes pairs.

`Response.description` is `Option<String>` (not `String`).

### Coding Best Practices

- Prefer splitting code into modules with multiple files instead of a giant one.
- Write small helper functions instead of an all-in-one one.
- `mod.rs` and `lib.rs` should not contain custom code, only `mod` and `use` instructions.

## Tests

Integration tests only (`tests/integration.rs`). They load `tests/assets/openapi.json` (Swagger Petstore, 3.0.2), run the writer into a `tempfile::tempdir()`, and assert the expected file paths exist. No content checks.

Run: `cargo test`

## Checklist

Before returning to the user, make sure the code is formatted and linter and tests pass:

- `cargo fmt`
- `cargo clippy` → `cargo clippy --fix --lib -p openapi2skill -- ` to auto-fix issues → fix remaining issues → repeat until no issue found
- `cargo test` 
