//! Shared operation and recipe execution engine.
//!
//! Every frontend passes byte input and a recipe to this module. The engine is
//! deliberately independent of terminals, stores, and transport protocols.

use std::{collections::HashMap, time::Duration, time::Instant};

use serde::{Deserialize, Serialize};

use crate::{operation::InputRequirement, runtime};

/// Default maximum number of recipe steps executed, including loop iterations
/// and steps executed inside flow-control branches.
pub const DEFAULT_MAX_STEPS: usize = 1_000_000;

/// One operation and its ordered argument values in a recipe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeStep {
    /// Operation name or normalized identifier.
    #[serde(alias = "operation")]
    pub op: String,
    /// Ordered raw arguments. Parsing is delegated to the shared registry
    /// runtime after variable and register expansion.
    #[serde(default)]
    pub args: Vec<String>,
}

/// A recipe executed by [`execute`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    /// Ordered recipe steps.
    pub steps: Vec<RecipeStep>,
}

impl From<Vec<RecipeStep>> for Recipe {
    fn from(steps: Vec<RecipeStep>) -> Self {
        Self { steps }
    }
}

/// Values available for `$NAME` and `${NAME}` expansion in operation
/// arguments. Names are matched case-insensitively.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VariableContext {
    values: HashMap<String, String>,
}

impl VariableContext {
    /// Create a context from name/value pairs.
    pub fn new(values: impl IntoIterator<Item = (String, String)>) -> Self {
        Self {
            values: values
                .into_iter()
                .map(|(name, value)| (name.to_uppercase(), value))
                .collect(),
        }
    }

    /// Insert or replace one value.
    pub fn insert(&mut self, name: impl AsRef<str>, value: impl Into<String>) {
        self.values
            .insert(name.as_ref().to_uppercase(), value.into());
    }

    fn expand(&self, input: &str) -> String {
        let characters: Vec<char> = input.chars().collect();
        let mut output = String::with_capacity(input.len());
        let mut index = 0;
        while index < characters.len() {
            if characters[index] == '$' && index + 1 < characters.len() {
                if characters[index + 1] == '{' {
                    if let Some(end) = characters[index + 2..]
                        .iter()
                        .position(|character| *character == '}')
                    {
                        let name: String = characters[index + 2..index + 2 + end].iter().collect();
                        if let Some(value) = self.values.get(&name.to_uppercase()) {
                            output.push_str(value);
                        } else {
                            output.push_str("${");
                            output.push_str(&name);
                            output.push('}');
                        }
                        index += end + 3;
                        continue;
                    }
                } else if characters[index + 1].is_alphabetic() || characters[index + 1] == '_' {
                    let start = index + 1;
                    let mut end = start;
                    while end < characters.len()
                        && (characters[end].is_alphanumeric() || characters[end] == '_')
                    {
                        end += 1;
                    }
                    let name: String = characters[start..end].iter().collect();
                    if let Some(value) = self.values.get(&name.to_uppercase()) {
                        output.push_str(value);
                    } else {
                        output.push('$');
                        output.push_str(&name);
                    }
                    index = end;
                    continue;
                }
            }
            output.push(characters[index]);
            index += 1;
        }
        output
    }
}

/// Resource and diagnostic options for one execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionOptions {
    /// Record size and timing metadata for executed steps.
    pub trace: bool,
    /// Maximum number of step executions before aborting.
    pub max_steps: usize,
    /// Optional maximum size of any intermediate or final output.
    pub max_output_bytes: Option<usize>,
}

impl Default for ExecutionOptions {
    fn default() -> Self {
        Self {
            trace: false,
            max_steps: DEFAULT_MAX_STEPS,
            max_output_bytes: None,
        }
    }
}

/// Complete input to the shared engine.
#[derive(Debug, Clone)]
pub struct ExecutionRequest {
    /// Exact input bytes.
    pub input: Vec<u8>,
    /// Distinguishes an absent input source from an explicitly supplied empty
    /// byte stream.
    pub input_supplied: bool,
    /// Recipe to execute.
    pub recipe: Recipe,
    /// Values expanded in arguments before operation dispatch.
    pub variables: VariableContext,
    /// Trace and resource-limit settings.
    pub options: ExecutionOptions,
}

