# GET /me/player/queue

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/me/player/queue` |
| **Full URL** | `https://api.spotify.com/v1/me/player/queue` |
| **Auth** | oauth_2_0 (scopes: user-read-currently-playing, user-read-playback-state) |

## Response 200

**Response Content-Type:** `application/json`

Information about the queue

```jsonc
{
  "currently_playing": null,  // any, optional, The currently playing track or episode. Can be `null`.
  "queue": [  // array of object, optional
    null
  ]
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

