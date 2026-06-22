use serde::{Deserialize, Serialize};

// ─── Recipe ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeStep {
    pub op: String,
    #[serde(default)]
    pub args: Vec<String>,
}

/// A named, saved pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub steps: Vec<RecipeStep>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl Recipe {
    pub fn new(name: impl Into<String>) -> Self {
        Recipe {
            name: name.into(),
            description: String::new(),
            steps: Vec::new(),
            tags: Vec::new(),
        }
    }
}

// ─── Variables ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
}

// ─── History ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryStep {
    pub op: String,
    #[serde(default)]
    pub args: Vec<String>,
    /// First 300 characters of the output (UTF-8 lossy or hex for binary).
    pub output_preview: String,
    pub output_bytes: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub timestamp: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    /// First 300 chars of input (UTF-8 lossy or hex).
    pub input_preview: String,
    pub input_bytes: usize,
    pub steps: Vec<HistoryStep>,
    pub output_preview: String,
    pub output_bytes: usize,
    pub success: bool,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Create a preview string from raw bytes: UTF-8 text or hex dump.
pub fn bytes_preview(bytes: &[u8], max_chars: usize) -> String {
    match std::str::from_utf8(bytes) {
        Ok(s) => {
            let s = s.trim_end();
            if s.len() <= max_chars {
                s.replace('\n', "↵").replace('\r', "")
            } else {
                format!("{}…", &s[..max_chars].replace('\n', "↵"))
            }
        }
        Err(_) => {
            let hex: String = bytes
                .iter()
                .take(max_chars / 3 + 1)
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ");
            if bytes.len() * 3 > max_chars {
                format!("{}…", hex)
            } else {
                hex
            }
        }
    }
}
