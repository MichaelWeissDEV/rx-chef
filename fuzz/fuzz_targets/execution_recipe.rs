#![no_main]

use libfuzzer_sys::fuzz_target;
use rxchef::execution::{
    execute, ExecutionOptions, ExecutionRequest, RecipeStep, VariableContext,
};

fuzz_target!(|data: &[u8]| {
    let Ok(steps) = serde_json::from_slice::<Vec<RecipeStep>>(data) else {
        return;
    };
    if steps.len() > 32 {
        return;
    }
    let _ = execute(ExecutionRequest {
        input: data[..data.len().min(4096)].to_vec(),
        recipe: steps.into(),
        variables: VariableContext::default(),
        options: ExecutionOptions {
            max_steps: 128,
            max_output_bytes: Some(64 * 1024),
            ..ExecutionOptions::default()
        },
    });
});
