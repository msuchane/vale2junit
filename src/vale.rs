use std::collections::HashMap;
use std::path::PathBuf;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Alerts(pub HashMap<PathBuf, Vec<Alert>>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Alert {
    pub span: (u64, u64),
    pub check: String,
    pub description: String,
    pub link: String,
    pub message: String,
    pub severity: Severity,
    pub r#match: String,
    pub line: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Suggestion,
    Warning,
    Error,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Suggestion => "ℹ️Suggestionℹ️",
            Self::Warning => "⚠️Warning⚠️",
            Self::Error => "❌Error❌",
        }
    }
}

impl Alert {
    pub fn main_description(&self) -> String {
        format!(
            "{} Line {}:{}. {}",
            self.severity.as_str(), self.line, self.span.0, self.message
        )
    }

    pub fn details(&self) -> String {
        format!("Line {}. Span {}–{}. Match `{}`.", self.line, self.span.0, self.span.1, self.r#match)
    }
}
