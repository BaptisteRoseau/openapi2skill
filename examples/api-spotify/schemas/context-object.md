# ContextObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `ContextObject` |

```jsonc
{
  "type": "string",  // string, optional, The object type, e.g. "artist", "playlist", "album", "show".
  "href": "string",  // string, optional, A link to the Web API endpoint providing full details of the track.
  "external_urls": {
    "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
  },
  "uri": "string"  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the context.
}
```
