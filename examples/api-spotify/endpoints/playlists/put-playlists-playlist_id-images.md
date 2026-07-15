# PUT /playlists/{playlist_id}/images

| | |
|--|--|
| **Method** | `PUT` |
| **URL** | `/playlists/{playlist_id}/images` |
| **Full URL** | `https://api.spotify.com/v1/playlists/{playlist_id}/images` |
| **Auth** | oauth_2_0 (scopes: ugc-image-upload, playlist-modify-public, playlist-modify-private) |
| **Request Content-Type** | `image/jpeg` |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `playlist_id` | string | Yes | - |

### Payload

```jsonc
"/9j/2wCEABoZGSccJz4lJT5CLy8vQkc9Ozs9R0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0cBHCcnMyYzPSYmPUc9Mj1HR0dEREdHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR0dHR//dAAQAAf/uAA5BZG9iZQBkwAAAAAH/wAARCAABAAEDACIAAREBAhEB/8QASwABAQAAAAAAAAAAAAAAAAAAAAYBAQAAAAAAAAAAAAAAAAAAAAAQAQAAAAAAAAAAAAAAAAAAAAARAQAAAAAAAAAAAAAAAAAAAAD/2gAMAwAAARECEQA/AJgAH//Z"  // string, format: byte, Base64 encoded JPEG image data, maximum payload size is 256 KB.
```

## Response 202

Image uploaded

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

