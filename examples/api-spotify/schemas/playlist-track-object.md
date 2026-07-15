# PlaylistTrackObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `PlaylistTrackObject` |

```jsonc
{
  "added_at": "string",  // string, format: date-time, optional, The date and time the track or episode was added. _**Note**: some very old playlists may return `null` in this field._
  "added_by": {
    "external_urls": {
      "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
    },
    "href": "string",  // string, optional, A link to the Web API endpoint for this user.
    "id": "string",  // string, optional, The [Spotify user ID](/documentation/web-api/concepts/spotify-uris-ids) for this user.
    "type": "user",  // string, optional, enum: "user", The object type.
    "uri": "string"  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for this user.
  },
  "is_local": false,  // boolean, optional, Whether this track or episode is a [local file](/documentation/web-api/concepts/playlists/#local-files) or not.
  "item": null,  // any, optional, Information about the track or episode.
  "track": null  // any, optional, **Deprecated:** Use `item` instead. Information about the track or episode.
}
```
