# GET /playlists/{playlist_id}

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/playlists/{playlist_id}` |
| **Full URL** | `https://api.spotify.com/v1/playlists/{playlist_id}` |
| **Auth** | oauth_2_0 |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `playlist_id` | string | Yes | - |

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `market` | string | No | - |
| `fields` | string | No | - |
| `additional_types` | string | No | - |

## Response 200

**Response Content-Type:** `application/json`

A playlist

See [PlaylistObject](../../schemas/playlist-object.md)

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

