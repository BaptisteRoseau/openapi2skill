# ArtistDiscographyAlbumObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `ArtistDiscographyAlbumObject` |

```jsonc
{
  "album_type": "compilation",  // string, required, enum: "album", "single", "compilation", The type of the album.
  "total_tracks": 9,  // integer, required, The number of tracks in the album.
  "available_markets": [  // array of string, required
    "string"
  ],
  "external_urls": {
    "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
  },
  "href": "string",  // string, required, A link to the Web API endpoint providing full details of the album.
  "id": "2up3OPMp9Tb4dAKM2erWXQ",  // string, required, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the album.
  "images": [  // array of ImageObject, required
    {
      "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
      "height": 300,  // integer, required, The image height in pixels.
      "width": 300  // integer, required, The image width in pixels.
    }
  ],
  "name": "string",  // string, required, The name of the album. In case of an album takedown, the value may be an empty string.
  "release_date": "1981-12",  // string, required, The date the album was first released.
  "release_date_precision": "year",  // string, required, enum: "year", "month", "day", The precision with which `release_date` value is known.
  "restrictions": {
    "reason": "market"  // string, optional, enum: "market", "product", "explicit", The reason for the restriction. Albums may be restricted if the content is not available in a given market, to the user's subscription type, or when the user's account is set to not play explicit content. Additional reasons may be added in the future.
  },
  "type": "album",  // string, required, enum: "album", The object type.
  "uri": "spotify:album:2up3OPMp9Tb4dAKM2erWXQ",  // string, required, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the album.
  "artists": [  // array of SimplifiedArtistObject, required
    {
      "external_urls": {
        "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
      },
      "href": "string",  // string, optional, A link to the Web API endpoint providing full details of the artist.
      "id": "string",  // string, optional, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the artist.
      "name": "string",  // string, optional, The name of the artist.
      "type": "artist",  // string, optional, enum: "artist", The object type.
      "uri": "string"  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the artist.
    }
  ],
  "album_group": "compilation"  // string, required, enum: "album", "single", "compilation", "appears_on", This field describes the relationship between the artist and the album.
}
```
