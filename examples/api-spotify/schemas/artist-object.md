# ArtistObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `ArtistObject` |

```jsonc
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
```
