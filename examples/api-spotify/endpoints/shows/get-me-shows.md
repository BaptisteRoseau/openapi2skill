# GET /me/shows

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/me/shows` |
| **Full URL** | `https://api.spotify.com/v1/me/shows` |
| **Auth** | oauth_2_0 (scopes: user-library-read) |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `limit` | integer (0..50) | No | - |
| `offset` | integer | No | - |

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

## Response 200

**Response Content-Type:** `application/json`

Pages of shows

```jsonc
{
  "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
  "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
  "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
  "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
  "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
  "total": 4,  // integer, required, The total number of items available to return.
  "items": [  // array of SavedShowObject, required
    {
      "added_at": "string",  // string, format: date-time, optional, The date and time the show was saved. Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC) with a zero offset: YYYY-MM-DDTHH:MM:SSZ. If the time is imprecise (for example, the date/time of an album release), an additional field indicates the precision; see for example, release_date in an album object.
      "show": {
        "available_markets": [  // array of string, required
          "string"
        ],
        "copyrights": [  // array of CopyrightObject, required
          {
            "text": "string",  // string, optional, The copyright text for this content.
            "type": "string"  // string, optional, The type of copyright: `C` = the copyright, `P` = the sound recording (performance) copyright.
          }
        ],
        "description": "string",  // string, required, A description of the show. HTML tags are stripped away from this field, use `html_description` field in case HTML tags are needed.
        "html_description": "string",  // string, required, A description of the show. This field may contain HTML tags.
        "explicit": false,  // boolean, required, Whether or not the show has explicit content (true = yes it does; false = no it does not OR unknown).
        "external_urls": {
          "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
        },
        "href": "string",  // string, required, A link to the Web API endpoint providing full details of the show.
        "id": "string",  // string, required, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the show.
        "images": [  // array of ImageObject, required
          {
            "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
            "height": 300,  // integer, required, The image height in pixels.
            "width": 300  // integer, required, The image width in pixels.
          }
        ],
        "is_externally_hosted": false,  // boolean, required, True if all of the shows episodes are hosted outside of Spotify's CDN. This field might be `null` in some cases.
        "languages": [  // array of string, required
          "string"
        ],
        "media_type": "string",  // string, required, The media type of the show.
        "name": "string",  // string, required, The name of the episode.
        "publisher": "string",  // string, required, The publisher of the show.
        "type": "show",  // string, required, enum: "show", The object type.
        "uri": "string",  // string, required, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the show.
        "total_episodes": 0  // integer, required, The total number of episodes in the show.
      }
    }
  ]
}
```

