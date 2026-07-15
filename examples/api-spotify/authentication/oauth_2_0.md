# oauth_2_0

Spotify supports OAuth 2.0 for authenticating all API requests.

OAuth 2.0 authentication.

**Authorization URL:** `https://accounts.spotify.com/authorize`
**Token URL:** `https://accounts.spotify.com/api/token`

**Scopes:**

| Scope | Description |
|-------|-------------|
| `app-remote-control` | Communicate with the Spotify app on your device. |
| `playlist-read-private` | Access your private playlists. |
| `playlist-read-collaborative` | Access your collaborative playlists. |
| `playlist-modify-public` | Manage your public playlists. |
| `playlist-modify-private` | Manage your private playlists. |
| `user-library-read` | Access your saved content. |
| `user-library-modify` | Manage your saved content. |
| `user-read-private` | Access your subscription details. |
| `user-read-email` | Get your real email address. |
| `user-follow-read` | Access your followers and who you are following. |
| `user-follow-modify` | Manage your saved content. |
| `user-top-read` | Read your top artists and content. |
| `user-read-playback-position` | Read your position in content you have played. |
| `user-read-playback-state` | Read your currently playing content and Spotify Connect devices information. |
| `user-read-recently-played` | Access your recently played items. |
| `user-read-currently-playing` | Read your currently playing content. |
| `user-modify-playback-state` | Control playback on your Spotify clients and Spotify Connect devices. |
| `ugc-image-upload` | Upload images to Spotify on your behalf. |
| `streaming` | Play content and control playback on your other devices. |


```http
GET /example HTTP/1.1
Authorization: Bearer <access_token>
```
