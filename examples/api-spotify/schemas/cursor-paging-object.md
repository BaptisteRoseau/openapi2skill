# CursorPagingObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `CursorPagingObject` |

```jsonc
{
  "href": "string",  // string, optional, A link to the Web API endpoint returning the full result of the request.
  "limit": 0,  // integer, optional, The maximum number of items in the response (as set in the query or by default).
  "next": "string",  // string, optional, URL to the next page of items. ( `null` if none)
  "cursors": {
    "after": "string",  // string, optional, The cursor to use as key to find the next page of items.
    "before": "string"  // string, optional, The cursor to use as key to find the previous page of items.
  },
  "total": 0  // integer, optional, The total number of items available to return.
}
```
