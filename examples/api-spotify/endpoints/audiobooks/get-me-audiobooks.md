# GET /me/audiobooks

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/me/audiobooks` |
| **Full URL** | `https://api.spotify.com/v1/me/audiobooks` |
| **Auth** | oauth_2_0 (scopes: user-library-read) |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `limit` | integer (0..50) | No | - |
| `offset` | integer | No | - |

## Response 200

**Response Content-Type:** `application/json`

Pages of audiobooks

```jsonc
{
  "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
  "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
  "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
  "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
  "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
  "total": 4,  // integer, required, The total number of items available to return.
  "items": [  // array of SimplifiedAudiobookObject, required
    {
      "authors": [  // array of AuthorObject, required
        {
          "name": "string"  // string, optional, The name of the author.
        }
      ],
      "available_markets": [  // array of string, required
        "string"
      ],
      "copyrights": [  // array of CopyrightObject, required
        {
          "text": "string",  // string, optional, The copyright text for this content.
          "type": "string"  // string, optional, The type of copyright: `C` = the copyright, `P` = the sound recording (performance) copyright.
        }
      ],
      "description": "string",  // string, required, A description of the audiobook. HTML tags are stripped away from this field, use `html_description` field in case HTML tags are needed.
      "html_description": "string",  // string, required, A description of the audiobook. This field may contain HTML tags.
      "edition": "Unabridged",  // string, optional, The edition of the audiobook.
      "explicit": false,  // boolean, required, Whether or not the audiobook has explicit content (true = yes it does; false = no it does not OR unknown).
      "external_urls": {
        "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
      },
      "href": "string",  // string, required, A link to the Web API endpoint providing full details of the audiobook.
      "id": "string",  // string, required, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the audiobook.
      "images": [  // array of ImageObject, required
        {
          "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
          "height": 300,  // integer, required, The image height in pixels.
          "width": 300  // integer, required, The image width in pixels.
        }
      ],
      "languages": [  // array of string, required
        "string"
      ],
      "media_type": "string",  // string, required, The media type of the audiobook.
      "name": "string",  // string, required, The name of the audiobook.
      "narrators": [  // array of NarratorObject, required
        {
          "name": "string"  // string, optional, The name of the Narrator.
        }
      ],
      "publisher": "string",  // string, required, The publisher of the audiobook.
      "type": "audiobook",  // string, required, enum: "audiobook", The object type.
      "uri": "string",  // string, required, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the audiobook.
      "total_chapters": 0  // integer, required, The number of chapters in this audiobook.
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

