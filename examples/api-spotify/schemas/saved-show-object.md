# SavedShowObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `SavedShowObject` |

```jsonc
{
  "added_at": "string",  // string, format: date-time, optional, The date and time the show was saved. Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC) with a zero offset: YYYY-MM-DDTHH:MM:SSZ. If the time is imprecise (for example, the date/time of an album release), an additional field indicates the precision; see for example, release_date in an album object.
  "show": {
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
}
```
