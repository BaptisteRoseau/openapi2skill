# PUT /me/player/volume

| | |
|--|--|
| **Method** | `PUT` |
| **URL** | `/me/player/volume` |
| **Full URL** | `https://api.spotify.com/v1/me/player/volume?volume_percent=50` |
| **Auth** | oauth_2_0 (scopes: user-modify-playback-state) |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `volume_percent` | integer | Yes | - |
| `device_id` | string | No | - |

## Response 204

Command sent

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

