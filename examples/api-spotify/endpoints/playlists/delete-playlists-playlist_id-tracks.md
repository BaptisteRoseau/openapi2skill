# DELETE /playlists/{playlist_id}/tracks

> **Deprecated.** Avoid using this endpoint when an alternative exists.

| | |
|--|--|
| **Method** | `DELETE` |
| **URL** | `/playlists/{playlist_id}/tracks` |
| **Full URL** | `https://api.spotify.com/v1/playlists/{playlist_id}/tracks` |
| **Auth** | oauth_2_0 (scopes: playlist-modify-public, playlist-modify-private) |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `playlist_id` | string | Yes | - |

### Payload

```jsonc
{
  "tracks": [  // array of object, required
    {
      "uri": "string"  // string, optional, Spotify URI
    }
  ],
  "snapshot_id": "string"  // string, optional, The playlist's snapshot ID against which you want to make the changes. The API will validate that the specified items exist and in the specified positions and make the changes, even if more recent changes have been made to the playlist.
}
```

## Response 200

**Response Content-Type:** `application/json`

A snapshot ID for the playlist

```jsonc
{
  "snapshot_id": "abc"  // string, optional
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

