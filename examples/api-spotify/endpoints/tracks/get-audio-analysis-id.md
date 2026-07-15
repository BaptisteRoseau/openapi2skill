# GET /audio-analysis/{id}

> **Deprecated.** Avoid using this endpoint when an alternative exists.

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/audio-analysis/{id}` |
| **Full URL** | `https://api.spotify.com/v1/audio-analysis/{id}` |
| **Auth** | oauth_2_0 |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string | Yes | - |

## Response 200

**Response Content-Type:** `application/json`

Audio analysis for one track

```jsonc
{
  "meta": {
    "analyzer_version": "4.0.0",  // string, optional, The version of the Analyzer used to analyze this track.
    "platform": "Linux",  // string, optional, The platform used to read the track's audio data.
    "detailed_status": "OK",  // string, optional, A detailed status code for this track. If analysis data is missing, this code may explain why.
    "status_code": 0,  // integer, optional, The return code of the analyzer process. 0 if successful, 1 if any errors occurred.
    "timestamp": 1495193577,  // integer, optional, The Unix timestamp (in seconds) at which this track was analyzed.
    "analysis_time": 6.93906,  // number, optional, The amount of time taken to analyze this track.
    "input_process": "libvorbisfile L+R 44100->22050"  // string, optional, The method used to read the track's audio data.
  },
  "track": {
    "num_samples": 4585515,  // integer, optional, The exact number of audio samples analyzed from this track. See also `analysis_sample_rate`.
    "duration": 207.95985,  // number, optional, Length of the track in seconds.
    "sample_md5": "string",  // string, optional, This field will always contain the empty string.
    "offset_seconds": 0,  // integer, optional, An offset to the start of the region of the track that was analyzed. (As the entire track is analyzed, this should always be 0.)
    "window_seconds": 0,  // integer, optional, The length of the region of the track was analyzed, if a subset of the track was analyzed. (As the entire track is analyzed, this should always be 0.)
    "analysis_sample_rate": 22050,  // integer, optional, The sample rate used to decode and analyze this track. May differ from the actual sample rate of this track available on Spotify.
    "analysis_channels": 1,  // integer, optional, The number of channels used for analysis. If 1, all channels are summed together to mono before analysis.
    "end_of_fade_in": 0.0,  // number, optional, The time, in seconds, at which the track's fade-in period ends. If the track has no fade-in, this will be 0.0.
    "start_of_fade_out": 201.13705,  // number, optional, The time, in seconds, at which the track's fade-out period starts. If the track has no fade-out, this should match the track's length.
    "loudness": -5.883,  // number, format: float, optional, The overall loudness of a track in decibels (dB). Loudness values are averaged across the entire track and are useful for comparing relative loudness of tracks. Loudness is the quality of a sound that is the primary psychological correlate of physical strength (amplitude). Values typically range between -60 and 0 db.
    "tempo": 118.211,  // number, format: float, optional, The overall estimated tempo of a track in beats per minute (BPM). In musical terminology, tempo is the speed or pace of a given piece and derives directly from the average beat duration.
    "tempo_confidence": 0.73,  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the `tempo`.
    "time_signature": 4,  // integer, optional, min: 3, max: 7, An estimated time signature. The time signature (meter) is a notational convention to specify how many beats are in each bar (or measure). The time signature ranges from 3 to 7 indicating time signatures of "3/4", to "7/4".
    "time_signature_confidence": 0.994,  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the `time_signature`.
    "key": 9,  // integer, optional, min: -1, max: 11, The key the track is in. Integers map to pitches using standard [Pitch Class notation](https://en.wikipedia.org/wiki/Pitch_class). E.g. 0 = C, 1 = C♯/D♭, 2 = D, and so on. If no key was detected, the value is -1.
    "key_confidence": 0.408,  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the `key`.
    "mode": 0,  // integer, optional, Mode indicates the modality (major or minor) of a track, the type of scale from which its melodic content is derived. Major is represented by 1 and minor is 0.
    "mode_confidence": 0.485,  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the `mode`.
    "codestring": "string",  // string, optional, An [Echo Nest Musical Fingerprint (ENMFP)](https://academiccommons.columbia.edu/doi/10.7916/D8Q248M4) codestring for this track.
    "code_version": 3.15,  // number, optional, A version number for the Echo Nest Musical Fingerprint format used in the codestring field.
    "echoprintstring": "string",  // string, optional, An [EchoPrint](https://github.com/spotify/echoprint-codegen) codestring for this track.
    "echoprint_version": 4.15,  // number, optional, A version number for the EchoPrint format used in the echoprintstring field.
    "synchstring": "string",  // string, optional, A [Synchstring](https://github.com/echonest/synchdata) for this track.
    "synch_version": 1.0,  // number, optional, A version number for the Synchstring used in the synchstring field.
    "rhythmstring": "string",  // string, optional, A Rhythmstring for this track. The format of this string is similar to the Synchstring.
    "rhythm_version": 1.0  // number, optional, A version number for the Rhythmstring used in the rhythmstring field.
  },
  "bars": [  // array of TimeIntervalObject, optional
    {
      "start": 0.49567,  // number, optional, The starting point (in seconds) of the time interval.
      "duration": 2.18749,  // number, optional, The duration (in seconds) of the time interval.
      "confidence": 0.925  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the interval.
    }
  ],
  "beats": [  // array of TimeIntervalObject, optional
    {
      "start": 0.49567,  // number, optional, The starting point (in seconds) of the time interval.
      "duration": 2.18749,  // number, optional, The duration (in seconds) of the time interval.
      "confidence": 0.925  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the interval.
    }
  ],
  "sections": [  // array of SectionObject, optional
    {
      "start": 0.0,  // number, optional, The starting point (in seconds) of the section.
      "duration": 6.97092,  // number, optional, The duration (in seconds) of the section.
      "confidence": 1.0,  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the section's "designation".
      "loudness": -14.938,  // number, optional, The overall loudness of the section in decibels (dB). Loudness values are useful for comparing relative loudness of sections within tracks.
      "tempo": 113.178,  // number, optional, The overall estimated tempo of the section in beats per minute (BPM). In musical terminology, tempo is the speed or pace of a given piece and derives directly from the average beat duration.
      "tempo_confidence": 0.647,  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the tempo. Some tracks contain tempo changes or sounds which don't contain tempo (like pure speech) which would correspond to a low value in this field.
      "key": 9,  // integer, optional, The estimated overall key of the section. The values in this field ranging from 0 to 11 mapping to pitches using standard Pitch Class notation (E.g. 0 = C, 1 = C♯/D♭, 2 = D, and so on). If no key was detected, the value is -1.
      "key_confidence": 0.297,  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the key. Songs with many key changes may correspond to low values in this field.
      "mode": -1,  // number, optional, enum: -1, 0, 1, Indicates the modality (major or minor) of a section, the type of scale from which its melodic content is derived. This field will contain a 0 for "minor", a 1 for "major", or a -1 for no result. Note that the major key (e.g. C major) could more likely be confused with the minor key at 3 semitones lower (e.g. A minor) as both keys carry the same pitches.
      "mode_confidence": 0.471,  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the `mode`.
      "time_signature": 4,  // integer, optional, min: 3, max: 7, An estimated time signature. The time signature (meter) is a notational convention to specify how many beats are in each bar (or measure). The time signature ranges from 3 to 7 indicating time signatures of "3/4", to "7/4".
      "time_signature_confidence": 1.0  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the `time_signature`. Sections with time signature changes may correspond to low values in this field.
    }
  ],
  "segments": [  // array of SegmentObject, optional
    {
      "start": 0.70154,  // number, optional, The starting point (in seconds) of the segment.
      "duration": 0.19891,  // number, optional, The duration (in seconds) of the segment.
      "confidence": 0.435,  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the segmentation. Segments of the song which are difficult to logically segment (e.g: noise) may correspond to low values in this field.
      "loudness_start": -23.053,  // number, optional, The onset loudness of the segment in decibels (dB). Combined with `loudness_max` and `loudness_max_time`, these components can be used to describe the "attack" of the segment.
      "loudness_max": -14.25,  // number, optional, The peak loudness of the segment in decibels (dB). Combined with `loudness_start` and `loudness_max_time`, these components can be used to describe the "attack" of the segment.
      "loudness_max_time": 0.07305,  // number, optional, The segment-relative offset of the segment peak loudness in seconds. Combined with `loudness_start` and `loudness_max`, these components can be used to desctibe the "attack" of the segment.
      "loudness_end": 0.0,  // number, optional, The offset loudness of the segment in decibels (dB). This value should be equivalent to the loudness_start of the following segment.
      "pitches": [  // array of number, optional
        0.0
      ],
      "timbre": [  // array of number, optional
        0.0
      ]
    }
  ],
  "tatums": [  // array of TimeIntervalObject, optional
    {
      "start": 0.49567,  // number, optional, The starting point (in seconds) of the time interval.
      "duration": 2.18749,  // number, optional, The duration (in seconds) of the time interval.
      "confidence": 0.925  // number, optional, min: 0, max: 1, The confidence, from 0.0 to 1.0, of the reliability of the interval.
    }
  ]
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

