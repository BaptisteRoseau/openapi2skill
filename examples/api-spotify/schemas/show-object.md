# ShowObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `ShowObject` |

```jsonc
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
  "total_episodes": 0,  // integer, required, The total number of episodes in the show.
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
  }
}
```
