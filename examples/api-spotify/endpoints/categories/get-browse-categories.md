# GET /browse/categories

> **Deprecated.** Avoid using this endpoint when an alternative exists.

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/browse/categories` |
| **Full URL** | `https://api.spotify.com/v1/browse/categories` |
| **Auth** | oauth_2_0 |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `locale` | string | No | - |
| `limit` | integer (0..50) | No | - |
| `offset` | integer | No | - |

## Response 200

**Response Content-Type:** `application/json`

A paged set of categories

```jsonc
{
  "categories": {
    "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
    "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
    "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
    "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
    "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
    "total": 4,  // integer, required, The total number of items available to return.
    "items": [  // array of CategoryObject, required
      {
        "href": "string",  // string, required, A link to the Web API endpoint returning full details of the category.
        "icons": [  // array of ImageObject, required
          {
            "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
            "height": 300,  // integer, required, The image height in pixels.
            "width": 300  // integer, required, The image width in pixels.
          }
        ],
        "id": "equal",  // string, required, The [Spotify category ID](/documentation/web-api/concepts/spotify-uris-ids) of the category.
        "name": "EQUAL"  // string, required, The name of the category.
      }
    ]
  }
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

