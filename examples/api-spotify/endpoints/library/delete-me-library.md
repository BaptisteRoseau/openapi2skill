# DELETE /me/library

| | |
|--|--|
| **Method** | `DELETE` |
| **URL** | `/me/library` |
| **Full URL** | `https://api.spotify.com/v1/me/library?uris=spotify:track:7a3LWj5xSFhFRYmztS8wgK,spotify:album:4aawyAB9vmqN3uQ7FjRGTy` |
| **Auth** | oauth_2_0 (scopes: user-library-modify, user-follow-modify, playlist-modify-public) |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `uris` | string | Yes | - |

## Response 200

Items removed from library

## Response 400

**Response Content-Type:** `application/json`

Bad Request. Possible reasons: missing `uris` parameter, invalid URI format, unsupported URI type, or more than 40 URIs provided.

See [ErrorObject](../../schemas/error-object.md)

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