/// Result of a successful execution.
#[derive(Debug, Clone)]
pub struct ExecutionOutcome {
    /// Exact final bytes.
    pub output: Vec<u8>,
    /// Metadata-only execution trace. Payloads and argument values are never
    /// retained here.
    pub trace: Vec<TraceEntry>,
}

/// Result status recorded for a trace entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceStatus {
    /// A normal operation produced output.
    Success,
    /// A flow-control step was processed by the engine.
    ControlFlow,
}

/// Non-sensitive metadata for one executed recipe step.
#[derive(Debug, Clone)]
pub struct TraceEntry {
    /// Zero-based recipe index.
    pub step_index: usize,
    /// Operation name as supplied by the recipe.
    pub operation: String,
    /// Input byte count.
    pub input_bytes: usize,
    /// Output byte count.
    pub output_bytes: usize,
    /// Wall-clock duration of this engine action.
    pub elapsed: Duration,
    /// Whether this was an operation or a flow-control action.
    pub status: TraceStatus,
}

/// Structured failures returned by the execution engine.
#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    /// Recipe structure or flow-control arguments are invalid.
    #[error("invalid recipe: {0}")]
    InvalidRecipe(String),
    /// An operation could not be resolved or executed.
    #[error("step {step_index} ({operation}): {message}")]
    Step {
        /// One-based step number for user-facing diagnostics.
        step_index: usize,
        /// Operation name.
        operation: String,
        /// Registry, argument, or operation error.
        message: String,
    },
    /// A typed registry, argument, availability, or operation failure.
    #[error("step {step_index} ({operation}): {source}")]
    RuntimeStep {
        step_index: usize,
        operation: String,
        #[source]
        source: runtime::RuntimeError,
    },
    /// The configured execution fuel was consumed.
    #[error("execution step limit exceeded ({limit})")]
    StepLimitExceeded {
        /// Configured maximum.
        limit: usize,
    },
    /// An intermediate result exceeded the configured byte limit.
    #[error("step {step_index} output is {actual} bytes, exceeding the {limit}-byte limit")]
    OutputLimitExceeded {
        /// One-based recipe step number.
        step_index: usize,
        /// Configured maximum.
        limit: usize,
        /// Observed output size.
        actual: usize,
    },
}

fn flow_name(name: &str) -> String {
    name.chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn validate_recipe(
    recipe: &[RecipeStep],
    variables: &VariableContext,
) -> Result<(), ExecutionError> {
    let mut labels = HashMap::new();
    let mut branches = Vec::new();
    for (index, step) in recipe.iter().enumerate() {
        let kind = flow_name(&step.op);
        match kind.as_str() {
            "fork" | "subsection" => branches.push((index, step.op.as_str())),
            "merge" => {
                if branches.pop().is_none() {
                    return Err(ExecutionError::InvalidRecipe(format!(
                        "step {} (Merge): no matching Fork or Subsection",
                        index + 1
                    )));
                }
            }
            "label" => {
                let name = variables.expand(argument(step, 0, ""));
                if name.is_empty() {
                    return Err(ExecutionError::InvalidRecipe(format!(
                        "step {} (Label): label name is empty",
                        index + 1
                    )));
                }
                if labels.insert(name.clone(), index).is_some() {
                    return Err(ExecutionError::InvalidRecipe(format!(
                        "step {} (Label): duplicate label '{name}'",
                        index + 1
                    )));
                }
            }
            _ => {}
        }
        let args = step
            .args
            .iter()
            .map(|value| variables.expand(value))
            .collect::<Vec<_>>();
        if !args.iter().any(|value| value.contains("$R")) {
            runtime::validate_operation_args(&step.op, &args).map_err(|source| {
                ExecutionError::RuntimeStep {
                    step_index: index + 1,
                    operation: step.op.clone(),
                    source,
                }
            })?;
        } else {
            runtime::operation_info(&step.op).map_err(|message| ExecutionError::Step {
                step_index: index + 1,
                operation: step.op.clone(),
                message,
            })?;
        }
    }
    if let Some((index, operation)) = branches.pop() {
        return Err(ExecutionError::InvalidRecipe(format!(
            "step {} ({operation}): missing matching Merge",
            index + 1
        )));
    }
    for (index, step) in recipe.iter().enumerate() {
        let kind = flow_name(&step.op);
        let label_index = match kind.as_str() {
            "jump" => Some(0),
            "conditionaljump" => Some(2),
            _ => None,
        };
        if let Some(label_index) = label_index {
            let label = variables.expand(argument(step, label_index, ""));
            if !label.contains("$R") && !labels.contains_key(&label) {
                return Err(ExecutionError::InvalidRecipe(format!(
                    "step {} ({}): unknown label '{label}'",
                    index + 1,
                    step.op
                )));
            }
        }
    }
    Ok(())
}

fn argument<'a>(step: &'a RecipeStep, index: usize, default: &'a str) -> &'a str {
    step.args.get(index).map(String::as_str).unwrap_or(default)
}

fn bool_argument(step: &RecipeStep, index: usize, default: bool) -> Result<bool, String> {
    let raw = step
        .args
        .get(index)
        .map(String::as_str)
        .unwrap_or(if default { "true" } else { "false" });
    let raw = raw.strip_prefix("bool:").unwrap_or(raw);
    match raw.to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Ok(true),
        "false" | "0" | "no" | "off" => Ok(false),
        _ => Err(format!("invalid boolean argument '{raw}'")),
    }
}

