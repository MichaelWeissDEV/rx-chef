/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: JSON Query mit jaq (Jsonata-Ersatz)
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Jsonata Query operation
pub struct Jsonata;

impl Operation for Jsonata {
    fn name(&self) -> &'static str {
        "Jsonata Query"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Query and transform JSON data using jaq. Jsonata is not natively available in Rust,
        so jaq is used as an alternative. Enable with: --features jsonata"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Query",
            description: "The jaq query to run",
            default_value: ".",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Json
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn is_broken(&self) -> bool {
        #[cfg(not(feature = "jsonata"))]
        return true;
        #[cfg(feature = "jsonata")]
        return false;
    }

    fn feature_requirements(&self) -> &'static [&'static str] {
        &["jsonata"]
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        #[cfg(feature = "jsonata")]
        {
            self.run_jaq(input, args)
        }
        #[cfg(not(feature = "jsonata"))]
        {
            let _ = (input, args);
            Err(OperationError::ProcessingError(
                "Jsonata Query requires --features jsonata. Enable with: cargo build --features jsonata"
                    .to_string(),
            ))
        }
    }
}

#[cfg(feature = "jsonata")]
impl Jsonata {
    fn run_jaq(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        use jaq_core::load::{Arena, File, Loader};
        use jaq_core::{data, unwrap_valr, Compiler, Ctx, Vars};
        use jaq_json::{read, Val};

        let query = args.get(0).and_then(|a| a.as_str()).unwrap_or(".");
        let input = read::parse_single(&input)
            .map_err(|error| OperationError::InvalidInput(error.to_string()))?;

        let definitions = jaq_core::defs()
            .chain(jaq_std::defs())
            .chain(jaq_json::defs());
        let functions = jaq_core::funs()
            .chain(jaq_std::funs())
            .chain(jaq_json::funs());
        let loader = Loader::new(definitions);
        let arena = Arena::default();
        let modules = loader
            .load(
                &arena,
                File {
                    code: query,
                    path: (),
                },
            )
            .map_err(|errors| OperationError::InvalidArgument {
                name: "Query".into(),
                reason: format!("{errors:?}"),
            })?;
        let filter = Compiler::default()
            .with_funs(functions)
            .compile(modules)
            .map_err(|errors| OperationError::InvalidArgument {
                name: "Query".into(),
                reason: format!("{errors:?}"),
            })?;
        let context = Ctx::<data::JustLut<Val>>::new(&filter.lut, Vars::new([]));
        let results: Result<Vec<_>, _> = filter.id.run((context, input)).map(unwrap_valr).collect();
        let results =
            results.map_err(|error| OperationError::ProcessingError(error.to_string()))?;

        let output = match results.as_slice() {
            [] => "null".to_string(),
            [value] => value.to_string(),
            values => format!(
                "[{}]",
                values
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        };
        Ok(output.into_bytes())
    }
}
