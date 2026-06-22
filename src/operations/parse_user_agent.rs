/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse User Agent operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse User Agent operation
pub struct ParseUserAgent;

impl Operation for ParseUserAgent {
    fn name(&self) -> &'static str {
        "Parse User Agent"
    }

    fn module(&self) -> &'static str {
        "UserAgent"
    }

    fn description(&self) -> &'static str {
        "Attempts to identify and categorise information contained in a user-agent string."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let ua_str = String::from_utf8_lossy(&input);

        // Simple regex-based parsing for common browsers and OSs
        // In a real port, we'd use a full library like `woothehee`

        let browser_name = if ua_str.contains("Firefox/") {
            "Firefox"
        } else if ua_str.contains("Chrome/") {
            "Chrome"
        } else if ua_str.contains("Safari/") && !ua_str.contains("Chrome/") {
            "Safari"
        } else if ua_str.contains("MSIE ") || ua_str.contains("Trident/") {
            "Internet Explorer"
        } else if ua_str.contains("Edge/") || ua_str.contains("Edg/") {
            "Edge"
        } else {
            "unknown"
        };

        let browser_ver_re =
            Regex::new(r"(Firefox|Chrome|Safari|Version|MSIE|Trident|Edge|Edg)/([\d\.]+)").unwrap();
        let browser_version = browser_ver_re
            .captures(&ua_str)
            .and_then(|c| c.get(2))
            .map(|m| m.as_str())
            .unwrap_or("unknown");

        let os_name = if ua_str.contains("Windows NT 10.0") {
            "Windows 10"
        } else if ua_str.contains("Windows NT 6.1") {
            "Windows 7"
        } else if ua_str.contains("Android") {
            "Android"
        } else if ua_str.contains("iPhone") || ua_str.contains("iPad") {
            "iOS"
        } else if ua_str.contains("Mac OS X") {
            "Mac OS"
        } else if ua_str.contains("Linux") {
            "Linux"
        } else {
            "unknown"
        };

        let os_ver_re = Regex::new(r"(Windows NT|Android|OS X|OS) ([\d_\.]+)").unwrap();
        let os_version = os_ver_re
            .captures(&ua_str)
            .and_then(|c| c.get(2))
            .map(|m| m.as_str().replace('_', "."))
            .unwrap_or("unknown".to_string());

        let device_type = if ua_str.contains("Mobi") {
            "mobile"
        } else if ua_str.contains("Tablet") || ua_str.contains("iPad") {
            "tablet"
        } else {
            "desktop"
        };

        let engine_name = if ua_str.contains("AppleWebKit") {
            "WebKit"
        } else if ua_str.contains("Gecko/") {
            "Gecko"
        } else if ua_str.contains("Trident/") {
            "Trident"
        } else {
            "unknown"
        };

        let engine_ver_re = Regex::new(r"(AppleWebKit|Gecko|Trident)/([\d\.]+)").unwrap();
        let engine_version = engine_ver_re
            .captures(&ua_str)
            .and_then(|c| c.get(2))
            .map(|m| m.as_str())
            .unwrap_or("unknown");

        let arch =
            if ua_str.contains("x86_64") || ua_str.contains("Win64") || ua_str.contains("WOW64") {
                "amd64"
            } else if ua_str.contains("arm64") || ua_str.contains("aarch64") {
                "arm64"
            } else if ua_str.contains("i686") || ua_str.contains("i386") {
                "ia32"
            } else {
                "unknown"
            };

        let out = format!(
            "Browser
    Name: {}
    Version: {}
Device
    Model: unknown
    Type: {}
    Vendor: unknown
Engine
    Name: {}
    Version: {}
OS
    Name: {}
    Version: {}
CPU
    Architecture: {}",
            browser_name,
            browser_version,
            device_type,
            engine_name,
            engine_version,
            os_name,
            os_version,
            arch
        );

        Ok(out.into_bytes())
    }
}
