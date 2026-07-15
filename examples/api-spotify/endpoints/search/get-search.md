# GET /search

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/search` |
| **Full URL** | `https://api.spotify.com/v1/search?q=remaster%20track:Doxy%20artist:Miles%20Davis&type=string` |
| **Auth** | oauth_2_0 |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `q` | string | Yes | - |
| `type` | array | Yes | - |
| `market` | string | No | - |
| `limit` | integer (0..10) | No | - |
| `offset` | integer (0..1000) | No | - |
| `include_external` | string (`audio`) | No | - |

## Response 200

**Response Content-Type:** `application/json`

Search response

```jsonc
{
  "tracks": {
    "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
    "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
    "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
    "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
    "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
    "total": 4,  // integer, required, The total number of items available to return.
    "items": [  // array of TrackObject, required
      {
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
          ]
        },
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
        "external_ids": {
          "isrc": "string",  // string, optional, [International Standard Recording Code](http://en.wikipedia.org/wiki/International_Standard_Recording_Code)
          "ean": "string",  // string, optional, [International Article Number](http://en.wikipedia.org/wiki/International_Article_Number_%28EAN%29)
          "upc": "string"  // string, optional, [Universal Product Code](http://en.wikipedia.org/wiki/Universal_Product_Code)
        },
        "external_urls": {
          "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
        },
        "href": "string",  // string, optional, A link to the Web API endpoint providing full details of the track.
        "id": "string",  // string, optional, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the track.
        "is_playable": false,  // boolean, optional, Part of the response when [Track Relinking](/documentation/web-api/concepts/track-relinking) is applied. If `true`, the track is playable in the given market. Otherwise `false`.
        "linked_from": null,  // object, optional, Part of the response when [Track Relinking](/documentation/web-api/concepts/track-relinking) is applied, and the requested track has been replaced with different track. The track in the `linked_from` object contains information about the originally requested track.
        "restrictions": {
          "reason": "string"  // string, optional, The reason for the restriction. Supported values: - `market` - The content item is not available in the given market. - `product` - The content item is not available for the user's subscription type. - `explicit` - The content item is explicit and the user's account is set to not play explicit content.  Additional reasons may be added in the future. **Note**: If you use this field, make sure that your application safely handles unknown values.
        },
        "name": "string",  // string, optional, The name of the track.
        "popularity": 0,  // integer, optional, The popularity of the track. The value will be between 0 and 100, with 100 being the most popular.<br/>The popularity of a track is a value between 0 and 100, with 100 being the most popular. The popularity is calculated by algorithm and is based, in the most part, on the total number of plays the track has had and how recent those plays are.<br/>Generally speaking, songs that are being played a lot now will have a higher popularity than songs that were played a lot in the past. Duplicate tracks (e.g. the same track from a single and an album) are rated independently. Artist and album popularity is derived mathematically from track popularity. _**Note**: the popularity value may lag actual popularity by a few days: the value is not updated in real time._
        "preview_url": "string",  // string, optional, A link to a 30 second preview (MP3 format) of the track. Can be `null`
        "track_number": 0,  // integer, optional, The number of the track. If an album has several discs, the track number is the number on the specified disc.
        "type": "track",  // string, optional, enum: "track", The object type: "track".
        "uri": "string",  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the track.
        "is_local": false  // boolean, optional, Whether or not the track is from a local file.
      }
    ]
  },
  "artists": {
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
  },
  "albums": {
    "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
    "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
    "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
    "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
    "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
    "total": 4,  // integer, required, The total number of items available to return.
    "items": [  // array of SimplifiedAlbumObject, required
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
        ]
      }
    ]
  },
  "playlists": { /* [PagingPlaylistObject](../../schemas/paging-playlist-object.md) */ },  // object, optional
  "shows": {
    "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
    "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
    "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
    "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
    "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
    "total": 4,  // integer, required, The total number of items available to return.
    "items": [  // array of SimplifiedShowObject, required
      {
        "available_markets": [  // array of string, required
          "string"
        ],
        "copyrights": [  // array of CopyrightObject, required
          {
            "text": "string",  // string, optional, The copyright text for this content.
            "type": "string"  // string, optional, The type of copyright: `C` = the copyright, `P` = the sound recording (performance) copyright.
          }
        ],
        "description": "string",  // string, required, A description of the show. HTML tags are stripped away from this field, use `html_description` field in case HTML tags are needed.
        "html_description": "string",  // string, required, A description of the show. This field may contain HTML tags.
        "explicit": false,  // boolean, required, Whether or not the show has explicit content (true = yes it does; false = no it does not OR unknown).
        "external_urls": {
          "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
        },
        "href": "string",  // string, required, A link to the Web API endpoint providing full details of the show.
        "id": "string",  // string, required, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the show.
        "images": [  // array of ImageObject, required
          {
            "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
            "height": 300,  // integer, required, The image height in pixels.
            "width": 300  // integer, required, The image width in pixels.
          }
        ],
        "is_externally_hosted": false,  // boolean, required, True if all of the shows episodes are hosted outside of Spotify's CDN. This field might be `null` in some cases.
        "languages": [  // array of string, required
          "string"
        ],
        "media_type": "string",  // string, required, The media type of the show.
        "name": "string",  // string, required, The name of the episode.
        "publisher": "string",  // string, required, The publisher of the show.
        "type": "show",  // string, required, enum: "show", The object type.
        "uri": "string",  // string, required, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the show.
        "total_episodes": 0  // integer, required, The total number of episodes in the show.
      }
    ]
  },
  "episodes": {
    "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
    "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
    "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
    "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
    "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
    "total": 4,  // integer, required, The total number of items available to return.
    "items": [  // array of SimplifiedEpisodeObject, required
      {
        "audio_preview_url": "https://p.scdn.co/mp3-preview/2f37da1d4221f40b9d1a98cd191f4d6f1646ad17",  // string, required, A URL to a 30 second preview (MP3 format) of the episode. `null` if not available.
        "description": "A Spotify podcast sharing fresh insights on important topics of the moment—in a way only Spotify can. You’ll hear from experts in the music, podcast and tech industries as we discover and uncover stories about our work and the world around us.\n",  // string, required, A description of the episode. HTML tags are stripped away from this field, use `html_description` field in case HTML tags are needed.
        "html_description": "<p>A Spotify podcast sharing fresh insights on important topics of the moment—in a way only Spotify can. You’ll hear from experts in the music, podcast and tech industries as we discover and uncover stories about our work and the world around us.</p>\n",  // string, required, A description of the episode. This field may contain HTML tags.
        "duration_ms": 1686230,  // integer, required, The episode length in milliseconds.
        "explicit": false,  // boolean, required, Whether or not the episode has explicit content (true = yes it does; false = no it does not OR unknown).
        "external_urls": {
          "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
        },
        "href": "https://api.spotify.com/v1/episodes/5Xt5DXGzch68nYYamXrNxZ",  // string, required, A link to the Web API endpoint providing full details of the episode.
        "id": "5Xt5DXGzch68nYYamXrNxZ",  // string, required, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the episode.
        "images": [  // array of ImageObject, required
          {
            "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
            "height": 300,  // integer, required, The image height in pixels.
            "width": 300  // integer, required, The image width in pixels.
          }
        ],
        "is_externally_hosted": false,  // boolean, required, True if the episode is hosted outside of Spotify's CDN.
        "is_playable": false,  // boolean, required, True if the episode is playable in the given market. Otherwise false.
        "language": "en",  // string, optional, The language used in the episode, identified by a [ISO 639](https://en.wikipedia.org/wiki/ISO_639) code. This field is deprecated and might be removed in the future. Please use the `languages` field instead.
        "languages": [  // array of string, required
          "string"
        ],
        "name": "Starting Your Own Podcast: Tips, Tricks, and Advice From Anchor Creators\n",  // string, required, The name of the episode.
        "release_date": "1981-12-15",  // string, required, The date the episode was first released, for example `"1981-12-15"`. Depending on the precision, it might be shown as `"1981"` or `"1981-12"`.
        "release_date_precision": "day",  // string, required, enum: "year", "month", "day", The precision with which `release_date` value is known.
        "resume_point": {
          "fully_played": false,  // boolean, optional, Whether or not the episode has been fully played by the user.
          "resume_position_ms": 0  // integer, optional, The user's most recent position in the episode in milliseconds.
        },
        "type": "episode",  // string, required, enum: "episode", The object type.
        "uri": "spotify:episode:0zLhl3WsOCQHbe1BPTiHgr",  // string, required, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the episode.
        "restrictions": {
          "reason": "string"  // string, optional, The reason for the restriction. Supported values: - `market` - The content item is not available in the given market. - `product` - The content item is not available for the user's subscription type. - `explicit` - The content item is explicit and the user's account is set to not play explicit content.  Additional reasons may be added in the future. **Note**: If you use this field, make sure that your application safely handles unknown values.
        }
      }
    ]
  },
  "audiobooks": {
    "href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20\n",  // string, required, A link to the Web API endpoint returning the full result of the request
    "limit": 20,  // integer, required, The maximum number of items in the response (as set in the query or by default).
    "next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the next page of items. ( `null` if none)
    "offset": 0,  // integer, required, The offset of the items returned (as set in the query or by default)
    "previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",  // string, required, URL to the previous page of items. ( `null` if none)
    "total": 4,  // integer, required, The total number of items available to return.
    "items": [  // array of SimplifiedAudiobookObject, required
      {
        "authors": [  // array of AuthorObject, required
          {
            "name": "string"  // string, optional, The name of the author.
          }
        ],
        "available_markets": [  // array of string, required
          "string"
        ],
        "copyrights": [  // array of CopyrightObject, required
          {
            "text": "string",  // string, optional, The copyright text for this content.
            "type": "string"  // string, optional, The type of copyright: `C` = the copyright, `P` = the sound recording (performance) copyright.
          }
        ],
        "description": "string",  // string, required, A description of the audiobook. HTML tags are stripped away from this field, use `html_description` field in case HTML tags are needed.
        "html_description": "string",  // string, required, A description of the audiobook. This field may contain HTML tags.
        "edition": "Unabridged",  // string, optional, The edition of the audiobook.
        "explicit": false,  // boolean, required, Whether or not the audiobook has explicit content (true = yes it does; false = no it does not OR unknown).
        "external_urls": {
          "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
        },
        "href": "string",  // string, required, A link to the Web API endpoint providing full details of the audiobook.
        "id": "string",  // string, required, The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the audiobook.
        "images": [  // array of ImageObject, required
          {
            "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
            "height": 300,  // integer, required, The image height in pixels.
            "width": 300  // integer, required, The image width in pixels.
          }
        ],
        "languages": [  // array of string, required
          "string"
        ],
        "media_type": "string",  // string, required, The media type of the audiobook.
        "name": "string",  // string, required, The name of the audiobook.
        "narrators": [  // array of NarratorObject, required
          {
            "name": "string"  // string, optional, The name of the Narrator.
          }
        ],
        "publisher": "string",  // string, required, The publisher of the audiobook.
        "type": "audiobook",  // string, required, enum: "audiobook", The object type.
        "uri": "string",  // string, required, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the audiobook.
        "total_chapters": 0  // integer, required, The number of chapters in this audiobook.
      }
    ]
  }
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

