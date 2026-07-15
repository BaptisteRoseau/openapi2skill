# PagingPlaylistTrackObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `PagingPlaylistTrackObject` |

```jsonc
{
  "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
  "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
  "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
  "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
  "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
  "total": 4,  // integer, required, The total number of items available to return.
  "items": [  // array of PlaylistTrackObject, required
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
  ]
}
```
