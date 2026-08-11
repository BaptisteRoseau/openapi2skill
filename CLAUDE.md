# openapi2skill

Rust CLI that converts an OpenAPI 3.x specification into a tree of markdown files formatted as a Claude Code **Skill** — a self-contained, index-linked documentation set an AI agent can navigate efficiently.

## Usage

```
openapi2skill <path-or-url> [--output-dir <dir>] [--token-env-var <NAME>]
```

`<path-or-url>` accepts a file path (`.json`, `.yaml`, `.yml`) or an HTTP/HTTPS URL. The output directory defaults to the snake_case title of the API (e.g. `swagger_petstore/`).

## Output format

### File tree

```
SKILL.md                          ← frontmatter (name, description, allowed-tools) + links to indexes
authentication/
  index.md                        ← bullet links to auth method files
  api_key.md / oauth.md / …       ← one file per security scheme
endpoints/
  index.md                        ← bullet links to category indexes
  {category}/                     ← one dir per operation tag (snake_case), "general" if untagged
    index.md                      ← bullet links to endpoint files
    {method}-{path-slug}.md       ← one file per operation
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
| **Docs** | [{description}]({url}) |       ← only when the operation has `externalDocs`

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

`src/writer/mod.rs` exports only `pipeline::openapi2skill` publicly. All other submodules are private to the `writer` module.

This is a binary-only crate — do not write `src/lib.rs`. Integration tests run the binary directly via `Command::new(env!("CARGO_BIN_EXE_openapi2skill"))`.

### Writer pipeline (`src/writer/`)

`openapi2skill()` in `pipeline.rs`:
1. Calls `{skill,manifest,auth,endpoint,schema}::Writer::collect_writes()` — each appends to a
   `Writes` collector synchronously (`Writes::push` logs and records one `(PathBuf, String)` pair).
2. Spawns one `tokio::task` per pair for parallel async file I/O.

`schema::render_schema_jsonc()` is shared between `schema/writer.rs` (schema files) and
`endpoint/body.rs` (request/response bodies).

### Shared writer helpers (`src/writer/utils/`)

Cross-cutting helpers live here rather than being re-derived per writer:

- `markdown.rs` — `Table` builder (every pipe table goes through it), `normalize_desc`,
  `desc_paragraph`, `desc_cell`, `build_index`.
- `naming.rs` — case conversions, path slugs, `endpoint_filename`, `op_category`, `category_label`.
- `refs.rs` — `schema_ref_name`, `ref_display_name`, `schema_doc_link` (the one place
  `#/components/schemas/` is parsed).
- `types.rs` — `primary_type`, `bare_type_name`, `type_label` (shared by parameter tables and
  schema comments; the `inlines_format` predicate is what differs between them).
- `servers.rs` — `effective_server_bases`.
- `writes.rs` — `Writes` collector and the `CollectWrites` trait.

### Fetcher (`src/fetcher/`)

- `loader.rs` — source resolution (URL vs path), extension detection, parsing.
- `sanitize.rs` — normalizes non-compliant real-world specs before handing them to `oas3`.

### Test fixtures

Unit tests share spec builders from `src/writer/testutil.rs` (`spec_from`, `empty_spec`,
`spec_with_servers`, `spec_with_paths`, `spec_with_schemas`, `first_operation`, `object_schema`)
instead of each module hand-rolling its own JSON spec string.

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
- `mod.rs` should not contain custom code, only `mod` and `use` instructions.

## Tests

Integration tests from `tests/integration.rs` load `tests/assets/*.[json,yaml,yml]`, runs the pipeline and ensure the skill has been generated smoothly. If you find a broken Open API 3.X spec, put it under `tests/assets/` and re-run the tests.

Otherwise, split the code in small testable functions and only write relevant happy-path and edge case scenarios unit tests.

Run: `cargo test`

## Checklist

Before returning to the user, make sure the code is formatted and linter and tests pass:

- `cargo fmt`
- `cargo clippy --fix --allow-dirty --` → `cargo clippy --fix -- ` to auto-fix issues → fix remaining issues → repeat until no issue found
- `cargo test`