fn usize_argument(step: &RecipeStep, index: usize, default: usize) -> Result<usize, String> {
    let raw = step.args.get(index).map(String::as_str).unwrap_or("");
    if raw.is_empty() {
        return Ok(default);
    }
    raw.strip_prefix("num:")
        .unwrap_or(raw)
        .parse::<usize>()
        .map_err(|error| format!("invalid integer argument '{raw}': {error}"))
}

fn unescape_delimiter(value: &str) -> Vec<u8> {
    let mut output = Vec::with_capacity(value.len());
    let mut characters = value.chars();
    while let Some(character) = characters.next() {
        if character != '\\' {
            let mut encoded = [0; 4];
            output.extend_from_slice(character.encode_utf8(&mut encoded).as_bytes());
            continue;
        }
        match characters.next() {
            Some('n') => output.push(b'\n'),
            Some('r') => output.push(b'\r'),
            Some('t') => output.push(b'\t'),
            Some('0') => output.push(0),
            Some('\\') => output.push(b'\\'),
            Some(other) => {
                output.push(b'\\');
                let mut encoded = [0; 4];
                output.extend_from_slice(other.encode_utf8(&mut encoded).as_bytes());
            }
            None => output.push(b'\\'),
        }
    }
    output
}

fn split_bytes<'a>(input: &'a [u8], delimiter: &[u8]) -> Result<Vec<&'a [u8]>, String> {
    if delimiter.is_empty() {
        return Err("Fork split delimiter must not be empty".to_string());
    }
    let mut parts = Vec::new();
    let mut start = 0;
    while let Some(offset) = input[start..]
        .windows(delimiter.len())
        .position(|window| window == delimiter)
    {
        let end = start + offset;
        parts.push(&input[start..end]);
        start = end + delimiter.len();
    }
    parts.push(&input[start..]);
    Ok(parts)
}

fn expand_registers(value: &str, registers: &[String]) -> String {
    let matcher = regex::Regex::new(r"\$R(\d+)").expect("static register regex");
    matcher
        .replace_all(value, |captures: &regex::Captures<'_>| {
            captures[1]
                .parse::<usize>()
                .ok()
                .and_then(|index| registers.get(index))
                .cloned()
                .unwrap_or_else(|| captures[0].to_string())
        })
        .into_owned()
}

struct Engine<'a> {
    recipe: &'a [RecipeStep],
    variables: &'a VariableContext,
    options: &'a ExecutionOptions,
    labels: HashMap<String, usize>,
    executions: usize,
    input_supplied: bool,
    trace: Vec<TraceEntry>,
}

