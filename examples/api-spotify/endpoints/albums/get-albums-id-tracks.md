# GET /albums/{id}/tracks

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/albums/{id}/tracks` |
| **Full URL** | `https://api.spotify.com/v1/albums/{id}/tracks` |
| **Auth** | oauth_2_0 |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string | Yes | - |

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `market` | string | No | - |
| `limit` | integer (0..50) | No | - |
| `offset` | integer | No | - |

## Response 200

**Response Content-Type:** `application/json`

Pages of tracks

```jsonc
{
  "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
  "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
  "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
  "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
  "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
  "total": 4,  // integer, required, The total number of items available to return.
  "items": [  // array of SimplifiedTrackObject, required
    {
      "artists": [  // array of SimplifiedArtistObject, optional
        {
          "external_urls": {
            "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
          },
          "href": "string",  // string, optional, A link to the Web API endpoint providing full details of the artist.
          "id": "string",  // string, optional, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the artist.
          "name": "string",  // string, optional, The name of the artist.
          "type": "artist",  // string, optional, enum: "artist", The object type.
          "uri": "string"  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the artist.
        }
      ],
      "available_markets": [  // array of string, optional
        "string"
      ],
      "disc_number": 0,  // integer, optional, The disc number (usually `1` unless the album consists of more than one disc).
      "duration_ms": 0,  // integer, optional, The track length in milliseconds.
      "explicit": false,  // boolean, optional, Whether or not the track has explicit lyrics ( `true` = yes it does; `false` = no it does not OR unknown).
      "external_urls": {
        "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
      },
      "href": "string",  // string, optional, A link to the Web API endpoint providing full details of the track.
      "id": "string",  // string, optional, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the track.
      "is_playable": false,  // boolean, optional, Part of the response when [Track Relinking](/documentation/web-api/concepts/track-relinking/) is applied. If `true`, the track is playable in the given market. Otherwise `false`.
      "linked_from": {
        "external_urls": {
          "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
        },
        "href": "string",  // string, optional, A link to the Web API endpoint providing full details of the track.
        "id": "string",  // string, optional, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the track.
        "type": "string",  // string, optional, The object type: "track".
        "uri": "string"  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the track.
      },
      "restrictions": {
        "reason": "string"  // string, optional, The reason for the restriction. Supported values: - `market` - The content item is not available in the given market. - `product` - The content item is not available for the user's subscription type. - `explicit` - The content item is explicit and the user's account is set to not play explicit content.  Additional reasons may be added in the future. **Note**: If you use this field, make sure that your application safely handles unknown values.
      },
      "name": "string",  // string, optional, The name of the track.
      "preview_url": "string",  // string, optional, A URL to a 30 second preview (MP3 format) of the track.
      "track_number": 0,  // integer, optional, The number of the track. If an album has several discs, the track number is the number on the specified disc.
      "type": "string",  // string, optional, The object type: "track".
      "uri": "string",  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the track.
      "is_local": false  // boolean, optional, Whether or not the track is from a local file.
    }
  ]
}
```

## Response 401

**Response Content-Type:** `application/json`

Bad or expired token. This can happen if the user revoked a token or the access token has expired. You should re-authenticate the user.

```jsonc
{
  "error": { /* [ErrorObject](../../schemas/error-object.md) */ }  // object, required
}
```

## Response 403

**Response Content-Type:** `application/json`

Bad OAuth request (wrong consumer key, bad nonce, expired timestamp...). Unfortunately, re-authenticating the user won't help here.

```jsonc
{
  "error": { /* [ErrorObject](../../schemas/error-object.md) */ }  // object, required
}
```

## Response 429

**Response Content-Type:** `application/json`

The app has exceeded its rate limits.

```jsonc
{
  "error": { /* [ErrorObject](../../schemas/error-object.md) */ }  // object, required
}
```

