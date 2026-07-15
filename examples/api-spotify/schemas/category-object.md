# CategoryObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `CategoryObject` |

```jsonc
{
  "href": "string",  // string, required, A link to the Web API endpoint returning full details of the category.
  "icons": [  // array of ImageObject, required
    {
      "url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228\n",  // string, required, The source URL of the image.
      "height": 300,  // integer, required, The image height in pixels.
      "width": 300  // integer, required, The image width in pixels.
    }
  ],
  "id": "equal",  // string, required, The [Spotify category ID](/documentation/web-api/concepts/spotify-uris-ids) of the category.
  "name": "EQUAL"  // string, required, The name of the category.
}
```
