# PUT /playlists/{playlist_id}/items

| | |
|--|--|
| **Method** | `PUT` |
| **URL** | `/playlists/{playlist_id}/items` |
| **Full URL** | `https://api.spotify.com/v1/playlists/{playlist_id}/items` |
| **Auth** | oauth_2_0 (scopes: playlist-modify-public, playlist-modify-private) |
| **Request Content-Type** | `application/json` |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `playlist_id` | string | Yes | - |

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `uris` | string | No | - |

### Payload

```jsonc
{
  "uris": [  // array of string, optional
    "string"
  ],
  "range_start": 0,  // integer, optional, The position of the first item to be reordered.
  "insert_before": 0,  // integer, optional, The position where the items should be inserted.<br/>To reorder the items to the end of the playlist, simply set _insert_before_ to the position after the last item.<br/>Examples:<br/>To reorder the first item to the last position in a playlist with 10 items, set _range_start_ to 0, and _insert_before_ to 10.<br/>To reorder the last item in a playlist with 10 items to the start of the playlist, set _range_start_ to 9, and _insert_before_ to 0.
  "range_length": 0,  // integer, optional, The amount of items to be reordered. Defaults to 1 if not set.<br/>The range of items to be reordered begins from the _range_start_ position, and includes the _range_length_ subsequent items.<br/>Example:<br/>To move the items at index 9-10 to the start of the playlist, _range_start_ is set to 9, and _range_length_ is set to 2.
  "snapshot_id": "string"  // string, optional, The playlist's snapshot ID against which you want to make the changes.
}
```

## Response 200

**Response Content-Type:** `application/json`

A snapshot ID for the playlist

```jsonc
{
  "snapshot_id": "abc"  // string, optional
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

