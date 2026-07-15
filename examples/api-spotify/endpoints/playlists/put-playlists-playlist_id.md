# PUT /playlists/{playlist_id}

| | |
|--|--|
| **Method** | `PUT` |
| **URL** | `/playlists/{playlist_id}` |
| **Full URL** | `https://api.spotify.com/v1/playlists/{playlist_id}` |
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
  "name": "string",  // string, optional, The new name for the playlist, for example `"My New Playlist Title"`
  "public": false,  // boolean, optional, The playlist's public/private status (if it should be added to the user's profile or not): `true` the playlist will be public, `false` the playlist will be private, `null` the playlist status is not relevant. For more about public/private status, see [Working with Playlists](/documentation/web-api/concepts/playlists)
  "collaborative": false,  // boolean, optional, If `true`, the playlist will become collaborative and other users will be able to modify the playlist in their Spotify client. <br/> _**Note**: You can only set `collaborative` to `true` on non-public playlists._
  "description": "string"  // string, optional, Value for playlist description as displayed in Spotify Clients and in the Web API.
}
```

## Response 200

Playlist updated

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

