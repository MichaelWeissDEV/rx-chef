/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse colour code operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse colour code operation
pub struct ParseColourCode;

impl Operation for ParseColourCode {
    fn name(&self) -> &'static str {
        "Parse colour code"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts a colour code in a standard format to other standard formats and displays the colour itself.<br><br><strong>Example inputs</strong><ul><li><code>#d9edf7</code></li><li><code>rgba(217,237,247,1)</code></li><li><code>hsla(200,65%,91%,1)</code></li><li><code>cmyk(0.12, 0.04, 0.00, 0.03)</code></li></ul>"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Html
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let (r, g, b, a) = {
            let hex_regex = Regex::new(r"(?i)#([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})").unwrap();
            let rgb_regex = Regex::new(r"(?i)rgba?\((\d{1,3}(?:\.\d+)?),\s?(\d{1,3}(?:\.\d+)?),\s?(\d{1,3}(?:\.\d+)?)(?:,\s?(\d(?:\.\d+)?))?\)").unwrap();
            let hsl_regex = Regex::new(r"(?i)hsla?\((\d{1,3}(?:\.\d+)?),\s?(\d{1,3}(?:\.\d+)?)%,\s?(\d{1,3}(?:\.\d+)?)%(?:,\s?(\d(?:\.\d+)?))?\)").unwrap();
            let cmyk_regex = Regex::new(
                r"(?i)cmyk\((\d(?:\.\d+)?),\s?(\d(?:\.\d+)?),\s?(\d(?:\.\d+)?),\s?(\d(?:\.\d+)?)\)",
            )
            .unwrap();

            if let Some(m) = hex_regex.captures(&input_str) {
                let r = u8::from_str_radix(&m[1], 16).unwrap_or(0) as f64;
                let g = u8::from_str_radix(&m[2], 16).unwrap_or(0) as f64;
                let b = u8::from_str_radix(&m[3], 16).unwrap_or(0) as f64;
                (r, g, b, 1.0)
            } else if let Some(m) = rgb_regex.captures(&input_str) {
                let r = m[1].parse::<f64>().unwrap_or(0.0);
                let g = m[2].parse::<f64>().unwrap_or(0.0);
                let b = m[3].parse::<f64>().unwrap_or(0.0);
                let mut a = 1.0;
                if let Some(a_match) = m.get(4) {
                    a = a_match.as_str().parse::<f64>().unwrap_or(1.0);
                }
                (r, g, b, a)
            } else if let Some(m) = hsl_regex.captures(&input_str) {
                let h_ = m[1].parse::<f64>().unwrap_or(0.0) / 360.0;
                let s_ = m[2].parse::<f64>().unwrap_or(0.0) / 100.0;
                let l_ = m[3].parse::<f64>().unwrap_or(0.0) / 100.0;
                let rgb_ = Self::_hsl_to_rgb(h_, s_, l_);
                let r = rgb_[0] as f64;
                let g = rgb_[1] as f64;
                let b = rgb_[2] as f64;
                let mut a = 1.0;
                if let Some(a_match) = m.get(4) {
                    a = a_match.as_str().parse::<f64>().unwrap_or(1.0);
                }
                (r, g, b, a)
            } else if let Some(m) = cmyk_regex.captures(&input_str) {
                let c_ = m[1].parse::<f64>().unwrap_or(0.0);
                let m_ = m[2].parse::<f64>().unwrap_or(0.0);
                let y_ = m[3].parse::<f64>().unwrap_or(0.0);
                let k_ = m[4].parse::<f64>().unwrap_or(0.0);

                let r = (255.0 * (1.0 - c_) * (1.0 - k_)).round();
                let g = (255.0 * (1.0 - m_) * (1.0 - k_)).round();
                let b = (255.0 * (1.0 - y_) * (1.0 - k_)).round();
                (r, g, b, 1.0)
            } else {
                return Err(OperationError::InvalidInput(
                    "Unsupported colour code format".to_string(),
                ));
            }
        };

        let hsl_ = Self::_rgb_to_hsl(r, g, b);
        let h = (hsl_[0] * 360.0).round();
        let s = (hsl_[1] * 100.0).round();
        let l = (hsl_[2] * 100.0).round();

        let k = 1.0 - (r / 255.0).max(g / 255.0).max(b / 255.0);
        let mut c = (1.0 - r / 255.0 - k) / (1.0 - k);
        let mut m = (1.0 - g / 255.0 - k) / (1.0 - k);
        let mut y = (1.0 - b / 255.0 - k) / (1.0 - k);

        if k == 1.0 {
            c = 0.0;
            m = 0.0;
            y = 0.0;
        } else {
            if c.is_nan() {
                c = 0.0;
            }
            if m.is_nan() {
                m = 0.0;
            }
            if y.is_nan() {
                y = 0.0;
            }
        }

        let hex_out = format!("#{:02x}{:02x}{:02x}", r as u8, g as u8, b as u8);
        let rgb_out = format!("rgb({}, {}, {})", r, g, b);
        let rgba_out = format!("rgba({}, {}, {}, {})", r, g, b, a);
        let hsl_out = format!("hsl({}, {}%, {}%)", h, s, l);
        let hsla_out = format!("hsla({}, {}%, {}%, {})", h, s, l, a);
        let cmyk_out = format!("cmyk({:.2}, {:.2}, {:.2}, {:.2})", c, m, y, k);

        let output = format!(
            r#"<div id="colorpicker" style="white-space: normal;"></div>
Hex:  {}
RGB:  {}
RGBA: {}
HSL:  {}
HSLA: {}
CMYK: {}
<script>
    $('#colorpicker').colorpicker({{
        format: 'rgba',
        color: '{}',
        container: true,
        inline: true,
        useAlpha: true
    }}).on('colorpickerChange', function(e) {{
        var color = e.color.string('rgba');
        window.app.manager.input.setInput(color);
        window.app.manager.input.inputChange(new Event("keyup"));
    }});
</script>"#,
            hex_out, rgb_out, rgba_out, hsl_out, hsla_out, cmyk_out, rgba_out
        );

        Ok(output.into_bytes())
    }
}

impl ParseColourCode {
    fn _hsl_to_rgb(h: f64, s: f64, l: f64) -> [u8; 3] {
        if s == 0.0 {
            let val = (l * 255.0).round() as u8;
            return [val, val, val];
        }

        let hue2rgb = |p: f64, q: f64, mut t: f64| -> f64 {
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            if t < 1.0 / 6.0 {
                return p + (q - p) * 6.0 * t;
            }
            if t < 1.0 / 2.0 {
                return q;
            }
            if t < 2.0 / 3.0 {
                return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
            }
            p
        };

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        let r = hue2rgb(p, q, h + 1.0 / 3.0);
        let g = hue2rgb(p, q, h);
        let b = hue2rgb(p, q, h - 1.0 / 3.0);

        [
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        ]
    }

    fn _rgb_to_hsl(r: f64, g: f64, b: f64) -> [f64; 3] {
        let r = r / 255.0;
        let g = g / 255.0;
        let b = b / 255.0;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let mut h = 0.0;
        let mut s = 0.0;
        let l = (max + min) / 2.0;

        if max != min {
            let d = max - min;
            s = if l > 0.5 {
                d / (2.0 - max - min)
            } else {
                d / (max + min)
            };
            if max == r {
                h = (g - b) / d + (if g < b { 6.0 } else { 0.0 });
            } else if max == g {
                h = (b - r) / d + 2.0;
            } else if max == b {
                h = (r - g) / d + 4.0;
            }
            h /= 6.0;
        }

        [h, s, l]
    }
}
