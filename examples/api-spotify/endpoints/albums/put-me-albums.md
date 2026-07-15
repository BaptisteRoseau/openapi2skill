# PUT /me/albums

> **Deprecated.** Avoid using this endpoint when an alternative exists.

| | |
|--|--|
| **Method** | `PUT` |
| **URL** | `/me/albums` |
| **Full URL** | `https://api.spotify.com/v1/me/albums?ids=382ObEPsp2rxGrnsizN5TX,1A2GTWGtFfWp7KSQTwWOyo,2noRn2Aes5aoNVsU6iWThc` |
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

The album is saved

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

