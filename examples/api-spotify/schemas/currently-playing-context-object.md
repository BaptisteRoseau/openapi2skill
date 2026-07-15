# CurrentlyPlayingContextObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `CurrentlyPlayingContextObject` |

```jsonc
{
  "device": {
    "id": "string",  // string, optional, The device ID. This ID is unique and persistent to some extent. However, this is not guaranteed and any cached `device_id` should periodically be cleared out and refetched as necessary.
    "is_active": false,  // boolean, optional, If this device is the currently active device.
    "is_private_session": false,  // boolean, optional, If this device is currently in a private session.
    "is_restricted": false,  // boolean, optional, Whether controlling this device is restricted. At present if this is "true" then no Web API commands will be accepted by this device.
    "name": "Kitchen speaker",  // string, optional, A human-readable name for the device. Some devices have a name that the user can configure (e.g. \"Loudest speaker\") and some devices have a generic name associated with the manufacturer or device model.
    "type": "computer",  // string, optional, Device type, such as "computer", "smartphone" or "speaker".
    "volume_percent": 59,  // integer, optional, min: 0, max: 100, The current volume in percent.
    "supports_volume": false  // boolean, optional, If this device can be used to set the volume.
  },
  "repeat_state": "string",  // string, optional, off, track, context
  "shuffle_state": false,  // boolean, optional, If shuffle is on or off.
  "context": {
    "type": "string",  // string, optional, The object type, e.g. "artist", "playlist", "album", "show".
    "href": "string",  // string, optional, A link to the Web API endpoint providing full details of the track.
    "external_urls": {
      "spotify": "string"  // string, optional, The [Spotify URL](/documentation/web-api/concepts/spotify-uris-ids) for the object.
    },
    "uri": "string"  // string, optional, The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the context.
  },
  "timestamp": 0,  // integer, optional, Unix Millisecond Timestamp when playback state was last changed (play, pause, skip, scrub, new song, etc.).
  "progress_ms": 0,  // integer, optional, Progress into the currently playing track or episode. Can be `null`.
  "is_playing": false,  // boolean, optional, If something is currently playing, return `true`.
  "item": null,  // any, optional, The currently playing track or episode. Can be `null`.
  "currently_playing_type": "string",  // string, optional, The object type of the currently playing item. Can be one of `track`, `episode`, `ad` or `unknown`.
  "actions": {
    "interrupting_playback": false,  // boolean, optional, Interrupting playback. Optional field.
    "pausing": false,  // boolean, optional, Pausing. Optional field.
    "resuming": false,  // boolean, optional, Resuming. Optional field.
    "seeking": false,  // boolean, optional, Seeking playback location. Optional field.
    "skipping_next": false,  // boolean, optional, Skipping to the next context. Optional field.
    "skipping_prev": false,  // boolean, optional, Skipping to the previous context. Optional field.
    "toggling_repeat_context": false,  // boolean, optional, Toggling repeat context flag. Optional field.
    "toggling_shuffle": false,  // boolean, optional, Toggling shuffle flag. Optional field.
    "toggling_repeat_track": false,  // boolean, optional, Toggling repeat track flag. Optional field.
    "transferring_playback": false  // boolean, optional, Transfering playback between devices. Optional field.
  }
}
```
