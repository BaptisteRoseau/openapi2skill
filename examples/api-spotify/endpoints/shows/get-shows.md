# GET /shows

> **Deprecated.** Avoid using this endpoint when an alternative exists.

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/shows` |
| **Full URL** | `https://api.spotify.com/v1/shows?ids=5CfCWKI5pZ28U0uOzXkDHe,5as3aKmN2k11yfDDDSrvaZ` |
| **Auth** | oauth_2_0 |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `market` | string | No | - |
| `ids` | string | Yes | - |

## Response 200

**Response Content-Type:** `application/json`

A set of shows

```jsonc
{
  "shows": [  // array of SimplifiedShowObject, required
    {
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

