# PlaylistObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `PlaylistObject` |

```jsonc
{
  "collaborative": false,  // boolean, optional, `true` if the owner allows other users to modify the playlist.
  "description": "string",  // string, optional, The playlist description. _Only returned for modified, verified playlists, otherwise_ `null`.
  "external_urls": {
    "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
  },
  "href": "string",  // string, optional, A link to the Web API endpoint providing full details of the playlist.
  "id": "string",  // string, optional, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the playlist.
  "images": [  // array of ImageObject, optional
    {
      "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
      "height": 300,  // integer, required, The image height in pixels.
      "width": 300  // integer, required, The image width in pixels.
    }
  ],
  "name": "string",  // string, optional, The name of the playlist.
  "owner": {
    "external_urls": {
      "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
    },
    "href": "string",  // string, optional, A link to the Web API endpoint for this user.
    "id": "string",  // string, optional, The [Spotify user ID](/documentation/web-api/concepts/spotify-uris-ids) for this user.
    "type": "user",  // string, optional, enum: "user", The object type.
    "uri": "string",  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for this user.
    "display_name": "string"  // string, optional, The name displayed on the user's profile. `null` if not available.
  },
  "public": false,  // boolean, optional, The playlist's public/private status (if it is added to the user's profile): `true` the playlist is public, `false` the playlist is private, `null` the playlist status is not relevant. For more about public/private status, see [Working with Playlists](/documentation/web-api/concepts/playlists)
  "snapshot_id": "string",  // string, optional, The version identifier for the current playlist. Can be supplied in other requests to target a specific playlist version
  "items": {
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
  },
  "tracks": {
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
  },
  "type": "string",  // string, optional, The object type: "playlist"
  "uri": "string"  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the playlist.
}
```
