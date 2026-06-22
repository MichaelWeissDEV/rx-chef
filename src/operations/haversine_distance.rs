/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Haversine distance operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Haversine Distance operation
///
/// Returns the distance between two pairs of GPS coordinates in metres.
/// Input format: lat1, lng1, lat2, lng2
pub struct HaversineDistance;

impl Operation for HaversineDistance {
    fn name(&self) -> &'static str {
        "Haversine distance"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Returns the distance between two pairs of GPS latitude and longitude co-ordinates in metres. Input format: lat1, lng1, lat2, lng2"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Number
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let s = String::from_utf8_lossy(&input);
        let s = s.trim();

        // Parse four decimal numbers separated by commas
        let parts: Vec<&str> = s.splitn(4, ',').collect();
        if parts.len() != 4 {
            return Err(OperationError::InvalidInput(
                "Input must be in the format lat1, lng1, lat2, lng2".to_string(),
            ));
        }

        let lat1: f64 = parts[0]
            .trim()
            .parse()
            .map_err(|_| OperationError::InvalidInput("Invalid lat1".to_string()))?;
        let lng1: f64 = parts[1]
            .trim()
            .parse()
            .map_err(|_| OperationError::InvalidInput("Invalid lng1".to_string()))?;
        let lat2: f64 = parts[2]
            .trim()
            .parse()
            .map_err(|_| OperationError::InvalidInput("Invalid lat2".to_string()))?;
        let lng2: f64 = parts[3]
            .trim()
            .parse()
            .map_err(|_| OperationError::InvalidInput("Invalid lng2".to_string()))?;

        let to_rad = std::f64::consts::PI / 180.0;
        let d_lat = (lat2 - lat1) * to_rad;
        let d_lng = (lng2 - lng1) * to_rad;
        let a = (d_lat / 2.0).sin().powi(2)
            + (lat1 * to_rad).cos() * (lat2 * to_rad).cos() * (d_lng / 2.0).sin().powi(2);
        let metres = 6_371_000.0 * 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        // Format: if result is exactly 0, output "0"; otherwise full precision
        let result_str = if metres == 0.0 {
            "0".to_string()
        } else {
            format!("{}", metres)
        };
        Ok(result_str.into_bytes())
    }
}
