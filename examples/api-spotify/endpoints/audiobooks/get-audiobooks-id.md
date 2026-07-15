# GET /audiobooks/{id}

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/audiobooks/{id}` |
| **Full URL** | `https://api.spotify.com/v1/audiobooks/{id}` |
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

An Audiobook

```jsonc
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
  "total_chapters": 0,  // integer, required, The number of chapters in this audiobook.
  "chapters": {
    "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
    "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
    "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
    "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
    "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
    "total": 4,  // integer, required, The total number of items available to return.
    "items": [  // array of SimplifiedChapterObject, required
      {
        "audio_preview_url": "https://p.scdn.co/mp3-preview/2f37da1d4221f40b9d1a98cd191f4d6f1646ad17",  // string, required, A URL to a 30 second preview (MP3 format) of the chapter. `null` if not available.
        "available_markets": [  // array of string, optional
          "string"
        ],
        "chapter_number": 1,  // integer, required, The number of the chapter
        "description": "We kept on ascending, with occasional periods of quick descent, but in the main always ascending. Suddenly, I became conscious of the fact that the driver was in the act of pulling up the horses in the courtyard of a vast ruined castle, from whose tall black windows came no ray of light, and whose broken battlements showed a jagged line against the moonlit sky.\n",  // string, required, A description of the chapter. HTML tags are stripped away from this field, use `html_description` field in case HTML tags are needed.
        "html_description": "<p>We kept on ascending, with occasional periods of quick descent, but in the main always ascending. Suddenly, I became conscious of the fact that the driver was in the act of pulling up the horses in the courtyard of a vast ruined castle, from whose tall black windows came no ray of light, and whose broken battlements showed a jagged line against the moonlit sky.</p>\n",  // string, required, A description of the chapter. This field may contain HTML tags.
        "duration_ms": 1686230,  // integer, required, The chapter length in milliseconds.
        "explicit": false,  // boolean, required, Whether or not the chapter has explicit content (true = yes it does; false = no it does not OR unknown).
        "external_urls": {
          "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
        },
        "href": "https://api.spotify.com/v1/episodes/5Xt5DXGzch68nYYamXrNxZ",  // string, required, A link to the Web API endpoint providing full details of the chapter.
        "id": "5Xt5DXGzch68nYYamXrNxZ",  // string, required, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the chapter.
        "images": [  // array of ImageObject, required
          {
            "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
            "height": 300,  // integer, required, The image height in pixels.
            "width": 300  // integer, required, The image width in pixels.
          }
        ],
        "is_playable": false,  // boolean, required, True if the chapter is playable in the given market. Otherwise false.
        "languages": [  // array of string, required
          "string"
        ],
        "name": "Starting Your Own Podcast: Tips, Tricks, and Advice From Anchor Creators\n",  // string, required, The name of the chapter.
        "release_date": "1981-12-15",  // string, required, The date the chapter was first released, for example `"1981-12-15"`. Depending on the precision, it might be shown as `"1981"` or `"1981-12"`.
        "release_date_precision": "day",  // string, required, enum: "year", "month", "day", The precision with which `release_date` value is known.
        "resume_point": {
          "fully_played": false,  // boolean, optional, Whether or not the episode has been fully played by the user.
          "resume_position_ms": 0  // integer, optional, The user's most recent position in the episode in milliseconds.
        },
        "type": "episode",  // string, required, enum: "episode", The object type.
        "uri": "spotify:episode:0zLhl3WsOCQHbe1BPTiHgr",  // string, required, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the chapter.
        "restrictions": {
          "reason": "string"  // string, optional, The reason for the restriction. Supported values: - `market` - The content item is not available in the given market. - `product` - The content item is not available for the user's subscription type. - `explicit` - The content item is explicit and the user's account is set to not play explicit content. - `payment_required` - Payment is required to play the content item.  Additional reasons may be added in the future. **Note**: If you use this field, make sure that your application safely handles unknown values.
        }
      }
    ]
  }
}
```

## Response 400

**Response Content-Type:** `application/json`

The request contains malformed data in path, query parameters, or body.

```jsonc
{
  "error": { /* [ErrorObject](../../schemas/error-object.md) */ }  // object, required
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

## Response 404

**Response Content-Type:** `application/json`

The requested resource cannot be found.

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

