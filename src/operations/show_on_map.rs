/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Show on map operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Show on map operation
pub struct ShowOnMap;

impl Operation for ShowOnMap {
    fn name(&self) -> &'static str {
        "Show on map"
    }

    fn module(&self) -> &'static str {
        "Maps"
    }

    fn description(&self) -> &'static str {
        "Displays comma-separated coordinates on an OpenStreetMap slippy map. Decimal degrees (DD), degrees/decimal minutes (DDM), and degrees/minutes/seconds (DMS) with N/S/E/W suffixes are converted to decimal degrees. Map tiles require network access in the HTML viewer."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Zoom Level",
                description: "Zoom level of the map (0-20)",
                default_value: "13",
                kind: crate::operation::ArgKind::Integer,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Input Format",
                description: "Format of the input coordinates",
                default_value: "Auto",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Input Delimiter",
                description: "Delimiter separating the coordinates",
                default_value: "Auto",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Html
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let zoom_level = args.first().and_then(|a| a.as_f64()).unwrap_or(13.0);
        if !zoom_level.is_finite() || !(0.0..=20.0).contains(&zoom_level) {
            return Err(OperationError::InvalidArgument {
                name: "Zoom Level".to_string(),
                reason: "must be between 0 and 20".to_string(),
            });
        }
        let zoom_level = zoom_level as usize;

        if input_str.trim().is_empty() {
            return Err(OperationError::InvalidInput(
                "coordinates must not be empty".to_string(),
            ));
        }

        let in_delim = args.get(2).and_then(|a| a.as_str()).unwrap_or("Auto");
        let lat_long = parse_coordinates(&input_str, in_delim)?;

        let html = generate_html(&lat_long, zoom_level);
        Ok(html.into_bytes())
    }
}

fn parse_coordinates(input: &str, delimiter: &str) -> Result<String, OperationError> {
    let delimiter = if delimiter == "Auto" || delimiter.is_empty() {
        if input.contains(',') {
            ","
        } else if input.contains(';') {
            ";"
        } else {
            return Err(OperationError::InvalidInput(
                "could not find a coordinate delimiter; use comma, semicolon, or set Input Delimiter"
                    .to_string(),
            ));
        }
    } else {
        delimiter
    };
    let parts = input
        .split(delimiter)
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(OperationError::InvalidInput(format!(
            "expected exactly latitude and longitude, found {} values",
            parts.len()
        )));
    }
    let lat = parse_dms(parts[0])
        .ok_or_else(|| OperationError::InvalidInput(format!("invalid latitude '{}'", parts[0])))?;
    let lon = parse_dms(parts[1])
        .ok_or_else(|| OperationError::InvalidInput(format!("invalid longitude '{}'", parts[1])))?;
    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return Err(OperationError::InvalidInput(format!(
            "coordinates out of range: {lat}, {lon}"
        )));
    }
    Ok(format!("{lat}, {lon}"))
}

fn parse_dms(s: &str) -> Option<f64> {
    let re = Regex::new(r"[-+]?(?:\d*\.\d+|\d+)").unwrap();
    let caps: Vec<_> = re
        .find_iter(s)
        .filter_map(|m| m.as_str().parse::<f64>().ok())
        .collect();
    let su = s.to_uppercase();
    if caps.len() >= 3 {
        let sign = if s.starts_with('-') || su.contains('S') || su.contains('W') {
            -1.0
        } else {
            1.0
        };
        Some(sign * (caps[0].abs() + caps[1] / 60.0 + caps[2] / 3600.0))
    } else if caps.len() == 2 {
        let sign = if s.starts_with('-') || su.contains('S') || su.contains('W') {
            -1.0
        } else {
            1.0
        };
        Some(sign * (caps[0].abs() + caps[1] / 60.0))
    } else if caps.len() == 1 {
        let sign = if s.starts_with('-') || su.contains('S') || su.contains('W') {
            -1.0
        } else {
            1.0
        };
        Some(sign * caps[0].abs())
    } else {
        None
    }
}

fn generate_html(data: &str, zoom_level: usize) -> String {
    let tile_url = "https://tile.openstreetmap.org/{z}/{x}/{y}.png";
    let tile_attribution =
        "&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors";
    let leaflet_url = "https://unpkg.com/leaflet@1.9.4/dist/leaflet.js";
    let leaflet_css_url = "https://unpkg.com/leaflet@1.9.4/dist/leaflet.css";

    format!(
        r#"<link rel="stylesheet" href="{}" crossorigin=""/>
<style>
    #output-text .cm-content,
    #output-text .cm-line,
    #output-html {{
        padding: 0;
        white-space: normal;
    }}
</style>
<div id="presentedMap" style="width: 100%; height: 100%; min-height: 400px;"></div>
<script type="text/javascript">
var mapscript = document.createElement('script');
document.body.appendChild(mapscript);
mapscript.onload = function() {{
    var presentMap = L.map('presentedMap').setView([{}], {});
    L.tileLayer('{}', {{
        attribution: '{}'
    }}).addTo(presentMap);

    L.marker([{}], {{
        title: '{}'
    }}).addTo(presentMap)
        .bindPopup('{}')
        .openPopup();
}};
mapscript.src = "{}";
</script>"#,
        leaflet_css_url,
        data,
        zoom_level,
        tile_url,
        tile_attribution,
        data,
        data,
        data,
        leaflet_url
    )
}

#[cfg(test)]
mod tests {
    use super::parse_coordinates;

    #[test]
    fn converts_published_dms_math_to_decimal_degrees() {
        assert_eq!(
            parse_coordinates("51° 30' 14.4\" N, 0° 7' 33.6\" W", "Auto").unwrap(),
            "51.504, -0.126"
        );
    }

    #[test]
    fn preserves_ddm_sign_and_fraction() {
        assert_eq!(
            parse_coordinates("33° 51.6' S; 151° 12.6' E", ";").unwrap(),
            "-33.86, 151.21"
        );
    }
}
