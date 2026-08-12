# Optical Character Recognition

!!! warning "Experimental / known broken"

    This operation is marked as broken in the runtime registry.

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