impl<'a> Engine<'a> {
    fn new(
        recipe: &'a [RecipeStep],
        variables: &'a VariableContext,
        options: &'a ExecutionOptions,
        input_supplied: bool,
    ) -> Result<Self, ExecutionError> {
        let mut labels = HashMap::new();
        for (index, step) in recipe.iter().enumerate() {
            if flow_name(&step.op) == "label" {
                let name = variables.expand(argument(step, 0, ""));
                if name.is_empty() {
                    return Err(ExecutionError::InvalidRecipe(format!(
                        "step {} (Label): label name is empty",
                        index + 1
                    )));
                }
                if labels.insert(name.clone(), index).is_some() {
                    return Err(ExecutionError::InvalidRecipe(format!(
                        "step {} (Label): duplicate label '{name}'",
                        index + 1
                    )));
                }
            }
        }
        Ok(Self {
            recipe,
            variables,
            options,
            labels,
            executions: 0,
            input_supplied,
            trace: Vec::new(),
        })
    }

    fn matching_merge(&self, start: usize, end: usize) -> Result<usize, ExecutionError> {
        let mut depth = 0usize;
        for index in start + 1..end {
            match flow_name(&self.recipe[index].op).as_str() {
                "fork" | "subsection" => depth += 1,
                "merge" if depth == 0 => return Ok(index),
                "merge" => depth -= 1,
                _ => {}
            }
        }
        Err(ExecutionError::InvalidRecipe(format!(
            "step {} ({}): missing matching Merge",
            start + 1,
            self.recipe[start].op
        )))
    }

    fn step_error(&self, index: usize, message: impl Into<String>) -> ExecutionError {
        ExecutionError::Step {
            step_index: index + 1,
            operation: self.recipe[index].op.clone(),
            message: message.into(),
        }
    }

    fn regex(
        &self,
        index: usize,
        pattern: &str,
        case_insensitive: bool,
        multiline: bool,
        dot_matches_new_line: bool,
    ) -> Result<regex::bytes::Regex, ExecutionError> {
        regex::bytes::RegexBuilder::new(pattern)
            .case_insensitive(case_insensitive)
            .multi_line(multiline)
            .dot_matches_new_line(dot_matches_new_line)
            .unicode(false)
            .build()
            .map_err(|error| self.step_error(index, format!("invalid regular expression: {error}")))
    }

    fn check_output(&self, step_index: usize, output_len: usize) -> Result<(), ExecutionError> {
        if let Some(limit) = self.options.max_output_bytes {
            if output_len > limit {
                return Err(ExecutionError::OutputLimitExceeded {
                    step_index: step_index + 1,
                    limit,
                    actual: output_len,
                });
            }
        }
        Ok(())
    }

    fn record_trace(
        &mut self,
        index: usize,
        input_bytes: usize,
        output_bytes: usize,
        started: Instant,
        status: TraceStatus,
    ) {
        if self.options.trace {
            self.trace.push(TraceEntry {
                step_index: index,
                operation: self.recipe[index].op.clone(),
                input_bytes,
                output_bytes,
                elapsed: started.elapsed(),
                status,
            });
        }
    }

