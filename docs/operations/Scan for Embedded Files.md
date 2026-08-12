# Scan for Embedded Files

Scans the data for potential embedded files by looking for magic bytes at all offsets. This operation is prone to false positives.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "Scan for Embedded Files"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Images | `true` | Scan for image files |
| 2 | Video | `true` | Scan for video files |
| 3 | Audio | `true` | Scan for audio files |
| 4 | Documents | `true` | Scan for document files |
| 5 | Applications | `true` | Scan for application files |
| 6 | Archives | `true` | Scan for archive files |

