# GET /browse/categories/{category_id}

> **Deprecated.** Avoid using this endpoint when an alternative exists.

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/browse/categories/{category_id}` |
| **Full URL** | `https://api.spotify.com/v1/browse/categories/{category_id}` |
| **Auth** | oauth_2_0 |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `category_id` | string | Yes | - |

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `locale` | string | No | - |

## Response 200

**Response Content-Type:** `application/json`

A category

```jsonc
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

