# POST /users/{user_id}/playlists

> **Deprecated.** Avoid using this endpoint when an alternative exists.

| | |
|--|--|
| **Method** | `POST` |
| **URL** | `/users/{user_id}/playlists` |
| **Full URL** | `https://api.spotify.com/v1/users/{user_id}/playlists` |
| **Auth** | oauth_2_0 (scopes: playlist-modify-public, playlist-modify-private) |
| **Request Content-Type** | `application/json` |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `user_id` | string | Yes | - |

### Payload

```jsonc
{
  "name": "string",  // string, required, The name for the new playlist, for example `"Your Coolest Playlist"`. This name does not need to be unique; a user may have several playlists with the same name.
  "public": false,  // boolean, optional, Defaults to `true`. The playlist's public/private status (if it should be added to the user's profile or not): `true` the playlist will be public, `false` the playlist will be private. To be able to create private playlists, the user must have granted the `playlist-modify-private` [scope](/documentation/web-api/concepts/scopes/#list-of-scopes). For more about public/private status, see [Working with Playlists](/documentation/web-api/concepts/playlists)
  "collaborative": false,  // boolean, optional, Defaults to `false`. If `true` the playlist will be collaborative. _**Note**: to create a collaborative playlist you must also set `public` to `false`. To create collaborative playlists you must have granted `playlist-modify-private` and `playlist-modify-public` [scopes](/documentation/web-api/concepts/scopes/#list-of-scopes)._
  "description": "string"  // string, optional, value for playlist description as displayed in Spotify Clients and in the Web API.
}
```

## Response 201

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

