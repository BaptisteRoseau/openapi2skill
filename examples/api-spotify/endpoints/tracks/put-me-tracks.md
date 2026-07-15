# PUT /me/tracks

> **Deprecated.** Avoid using this endpoint when an alternative exists.

| | |
|--|--|
| **Method** | `PUT` |
| **URL** | `/me/tracks` |
| **Full URL** | `https://api.spotify.com/v1/me/tracks` |
| **Auth** | oauth_2_0 (scopes: user-library-modify) |
| **Request Content-Type** | `application/json` |

## Input

### Payload

```jsonc
{
  "ids": [  // array of string, optional
    "string"
  ],
  "timestamped_ids": [  // array of object, optional
    {
      "id": "string",  // string, required, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the track.
      "added_at": "string"  // string, format: date-time, required, The timestamp when the track was added to the library. Use ISO 8601 format with UTC timezone (e.g., `2023-01-15T14:30:00Z`). You can specify past timestamps to insert tracks at specific positions in the library's chronological order. The API uses minute-level granularity for ordering, though the timestamp supports millisecond precision.
    }
  ]
}
```

## Response 200

Track saved

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

