use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct GradingPolicy {
    pub budget: u64,
    pub memory: u64,
    pub score: u64,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct Testcase {
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stdin_file: Option<String>,
    pub stdout_file: Option<String>,
    pub score: u64,
    pub description: Option<String>,
    pub sample: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct Problem {
    pub name: String,
    pub description: String,
    pub policy: Vec<GradingPolicy>,
    pub testcase: Vec<Testcase>,
    pub input: Option<String>,
    pub output: Option<String>,
    pub hint: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemSummary {
    pub id: String,
    pub name: String,
    pub tags: Vec<String>,
}
