# PUT /me/player

| | |
|--|--|
| **Method** | `PUT` |
| **URL** | `/me/player` |
| **Full URL** | `https://api.spotify.com/v1/me/player` |
| **Auth** | oauth_2_0 (scopes: user-modify-playback-state) |
| **Request Content-Type** | `application/json` |

## Input

### Payload

```jsonc
{
  "device_ids": [  // array of string, required
    "string"
  ],
  "play": false  // boolean, optional, **true**: ensure playback happens on new device.<br/>**false** or not provided: keep the current playback state.
}
```

## Response 204

Playback transferred

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

