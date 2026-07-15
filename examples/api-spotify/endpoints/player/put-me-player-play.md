# PUT /me/player/play

| | |
|--|--|
| **Method** | `PUT` |
| **URL** | `/me/player/play` |
| **Full URL** | `https://api.spotify.com/v1/me/player/play` |
| **Auth** | oauth_2_0 (scopes: user-modify-playback-state) |
| **Request Content-Type** | `application/json` |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `device_id` | string | No | - |

### Payload

```jsonc
{
  "context_uri": "string",  // string, optional, Optional. Spotify URI of the context to play. Valid contexts are albums, artists & playlists. `{context_uri:"spotify:album:1Je1IMUlBXcx1Fz0WE7oPT"}`
  "uris": [  // array of string, optional
    "string"
  ],
  "offset": null,  // object, optional, Optional. Indicates from where in the context playback should start. Only available when context_uri corresponds to an album or playlist object "position" is zero based and can’t be negative. Example: `"offset": {"position": 5}` "uri" is a string representing the uri of the item to start at. Example: `"offset": {"uri": "spotify:track:1301WleyT98MSxVHPZCA6M"}`
  "position_ms": 0  // integer, optional, integer
}
```

## Response 204

Playback started

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

