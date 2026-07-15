# PagingSavedAlbumObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `PagingSavedAlbumObject` |

```jsonc
{
  "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
  "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
  "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
  "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
  "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
  "total": 4,  // integer, required, The total number of items available to return.
  "items": [  // array of SavedAlbumObject, required
    {
      "added_at": "string",  // string, format: date-time, optional, The date and time the album was saved Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC) with a zero offset: YYYY-MM-DDTHH:MM:SSZ. If the time is imprecise (for example, the date/time of an album release), an additional field indicates the precision; see for example, release_date in an album object.
      "album": {
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
        "tracks": {
          "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
          "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
          "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
          "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
          "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
          "total": 4,  // integer, required, The total number of items available to return.
          "items": [  // array of SimplifiedTrackObject, required
            {
              "artists": [  // array of SimplifiedArtistObject, optional
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
              "available_markets": [  // array of string, optional
                "string"
              ],
              "disc_number": 0,  // integer, optional, The disc number (usually `1` unless the album consists of more than one disc).
              "duration_ms": 0,  // integer, optional, The track length in milliseconds.
              "explicit": false,  // boolean, optional, Whether or not the track has explicit lyrics ( `true` = yes it does; `false` = no it does not OR unknown).
              "external_urls": {
                "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
              },
              "href": "string",  // string, optional, A link to the Web API endpoint providing full details of the track.
              "id": "string",  // string, optional, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the track.
              "is_playable": false,  // boolean, optional, Part of the response when [Track Relinking](/documentation/web-api/concepts/track-relinking/) is applied. If `true`, the track is playable in the given market. Otherwise `false`.
              "linked_from": {
                "external_urls": {
                  "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
                },
                "href": "string",  // string, optional, A link to the Web API endpoint providing full details of the track.
                "id": "string",  // string, optional, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the track.
                "type": "string",  // string, optional, The object type: "track".
                "uri": "string"  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the track.
              },
              "restrictions": {
                "reason": "string"  // string, optional, The reason for the restriction. Supported values: - `market` - The content item is not available in the given market. - `product` - The content item is not available for the user's subscription type. - `explicit` - The content item is explicit and the user's account is set to not play explicit content.  Additional reasons may be added in the future. **Note**: If you use this field, make sure that your application safely handles unknown values.
              },
              "name": "string",  // string, optional, The name of the track.
              "preview_url": "string",  // string, optional, A URL to a 30 second preview (MP3 format) of the track.
              "track_number": 0,  // integer, optional, The number of the track. If an album has several discs, the track number is the number on the specified disc.
              "type": "string",  // string, optional, The object type: "track".
              "uri": "string",  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the track.
              "is_local": false  // boolean, optional, Whether or not the track is from a local file.
            }
          ]
        },
        "copyrights": [  // array of CopyrightObject, required
          {
            "text": "string",  // string, optional, The copyright text for this content.
            "type": "string"  // string, optional, The type of copyright: `C` = the copyright, `P` = the sound recording (performance) copyright.
          }
        ],
        "external_ids": {
          "isrc": "string",  // string, optional, [International Standard Recording Code](http://en.wikipedia.org/wiki/International_Standard_Recording_Code)
          "ean": "string",  // string, optional, [International Article Number](http://en.wikipedia.org/wiki/International_Article_Number_%28EAN%29)
          "upc": "string"  // string, optional, [Universal Product Code](http://en.wikipedia.org/wiki/Universal_Product_Code)
        },
        "genres": [  // array of string, required
          "string"
        ],
        "label": "string",  // string, required, The label associated with the album.
        "popularity": 0  // integer, required, The popularity of the album. The value will be between 0 and 100, with 100 being the most popular.
      }
    }
  ]
}
```
