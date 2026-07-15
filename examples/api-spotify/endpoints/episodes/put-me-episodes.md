# PUT /me/episodes

> **Deprecated.** Avoid using this endpoint when an alternative exists.

| | |
|--|--|
| **Method** | `PUT` |
| **URL** | `/me/episodes` |
| **Full URL** | `https://api.spotify.com/v1/me/episodes?ids=77o6BIVlYM3msb4MMIL1jH,0Q86acNRm6V9GYx55SXKwf` |
| **Auth** | oauth_2_0 (scopes: user-library-modify) |
| **Request Content-Type** | `application/json` |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `ids` | string | Yes | - |

### Payload

```jsonc
{
  "ids": [  // array of string, optional
    "string"
  ]
}
```

## Response 200

Episode saved

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

