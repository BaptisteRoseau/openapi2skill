# GET /me/player/devices

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/me/player/devices` |
| **Full URL** | `https://api.spotify.com/v1/me/player/devices` |
| **Auth** | oauth_2_0 (scopes: user-read-playback-state) |

## Response 200

**Response Content-Type:** `application/json`

A set of devices

```jsonc
{
  "devices": [  // array of DeviceObject, required
    {
      "id": "string",  // string, optional, The device ID. This ID is unique and persistent to some extent. However, this is not guaranteed and any cached `device_id` should periodically be cleared out and refetched as necessary.
      "is_active": false,  // boolean, optional, If this device is the currently active device.
      "is_private_session": false,  // boolean, optional, If this device is currently in a private session.
      "is_restricted": false,  // boolean, optional, Whether controlling this device is restricted. At present if this is "true" then no Web API commands will be accepted by this device.
      "name": "Kitchen speaker",  // string, optional, A human-readable name for the device. Some devices have a name that the user can configure (e.g. \"Loudest speaker\") and some devices have a generic name associated with the manufacturer or device model.
      "type": "computer",  // string, optional, Device type, such as "computer", "smartphone" or "speaker".
      "volume_percent": 59,  // integer, optional, min: 0, max: 100, The current volume in percent.
      "supports_volume": false  // boolean, optional, If this device can be used to set the volume.
    }
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

