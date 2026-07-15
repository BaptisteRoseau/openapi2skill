# PagingArtistObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `PagingArtistObject` |

```jsonc
{
  "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
  "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
  "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
  "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
  "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
  "total": 4,  // integer, required, The total number of items available to return.
  "items": [  // array of ArtistObject, required
    {
      "external_urls": {
        "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
      },
      "followers": {
        "href": "string",  // string, optional, This will always be set to null, as the Web API does not support it at the moment.
        "total": 0  // integer, optional, The total number of followers.
      },
      "genres": [  // array of string, optional
        "string"
      ],
      "href": "string",  // string, optional, A link to the Web API endpoint providing full details of the artist.
      "id": "string",  // string, optional, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the artist.
      "images": [  // array of ImageObject, optional
        {
          "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
          "height": 300,  // integer, required, The image height in pixels.
          "width": 300  // integer, required, The image width in pixels.
        }
      ],
      "name": "string",  // string, optional, The name of the artist.
      "popularity": 0,  // integer, optional, The popularity of the artist. The value will be between 0 and 100, with 100 being the most popular. The artist's popularity is calculated from the popularity of all the artist's tracks.
      "type": "artist",  // string, optional, enum: "artist", The object type.
      "uri": "string"  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the artist.
    }
  ]
}
```
