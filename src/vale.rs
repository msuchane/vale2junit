use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Alerts(HashMap<PathBuf, Vec<Alert>>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Alert {
    span: (u64, u64),
    check: String,
    description: String,
    link: String,
    message: String,
    severity: Severity,
    r#match: String,
    line: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Suggestion,
    Warning,
    Error,
}