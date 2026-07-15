# GET /me

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/me` |
| **Full URL** | `https://api.spotify.com/v1/me` |
| **Auth** | oauth_2_0 (scopes: user-read-private, user-read-email) |

## Response 200

**Response Content-Type:** `application/json`

A user

```jsonc
{
  "account_id": "aB3dE5fG7h",  // string, optional, A public, immutable, pseudoanonymous identifier for the user's account. Use this field for account linking rather than the `id` field, as it is stable and will not change over the lifetime of the account.
  "country": "string",  // string, optional, The country of the user, as set in the user's account profile. An [ISO 3166-1 alpha-2 country code](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2). _This field is only available when the current user has granted access to the [user-read-private](/documentation/web-api/concepts/scopes/#list-of-scopes) scope._
  "display_name": "string",  // string, optional, The name displayed on the user's profile. `null` if not available.
  "email": "string",  // string, optional, The user's email address, as entered by the user when creating their account. _**Important!** This email address is unverified; there is no proof that it actually belongs to the user._ _This field is only available when the current user has granted access to the [user-read-email](/documentation/web-api/concepts/scopes/#list-of-scopes) scope._
  "explicit_content": {
    "filter_enabled": false,  // boolean, optional, When `true`, indicates that explicit content should not be played.
    "filter_locked": false  // boolean, optional, When `true`, indicates that the explicit content setting is locked and can't be changed by the user.
  },
  "external_urls": {
    "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
  },
  "followers": {
    "href": "string",  // string, optional, This will always be set to null, as the Web API does not support it at the moment.
    "total": 0  // integer, optional, The total number of followers.
  },
  "href": "string",  // string, optional, A link to the Web API endpoint for this user.
  "id": "string",  // string, optional, The [Spotify user ID](/documentation/web-api/concepts/spotify-uris-ids) for the user. Do not use this field for account linking — use `account_id` instead, which is immutable.
  "images": [  // array of ImageObject, optional
    {
      "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
      "height": 300,  // integer, required, The image height in pixels.
      "width": 300  // integer, required, The image width in pixels.
    }
  ],
  "product": "string",  // string, optional, The user's Spotify subscription level: "premium", "free", etc. (The subscription level "open" can be considered the same as "free".) _This field is only available when the current user has granted access to the [user-read-private](/documentation/web-api/concepts/scopes/#list-of-scopes) scope._
  "type": "string",  // string, optional, The object type: "user"
  "uri": "string"  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the user.
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

