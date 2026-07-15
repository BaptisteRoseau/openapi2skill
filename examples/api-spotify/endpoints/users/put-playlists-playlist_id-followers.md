# PUT /playlists/{playlist_id}/followers

> **Deprecated.** Avoid using this endpoint when an alternative exists.

| | |
|--|--|
| **Method** | `PUT` |
| **URL** | `/playlists/{playlist_id}/followers` |
| **Full URL** | `https://api.spotify.com/v1/playlists/{playlist_id}/followers` |
| **Auth** | oauth_2_0 (scopes: playlist-modify-public, playlist-modify-private) |
| **Request Content-Type** | `application/json` |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `playlist_id` | string | Yes | - |

### Payload

```jsonc
{
  "public": false  // boolean, optional, Defaults to `true`. If `true` the playlist will be included in user's public playlists (added to profile), if `false` it will remain private. For more about public/private status, see [Working with Playlists](/documentation/web-api/concepts/playlists)
}
```

## Response 200

Playlist followed

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

