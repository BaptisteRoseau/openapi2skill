# GET /me/playlists

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/me/playlists` |
| **Full URL** | `https://api.spotify.com/v1/me/playlists` |
| **Auth** | oauth_2_0 (scopes: playlist-read-private) |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `limit` | integer (0..50) | No | - |
| `offset` | integer | No | - |

## Response 200

**Response Content-Type:** `application/json`

A paged set of playlists

See [PagingPlaylistObject](../../schemas/paging-playlist-object.md)

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