    fn run_range(
        &mut self,
        start: usize,
        end: usize,
        mut current: Vec<u8>,
        registers: &mut Vec<String>,
    ) -> Result<Vec<u8>, ExecutionError> {
        let mut pc = start;
        let mut jump_counts: HashMap<usize, usize> = HashMap::new();
        while pc < end {
            self.executions += 1;
            if self.executions > self.options.max_steps {
                return Err(ExecutionError::StepLimitExceeded {
                    limit: self.options.max_steps,
                });
            }

            let index = pc;
            let started = Instant::now();
            let input_bytes = current.len();
            let step = &self.recipe[index];
            let kind = flow_name(&step.op);
            let result = match kind.as_str() {
                "fork" => {
                    let merge = self.matching_merge(index, end)?;
                    let split =
                        unescape_delimiter(&self.variables.expand(argument(step, 0, "\\n")));
                    let join = unescape_delimiter(&self.variables.expand(argument(step, 1, "\\n")));
                    let ignore_errors = bool_argument(step, 2, false)
                        .map_err(|error| self.step_error(index, error))?;
                    let branches = split_bytes(&current, &split)
                        .map_err(|error| self.step_error(index, error))?;
                    let mut joined = Vec::new();
                    for (branch_index, branch) in branches.iter().enumerate() {
                        if branch_index > 0 {
                            joined.extend_from_slice(&join);
                        }
                        let mut branch_registers = registers.clone();
                        match self.run_range(
                            index + 1,
                            merge,
                            branch.to_vec(),
                            &mut branch_registers,
                        ) {
                            Ok(output) => joined.extend_from_slice(&output),
                            Err(_) if ignore_errors => joined.extend_from_slice(branch),
                            Err(error) => return Err(error),
                        }
                    }
                    pc = merge + 1;
                    Some(joined)
                }
                "subsection" => {
                    let merge = self.matching_merge(index, end)?;
                    let pattern =
                        expand_registers(&self.variables.expand(argument(step, 0, "")), registers);
                    let case_sensitive = bool_argument(step, 1, true)
                        .map_err(|error| self.step_error(index, error))?;
                    let global = bool_argument(step, 2, true)
                        .map_err(|error| self.step_error(index, error))?;
                    let ignore_errors = bool_argument(step, 3, false)
                        .map_err(|error| self.step_error(index, error))?;
                    let matcher = self.regex(index, &pattern, !case_sensitive, true, true)?;
                    let matches: Vec<_> = matcher.find_iter(&current).collect();
                    let mut output = Vec::with_capacity(current.len());
                    let mut offset = 0;
                    for (match_index, found) in matches.iter().enumerate() {
                        if !global && match_index > 0 {
                            break;
                        }
                        output.extend_from_slice(&current[offset..found.start()]);
                        let original = &current[found.start()..found.end()];
                        let mut section_registers = registers.clone();
                        match self.run_range(
                            index + 1,
                            merge,
                            original.to_vec(),
                            &mut section_registers,
                        ) {
                            Ok(section) => output.extend_from_slice(&section),
                            Err(_) if ignore_errors => output.extend_from_slice(original),
                            Err(error) => return Err(error),
                        }
                        offset = found.end();
                    }
                    output.extend_from_slice(&current[offset..]);
                    pc = merge + 1;
                    Some(output)
                }
                "register" => {
                    let pattern = expand_registers(
                        &self.variables.expand(argument(step, 0, "([\\s\\S]*)")),
                        registers,
                    );
                    let case_insensitive = bool_argument(step, 1, true)
                        .map_err(|error| self.step_error(index, error))?;
                    let multiline = bool_argument(step, 2, false)
                        .map_err(|error| self.step_error(index, error))?;
                    let dot_all = bool_argument(step, 3, false)
                        .map_err(|error| self.step_error(index, error))?;
                    let matcher =
                        self.regex(index, &pattern, case_insensitive, multiline, dot_all)?;
                    registers.clear();
                    if let Some(captures) = matcher.captures(&current) {
                        let first = usize::from(captures.len() > 1);
                        registers.extend((first..captures.len()).map(|capture_index| {
                            captures
                                .get(capture_index)
                                .map(|value| String::from_utf8_lossy(value.as_bytes()).into_owned())
                                .unwrap_or_default()
                        }));
                    }
                    pc += 1;
                    None
                }
                "jump" | "conditionaljump" => {
                    let (should_jump, label_index, maximum_index) = if kind == "jump" {
                        (true, 0, 1)
                    } else {
                        let pattern = expand_registers(
                            &self.variables.expand(argument(step, 0, "")),
                            registers,
                        );
                        let invert = bool_argument(step, 1, false)
                            .map_err(|error| self.step_error(index, error))?;
                        let matches = self
                            .regex(index, &pattern, false, false, false)?
                            .is_match(&current);
                        (matches != invert, 2, 3)
                    };
                    if !should_jump {
                        pc += 1;
                    } else {
                        let label = expand_registers(
                            &self.variables.expand(argument(step, label_index, "")),
                            registers,
                        );
                        let target = *self.labels.get(&label).ok_or_else(|| {
                            self.step_error(index, format!("unknown label '{label}'"))
                        })?;
                        if target < start || target >= end {
                            return Err(self.step_error(
                                index,
                                format!("label '{label}' is outside the current branch"),
                            ));
                        }
                        if target <= index {
                            let maximum = usize_argument(step, maximum_index, 10)
                                .map_err(|error| self.step_error(index, error))?;
                            let count = jump_counts.entry(index).or_default();
                            if *count >= maximum {
                                pc += 1;
                            } else {
                                *count += 1;
                                pc = target;
                            }
                        } else {
                            pc = target;
                        }
                    }
                    None
                }
                "label" | "merge" => {
                    pc += 1;
                    None
                }
                _ => {
                    let args = step
                        .args
                        .iter()
                        .map(|value| expand_registers(&self.variables.expand(value), registers))
                        .collect::<Vec<_>>();
                    let operation_input = std::mem::take(&mut current);
                    let info = runtime::operation_info(&step.op)
                        .map_err(|error| self.step_error(index, error))?;
                    if info.input_requirement == InputRequirement::Required && !self.input_supplied
                    {
                        return Err(self.step_error(index, "input source is required"));
                    }
                    let output = runtime::run_operation(&step.op, operation_input, &args).map_err(
                        |source| ExecutionError::RuntimeStep {
                            step_index: index + 1,
                            operation: step.op.clone(),
                            source,
                        },
                    )?;
                    self.input_supplied = true;
                    pc += 1;
                    Some(output)
                }
            };
            if let Some(output) = result {
                self.check_output(index, output.len())?;
                current = output;
            }
            self.record_trace(
                index,
                input_bytes,
                current.len(),
                started,
                if matches!(
                    kind.as_str(),
                    "fork"
                        | "subsection"
                        | "register"
                        | "jump"
                        | "conditionaljump"
                        | "label"
                        | "merge"
                ) {
                    TraceStatus::ControlFlow
                } else {
                    TraceStatus::Success
                },
            );
        }
        Ok(current)
    }
}

