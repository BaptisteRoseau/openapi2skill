# SectionObject

```jsonc
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
```
