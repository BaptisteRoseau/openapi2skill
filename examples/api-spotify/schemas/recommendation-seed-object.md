# RecommendationSeedObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `RecommendationSeedObject` |

```jsonc
{
  "afterFilteringSize": 0,  // integer, optional, The number of tracks available after min\_\* and max\_\* filters have been applied.
  "afterRelinkingSize": 0,  // integer, optional, The number of tracks available after relinking for regional availability.
  "href": "string",  // string, optional, A link to the full track or artist data for this seed. For tracks this will be a link to a Track Object. For artists a link to an Artist Object. For genre seeds, this value will be `null`.
  "id": "string",  // string, optional, The id used to select this seed. This will be the same as the string used in the `seed_artists`, `seed_tracks` or `seed_genres` parameter.
  "initialPoolSize": 0,  // integer, optional, The number of recommended tracks available for this seed.
  "type": "string"  // string, optional, The entity type of this seed. One of `artist`, `track` or `genre`.
}
```
