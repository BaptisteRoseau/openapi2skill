# ErrorObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `ErrorObject` |

```jsonc
{
  "status": 0,  // integer, required, min: 400, max: 599, The HTTP status code (also returned in the response header; see [Response Status Codes](/documentation/web-api/concepts/api-calls#response-status-codes) for more information).
  "message": "string"  // string, required, A short description of the cause of the error.
}
```
