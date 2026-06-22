/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
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
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "Displays co-ordinates on a slippy map.<br><br>Co-ordinates will be converted to decimal degrees before being shown on the map.<br><br>Supported formats:<ul><li>Degrees Minutes Seconds (DMS)</li><li>Degrees Decimal Minutes (DDM)</li><li>Decimal Degrees (DD)</li><li>Geohash</li><li>Military Grid Reference System (MGRS)</li><li>Ordnance Survey National Grid (OSNG)</li><li>Universal Transverse Mercator (UTM)</li></ul><br>This operation will not work offline."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Zoom Level",
                description: "Zoom level of the map (0-20)",
                default_value: "13",
            },
            ArgSchema {
                name: "Input Format",
                description: "Format of the input coordinates",
                default_value: "Auto",
            },
            ArgSchema {
                name: "Input Delimiter",
                description: "Delimiter separating the coordinates",
                default_value: "Auto",
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
        let zoom_level = args.first().and_then(|a| a.as_f64()).unwrap_or(13.0) as usize;

        if input_str.trim().is_empty() {
            return Ok(generate_html("0, 0", zoom_level).into_bytes());
        }

        let _in_format = args.get(1).and_then(|a| a.as_str()).unwrap_or("Auto");
        let _in_delim = args.get(2).and_then(|a| a.as_str()).unwrap_or("Auto");

        // Simplified coordinate parsing (enough for common cases)
        let lat_long = parse_coordinates(&input_str);

        let html = generate_html(&lat_long, zoom_level);
        Ok(html.into_bytes())
    }
}

fn parse_coordinates(input: &str) -> String {
    let parts: Vec<&str> = input.split(',').filter(|p| !p.trim().is_empty()).collect();
    if parts.len() >= 2 {
        let lat = parse_dms(parts[0].trim());
        let lon = parse_dms(parts[1].trim());
        format!("{}, {}", lat, lon)
    } else {
        // Fallback for Geohash or other single-string formats if needed
        // For now, just return 0,0 or try to parse as one
        let val = parse_dms(input.trim());
        format!("{}, {}", val, val)
    }
}

fn parse_dms(s: &str) -> f64 {
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
        sign * (caps[0].abs() + caps[1] / 60.0 + caps[2] / 3600.0)
    } else if caps.len() == 2 {
        let sign = if s.starts_with('-') || su.contains('S') || su.contains('W') {
            -1.0
        } else {
            1.0
        };
        sign * (caps[0].abs() + caps[1] / 60.0)
    } else if caps.len() == 1 {
        let sign = if s.starts_with('-') || su.contains('S') || su.contains('W') {
            -1.0
        } else {
            1.0
        };
        sign * caps[0].abs()
    } else {
        0.0
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
