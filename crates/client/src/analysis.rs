//! Transport adapters for the existing Magic and streaming Scan engines.
use crate::*;

fn depth() -> usize {
    3
}
fn results() -> usize {
    20
}
fn min_len() -> usize {
    16
}
fn findings() -> usize {
    100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MagicRequest {
    pub input_base64: String,
    #[serde(default = "depth")]
    pub depth: usize,
    #[serde(default)]
    pub intensive: bool,
    #[serde(default = "results")]
    pub max_results: usize,
    #[serde(default)]
    pub crib: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MagicCandidate {
    pub recipe: Vec<execution::RecipeStep>,
    pub result: integration::ExecutionResult,
    pub score: f64,
    pub entropy: f64,
    pub printable_ratio: f64,
    pub matched_crib: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ScanRequest {
    pub input_base64: String,
    #[serde(default = "min_len")]
    pub min_len: usize,
    #[serde(default = "findings")]
    pub max_findings: usize,
    #[serde(default)]
    pub decode: bool,
    #[serde(default = "depth")]
    pub depth: usize,
    #[serde(default)]
    pub crib: Option<String>,
}

fn crib(pattern: Option<String>) -> Result<Option<regex::Regex>, ClientError> {
    pattern
        .map(|pattern| {
            if pattern.len() > 4096 {
                return Err(ClientError::new(
                    ErrorKind::LimitExceeded,
                    "crib exceeds pattern limit",
                ));
            }
            regex::RegexBuilder::new(&pattern)
                .size_limit(1024 * 1024)
                .build()
                .map_err(|_| ClientError::new(ErrorKind::InvalidArgument, "invalid crib regex"))
        })
        .transpose()
}

impl LocalEngineClient {
    pub(crate) fn magic_request(
        &self,
        request: MagicRequest,
    ) -> Result<Vec<MagicCandidate>, ClientError> {
        let input = self.decode_input(&request.input_base64)?;
        if request.depth > 8 || request.max_results > 100 {
            return Err(ClientError::new(
                ErrorKind::LimitExceeded,
                "Magic depth/results exceed limits (8/100)",
            ));
        }
        let opts = rxchef::magic::MagicOptions {
            depth: request.depth,
            intensive: request.intensive,
            max_results: request.max_results,
            crib: crib(request.crib)?,
            max_candidates: self.policy.max_steps.min(256),
            max_candidate_bytes: self.policy.max_output_bytes,
            max_total_decoded_bytes: self.policy.max_output_bytes,
        };
        Ok(
            rxchef::magic::magic_with_filter(&input, &opts, &|op| self.check_operation(op).is_ok())
                .into_iter()
                .map(|candidate| MagicCandidate {
                    recipe: candidate
                        .recipe
                        .into_iter()
                        .map(|step| execution::RecipeStep {
                            op: step.op,
                            args: step.args,
                        })
                        .collect(),
                    result: integration::ExecutionResult::from_bytes(candidate.data),
                    score: candidate.score,
                    entropy: candidate.entropy,
                    printable_ratio: candidate.printable_ratio,
                    matched_crib: candidate.matched_crib,
                })
                .collect(),
        )
    }

    pub(crate) fn scan_request(
        &self,
        request: ScanRequest,
    ) -> Result<Vec<rxchef::scan::Finding>, ClientError> {
        let input = self.decode_input(&request.input_base64)?;
        if request.depth > 8
            || request.max_findings > 1000
            || request.min_len == 0
            || request.min_len > 65536
        {
            return Err(ClientError::new(
                ErrorKind::LimitExceeded,
                "Scan limits: depth <= 8, findings <= 1000, token minimum 1..65536",
            ));
        }
        let opts = rxchef::scan::ScanOptions {
            min_len: request.min_len,
            max_len: 65536,
            max_findings: request.max_findings,
            depth: request.depth,
            decode: request.decode,
            crib: crib(request.crib)?,
            ..Default::default()
        };
        let client = self.clone();
        let mut scanner =
            rxchef::scan::Scanner::new(opts).with_decoder_filter(std::sync::Arc::new(move |op| {
                client.check_operation(op).is_ok()
            }));
        let mut findings = Vec::new();
        for chunk in input.chunks(64 * 1024) {
            scanner.push(chunk, &mut |finding| findings.push(finding));
            if scanner.limit_reached() {
                break;
            }
        }
        scanner.finish(&mut |finding| findings.push(finding));
        Ok(findings)
    }
}
