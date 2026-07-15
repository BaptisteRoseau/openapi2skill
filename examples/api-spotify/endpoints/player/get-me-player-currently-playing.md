# GET /me/player/currently-playing

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/me/player/currently-playing` |
| **Full URL** | `https://api.spotify.com/v1/me/player/currently-playing` |
| **Auth** | oauth_2_0 (scopes: user-read-currently-playing) |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `market` | string | No | - |
| `additional_types` | string | No | - |

## Response 200

**Response Content-Type:** `application/json`

Information about the currently playing track

See [CurrentlyPlayingContextObject](../../schemas/currently-playing-context-object.md)

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

