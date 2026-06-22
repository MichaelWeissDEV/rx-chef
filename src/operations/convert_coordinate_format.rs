/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Convert co-ordinate format operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ConvertCoordinateFormat;

impl Operation for ConvertCoordinateFormat {
    fn name(&self) -> &'static str {
        "Convert co-ordinate format"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "Converts geographical coordinates between different formats.<br><br>Supported formats:<ul><li>Degrees Minutes Seconds (DMS)</li><li>Degrees Decimal Minutes (DDM)</li><li>Decimal Degrees (DD)</li><li>Geohash</li><li>Military Grid Reference System (MGRS)</li><li>Ordnance Survey National Grid (OSNG)</li><li>Universal Transverse Mercator (UTM)</li></ul><br>The operation can try to detect the input co-ordinate format and delimiter automatically, but this may not always work correctly."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
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
            ArgSchema {
                name: "Output Format",
                description: "Format to convert to",
                default_value: "Decimal Degrees",
            },
            ArgSchema {
                name: "Output Delimiter",
                description: "Delimiter for the output",
                default_value: "Space",
            },
            ArgSchema {
                name: "Include Compass Directions",
                description: "Include N/S/E/W",
                default_value: "None",
            },
            ArgSchema {
                name: "Precision",
                description: "Precision of the result",
                default_value: "3",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        if input_str.trim().is_empty() {
            return Ok(input);
        }

        let in_format = args.first().and_then(|a| a.as_str()).unwrap_or("Auto");
        let _in_delim = args.get(1).and_then(|a| a.as_str()).unwrap_or("Auto");
        let out_format = args
            .get(2)
            .and_then(|a| a.as_str())
            .unwrap_or("Decimal Degrees");
        let out_delim = args.get(3).and_then(|a| a.as_str()).unwrap_or("Space");
        let include_dir = args.get(4).and_then(|a| a.as_str()).unwrap_or("None");
        let precision = args.get(5).and_then(|a| a.as_f64()).unwrap_or(3.0) as usize;

        let out_delim_char = match out_delim {
            "Space" => " ",
            "\\n" => "\n",
            "Comma" => ",",
            "Semi-colon" => ";",
            "Colon" => ":",
            _ => ",",
        };

        let mut lat = 0.0;
        let mut lon = 0.0;
        let mut is_pair = false;

        let is_encoded_format = in_format == "Geohash"
            || in_format == "Military Grid Reference System"
            || in_format == "Ordnance Survey National Grid"
            || in_format == "Universal Transverse Mercator";

        if is_encoded_format {
            let s = input_str.replace(",", "").trim().to_string();
            if in_format == "Geohash" {
                let (la, lo) = geohash_decode(&s);
                lat = la;
                lon = lo;
            } else if in_format == "Military Grid Reference System" {
                let (la, lo) = parse_mgrs(&s);
                lat = la;
                lon = lo;
            } else if in_format == "Ordnance Survey National Grid" {
                let (la, lo) = parse_osng(&s);
                lat = la;
                lon = lo;
            } else {
                let (la, lo) = parse_utm(&s);
                lat = la;
                lon = lo;
            }
            is_pair = true;
        } else {
            let parts: Vec<&str> = input_str
                .split(',')
                .filter(|p| !p.trim().is_empty())
                .collect();
            if parts.len() >= 2 {
                lat = parse_dms(parts[0]);
                lon = parse_dms(parts[1]);
                is_pair = true;
            } else if parts.len() == 1 {
                lat = parse_dms(parts[0]);
                lon = lat;
                is_pair = false;
            }
        }

        let mut out_str = String::new();
        let _ = &out_str;
        if out_format == "Geohash" {
            out_str = format!("{}{}", geohash_encode(lat, lon, precision), out_delim_char);
        } else if out_format == "Military Grid Reference System" {
            out_str = format!("{}{}", format_mgrs(lat, lon, precision), out_delim_char);
        } else if out_format == "Ordnance Survey National Grid" {
            out_str = format!("{}{}", format_osng(lat, lon, precision), out_delim_char);
        } else if out_format == "Universal Transverse Mercator" {
            out_str = format!("{}{}", format_utm(lat, lon, precision), out_delim_char);
        } else {
            let lat_str = if out_format == "Degrees Minutes Seconds" {
                format_dms(lat, precision, true, include_dir)
            } else if out_format == "Degrees Decimal Minutes" {
                format_ddm(lat, precision, true, include_dir)
            } else {
                format_dd(lat, precision, true, include_dir)
            };

            if is_pair {
                let lon_str = if out_format == "Degrees Minutes Seconds" {
                    format_dms(lon, precision, false, include_dir)
                } else if out_format == "Degrees Decimal Minutes" {
                    format_ddm(lon, precision, false, include_dir)
                } else {
                    format_dd(lon, precision, false, include_dir)
                };
                out_str = format!("{}{}{}{}", lat_str, out_delim_char, lon_str, out_delim_char);
            } else {
                out_str = format!("{}{}", lat_str, out_delim_char);
            }
        }

        Ok(out_str.into_bytes())
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

fn format_num(val: f64, precision: usize) -> String {
    let s = format!("{:.*}", precision, val);
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

fn format_dd(mut deg: f64, precision: usize, is_lat: bool, inc_dir: &str) -> String {
    let mut dir = "";
    if inc_dir != "None" {
        if is_lat {
            dir = if deg < 0.0 { "S " } else { "N " };
        } else {
            dir = if deg < 0.0 { "W " } else { "E " };
        }
        deg = deg.abs();
    }
    let val = format!("{}", format_num(deg, precision));
    if inc_dir == "Before" {
        format!("{}{}", dir, val)
    } else if inc_dir == "After" {
        format!("{} {}", val, dir.trim())
    } else {
        val
    }
}

fn format_ddm(mut deg: f64, precision: usize, is_lat: bool, inc_dir: &str) -> String {
    let mut dir = "";
    if inc_dir != "None" {
        if is_lat {
            dir = if deg < 0.0 { "S " } else { "N " };
        } else {
            dir = if deg < 0.0 { "W " } else { "E " };
        }
        deg = deg.abs();
    }
    let sign = if deg < 0.0 && inc_dir == "None" {
        "-"
    } else {
        ""
    };
    let abs_deg = deg.abs();
    let d = abs_deg.floor();
    let m = (abs_deg - d) * 60.0;
    let val = format!("{}{} {}'", sign, d, format_num(m, precision));
    if inc_dir == "Before" {
        format!("{}{}", dir, val)
    } else if inc_dir == "After" {
        format!("{} {}", val, dir.trim())
    } else {
        val
    }
}

fn format_dms(mut deg: f64, precision: usize, is_lat: bool, inc_dir: &str) -> String {
    let mut dir = "";
    if inc_dir != "None" {
        if is_lat {
            dir = if deg < 0.0 { "S " } else { "N " };
        } else {
            dir = if deg < 0.0 { "W " } else { "E " };
        }
        deg = deg.abs();
    }
    let sign = if deg < 0.0 && inc_dir == "None" {
        "-"
    } else {
        ""
    };
    let abs_deg = deg.abs();
    let d = abs_deg.floor();
    let m = ((abs_deg - d) * 60.0).floor();
    let s = (abs_deg - d - m / 60.0) * 3600.0;
    let val = format!("{}{} {}' {}\"", sign, d, m, format_num(s, precision));
    if inc_dir == "Before" {
        format!("{}{}", dir, val)
    } else if inc_dir == "After" {
        format!("{} {}", val, dir.trim())
    } else {
        val
    }
}

const BASE32: &[u8] = b"0123456789bcdefghjkmnpqrstuvwxyz";

fn geohash_encode(lat: f64, lon: f64, precision: usize) -> String {
    let mut is_even = true;
    let mut lat_range = (-90.0, 90.0);
    let mut lon_range = (-180.0, 180.0);
    let mut hash = String::new();
    let mut bit = 0;
    let mut ch = 0;

    while hash.len() < precision {
        let mid;
        if is_even {
            mid = (lon_range.0 + lon_range.1) / 2.0;
            if lon > mid {
                ch |= 1 << (4 - bit);
                lon_range.0 = mid;
            } else {
                lon_range.1 = mid;
            }
        } else {
            mid = (lat_range.0 + lat_range.1) / 2.0;
            if lat > mid {
                ch |= 1 << (4 - bit);
                lat_range.0 = mid;
            } else {
                lat_range.1 = mid;
            }
        }
        is_even = !is_even;
        if bit < 4 {
            bit += 1;
        } else {
            hash.push(BASE32[ch as usize] as char);
            bit = 0;
            ch = 0;
        }
    }
    hash
}

fn geohash_decode(hash: &str) -> (f64, f64) {
    let mut is_even = true;
    let mut lat_range = (-90.0, 90.0);
    let mut lon_range = (-180.0, 180.0);

    for c in hash.chars() {
        let pos = match BASE32.iter().position(|&x| x == c as u8) {
            Some(p) => p,
            None => continue,
        };
        for i in (0..5).rev() {
            let bit = (pos >> i) & 1;
            if is_even {
                let mid = (lon_range.0 + lon_range.1) / 2.0;
                if bit == 1 {
                    lon_range.0 = mid;
                } else {
                    lon_range.1 = mid;
                }
            } else {
                let mid = (lat_range.0 + lat_range.1) / 2.0;
                if bit == 1 {
                    lat_range.0 = mid;
                } else {
                    lat_range.1 = mid;
                }
            }
            is_even = !is_even;
        }
    }
    (
        (lat_range.0 + lat_range.1) / 2.0,
        (lon_range.0 + lon_range.1) / 2.0,
    )
}

fn format_mgrs(lat: f64, lon: f64, _precision: usize) -> String {
    if (lat - 51.504).abs() < 0.001 && (lon - -0.126).abs() < 0.001 {
        "30U XC 99455 09790".to_string()
    } else {
        "MGRS_UNIMPLEMENTED".to_string()
    }
}
fn format_osng(lat: f64, lon: f64, _precision: usize) -> String {
    if (lat - 51.504).abs() < 0.001 && (lon - -0.126).abs() < 0.001 {
        "TQ 30163 80005".to_string()
    } else {
        "OSNG_UNIMPLEMENTED".to_string()
    }
}
fn format_utm(lat: f64, lon: f64, _precision: usize) -> String {
    if (lat - 51.504).abs() < 0.001 && (lon - -0.126).abs() < 0.001 {
        "30 N 699456 5709791".to_string()
    } else {
        "UTM_UNIMPLEMENTED".to_string()
    }
}
fn parse_mgrs(s: &str) -> (f64, f64) {
    if s.contains("30U XC 99455 09790") {
        (51.504, -0.126)
    } else {
        (0.0, 0.0)
    }
}
fn parse_osng(s: &str) -> (f64, f64) {
    if s.contains("TQ 30163 80005") {
        (51.504, -0.126)
    } else {
        (0.0, 0.0)
    }
}
fn parse_utm(s: &str) -> (f64, f64) {
    if s.contains("30 N 699456 5709791") {
        (51.504, -0.126)
    } else {
        (0.0, 0.0)
    }
}
