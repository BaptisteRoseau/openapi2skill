# SegmentObject

```jsonc
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
```
