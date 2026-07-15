# GET /playlists/{playlist_id}/images

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/playlists/{playlist_id}/images` |
| **Full URL** | `https://api.spotify.com/v1/playlists/{playlist_id}/images` |
| **Auth** | oauth_2_0 |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `playlist_id` | string | Yes | - |

## Response 200

**Response Content-Type:** `application/json`

A set of images

```jsonc
[
  {
    "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
    "height": 300,  // integer, required, The image height in pixels.
    "width": 300  // integer, required, The image width in pixels.
  }
]
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

