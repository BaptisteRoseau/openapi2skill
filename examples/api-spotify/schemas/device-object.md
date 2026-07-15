# DeviceObject

### Extensions

| Extension | Value |
|-----------|-------|
| `spotify-docs-type` | `DeviceObject` |

```jsonc
{
  "id": "string",  // string, optional, The device ID. This ID is unique and persistent to some extent. However, this is not guaranteed and any cached `device_id` should periodically be cleared out and refetched as necessary.
  "is_active": false,  // boolean, optional, If this device is the currently active device.
  "is_private_session": false,  // boolean, optional, If this device is currently in a private session.
  "is_restricted": false,  // boolean, optional, Whether controlling this device is restricted. At present if this is "true" then no Web API commands will be accepted by this device.
  "name": "Kitchen speaker",  // string, optional, A human-readable name for the device. Some devices have a name that the user can configure (e.g. \"Loudest speaker\") and some devices have a generic name associated with the manufacturer or device model.
  "type": "computer",  // string, optional, Device type, such as "computer", "smartphone" or "speaker".
  "volume_percent": 59,  // integer, optional, min: 0, max: 100, The current volume in percent.
  "supports_volume": false  // boolean, optional, If this device can be used to set the volume.
}
```
