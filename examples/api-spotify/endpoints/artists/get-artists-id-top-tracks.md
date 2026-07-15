# GET /artists/{id}/top-tracks

> **Deprecated.** Avoid using this endpoint when an alternative exists.

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/artists/{id}/top-tracks` |
| **Full URL** | `https://api.spotify.com/v1/artists/{id}/top-tracks` |
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

## Response 200

**Response Content-Type:** `application/json`

A set of tracks

```jsonc
{
  "tracks": [  // array of TrackObject, required
    {
      "album": {
        "album_type": "compilation",  // string, required, enum: "album", "single", "compilation", The type of the album.
        "total_tracks": 9,  // integer, required, The number of tracks in the album.
        "available_markets": [  // array of string, required
          "string"
        ],
        "external_urls": {
          "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
        },
        "href": "string",  // string, required, A link to the Web API endpoint providing full details of the album.
        "id": "2up3OPMp9Tb4dAKM2erWXQ",  // string, required, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the album.
        "images": [  // array of ImageObject, required
          {
            "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
            "height": 300,  // integer, required, The image height in pixels.
            "width": 300  // integer, required, The image width in pixels.
          }
        ],
        "name": "string",  // string, required, The name of the album. In case of an album takedown, the value may be an empty string.
        "release_date": "1981-12",  // string, required, The date the album was first released.
        "release_date_precision": "year",  // string, required, enum: "year", "month", "day", The precision with which `release_date` value is known.
        "restrictions": {
          "reason": "market"  // string, optional, enum: "market", "product", "explicit", The reason for the restriction. Albums may be restricted if the content is not available in a given market, to the user's subscription type, or when the user's account is set to not play explicit content. Additional reasons may be added in the future.
        },
        "type": "album",  // string, required, enum: "album", The object type.
        "uri": "spotify:album:2up3OPMp9Tb4dAKM2erWXQ",  // string, required, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the album.
        "artists": [  // array of SimplifiedArtistObject, required
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
        ]
      },
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
      "external_ids": {
        "isrc": "string",  // string, optional, [International Standard Recording Code](http://en.wikipedia.org/wiki/International_Standard_Recording_Code)
        "ean": "string",  // string, optional, [International Article Number](http://en.wikipedia.org/wiki/International_Article_Number_%28EAN%29)
        "upc": "string"  // string, optional, [Universal Product Code](http://en.wikipedia.org/wiki/Universal_Product_Code)
      },
      "external_urls": {
        "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
      },
      "href": "string",  // string, optional, A link to the Web API endpoint providing full details of the track.
      "id": "string",  // string, optional, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the track.
      "is_playable": false,  // boolean, optional, Part of the response when [Track Relinking](/documentation/web-api/concepts/track-relinking) is applied. If `true`, the track is playable in the given market. Otherwise `false`.
      "linked_from": null,  // object, optional, Part of the response when [Track Relinking](/documentation/web-api/concepts/track-relinking) is applied, and the requested track has been replaced with different track. The track in the `linked_from` object contains information about the originally requested track.
      "restrictions": {
        "reason": "string"  // string, optional, The reason for the restriction. Supported values: - `market` - The content item is not available in the given market. - `product` - The content item is not available for the user's subscription type. - `explicit` - The content item is explicit and the user's account is set to not play explicit content.  Additional reasons may be added in the future. **Note**: If you use this field, make sure that your application safely handles unknown values.
      },
      "name": "string",  // string, optional, The name of the track.
      "popularity": 0,  // integer, optional, The popularity of the track. The value will be between 0 and 100, with 100 being the most popular.<br/>The popularity of a track is a value between 0 and 100, with 100 being the most popular. The popularity is calculated by algorithm and is based, in the most part, on the total number of plays the track has had and how recent those plays are.<br/>Generally speaking, songs that are being played a lot now will have a higher popularity than songs that were played a lot in the past. Duplicate tracks (e.g. the same track from a single and an album) are rated independently. Artist and album popularity is derived mathematically from track popularity. _**Note**: the popularity value may lag actual popularity by a few days: the value is not updated in real time._
      "preview_url": "string",  // string, optional, A link to a 30 second preview (MP3 format) of the track. Can be `null`
      "track_number": 0,  // integer, optional, The number of the track. If an album has several discs, the track number is the number on the specified disc.
      "type": "track",  // string, optional, enum: "track", The object type: "track".
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