/// Execute an operation recipe through the shared engine.
///
/// # Errors
///
/// Returns [`ExecutionError`] when recipe validation, resource limits,
/// operation lookup, argument parsing, or operation execution fails.
pub fn execute(request: ExecutionRequest) -> Result<ExecutionOutcome, ExecutionError> {
    validate_recipe(&request.recipe.steps, &request.variables)?;
    if request.options.max_steps == 0 && !request.recipe.steps.is_empty() {
        return Err(ExecutionError::StepLimitExceeded { limit: 0 });
    }
    if let Some(limit) = request.options.max_output_bytes {
        if request.input.len() > limit && request.recipe.steps.is_empty() {
            return Err(ExecutionError::OutputLimitExceeded {
                step_index: 0,
                limit,
                actual: request.input.len(),
            });
        }
    }
    let mut engine = Engine::new(
        &request.recipe.steps,
        &request.variables,
        &request.options,
        request.input_supplied,
    )?;
    let output = engine.run_range(
        0,
        request.recipe.steps.len(),
        request.input,
        &mut Vec::new(),
    )?;
    Ok(ExecutionOutcome {
        output,
        trace: engine.trace,
    })
}

/// Execute a single operation through the same engine used for recipes.
///
/// # Errors
///
/// Returns [`ExecutionError`] for operation, argument, or limit failures.
pub fn run(
    operation: impl Into<String>,
    input: Vec<u8>,
    args: Vec<String>,
) -> Result<ExecutionOutcome, ExecutionError> {
    execute(ExecutionRequest {
        input,
        input_supplied: true,
        recipe: Recipe::from(vec![RecipeStep {
            op: operation.into(),
            args,
        }]),
        variables: VariableContext::default(),
        options: ExecutionOptions::default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_and_flow_recipes_use_the_same_engine() {
        let linear = execute(ExecutionRequest {
            input: b"Hello".to_vec(),
            input_supplied: true,
            recipe: vec![
                RecipeStep {
                    op: "To Upper case".into(),
                    args: vec![],
                },
                RecipeStep {
                    op: "To Base64".into(),
                    args: vec![],
                },
            ]
            .into(),
            variables: VariableContext::default(),
            options: ExecutionOptions::default(),
        })
        .unwrap();
        assert_eq!(linear.output, b"SEVMTE8=");

        let flow = execute(ExecutionRequest {
            input: b"one\ntwo".to_vec(),
            input_supplied: true,
            recipe: vec![
                RecipeStep {
                    op: "Fork".into(),
                    args: vec!["\\n".into(), "|".into(), "false".into()],
                },
                RecipeStep {
                    op: "To Upper case".into(),
                    args: vec![],
                },
                RecipeStep {
                    op: "Merge".into(),
                    args: vec![],
                },
            ]
            .into(),
            variables: VariableContext::default(),
            options: ExecutionOptions::default(),
        })
        .unwrap();
        assert_eq!(flow.output, b"ONE|TWO");
    }

    #[test]
    fn step_and_output_limits_fail_structurally() {
        let request = ExecutionRequest {
            input: b"a".to_vec(),
            input_supplied: true,
            recipe: vec![RecipeStep {
                op: "To Base64".into(),
                args: vec![],
            }]
            .into(),
            variables: VariableContext::default(),
            options: ExecutionOptions {
                max_steps: 0,
                ..ExecutionOptions::default()
            },
        };
        assert!(matches!(
            execute(request),
            Err(ExecutionError::StepLimitExceeded { limit: 0 })
        ));

        let request = ExecutionRequest {
            input: b"hello".to_vec(),
            input_supplied: true,
            recipe: vec![RecipeStep {
                op: "To Base64".into(),
                args: vec![],
            }]
            .into(),
            variables: VariableContext::default(),
            options: ExecutionOptions {
                max_output_bytes: Some(4),
                ..ExecutionOptions::default()
            },
        };
        assert!(matches!(
            execute(request),
            Err(ExecutionError::OutputLimitExceeded { limit: 4, .. })
        ));
    }

    #[test]
    fn trace_contains_sizes_but_no_payloads() {
        let outcome = execute(ExecutionRequest {
            input: b"secret".to_vec(),
            input_supplied: true,
            recipe: vec![RecipeStep {
                op: "To Base64".into(),
                args: vec![],
            }]
            .into(),
            variables: VariableContext::default(),
            options: ExecutionOptions {
                trace: true,
                ..ExecutionOptions::default()
            },
        })
        .unwrap();
        assert_eq!(outcome.trace.len(), 1);
        assert_eq!(outcome.trace[0].input_bytes, 6);
        assert_eq!(outcome.trace[0].output_bytes, 8);
    }

    #[test]
    fn input_requirement_distinguishes_missing_from_explicit_empty() {
        let recipe = Recipe::from(vec![RecipeStep {
            op: "To Base64".into(),
            args: vec![],
        }]);
        let missing = execute(ExecutionRequest {
            input: Vec::new(),
            input_supplied: false,
            recipe: recipe.clone(),
            variables: VariableContext::default(),
            options: ExecutionOptions::default(),
        });
        assert!(matches!(missing, Err(ExecutionError::Step { .. })));

        let empty = execute(ExecutionRequest {
            input: Vec::new(),
            input_supplied: true,
            recipe,
            variables: VariableContext::default(),
            options: ExecutionOptions::default(),
        })
        .unwrap();
        assert!(empty.output.is_empty());

        let generated = execute(ExecutionRequest {
            input: Vec::new(),
            input_supplied: false,
            recipe: Recipe::from(vec![RecipeStep {
                op: "Generate UUID".into(),
                args: vec![],
            }]),
            variables: VariableContext::default(),
            options: ExecutionOptions::default(),
        })
        .unwrap();
        assert!(!generated.output.is_empty());
    }
}
