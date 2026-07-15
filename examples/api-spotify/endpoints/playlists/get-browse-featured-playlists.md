# GET /browse/featured-playlists

> **Deprecated.** Avoid using this endpoint when an alternative exists.

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/browse/featured-playlists` |
| **Full URL** | `https://api.spotify.com/v1/browse/featured-playlists` |
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

A paged set of playlists

See [PagingFeaturedPlaylistObject](../../schemas/paging-featured-playlist-object.md)

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

