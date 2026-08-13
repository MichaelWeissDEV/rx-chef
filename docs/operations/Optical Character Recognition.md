# Optical Character Recognition

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

Optical character recognition or optical character reader (OCR) is the mechanical or electronic 
        conversion of images of typed, handwritten or printed text into machine-encoded text.


        Supported image formats: png, jpg, bmp, pbm.


        Requires Tesseract library. Enable with: --features tesseract

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "Optical Character Recognition"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Show confidence | `true` | Whether to show the confidence level of the OCR |
| 2 | OCR Engine Mode | `LSTM only` | The OCR engine mode to use |

