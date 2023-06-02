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

#[derive(Debug, Serialize, Deserialize)]
pub struct FastIOJudgeSpec {
    /// The input string
    pub input: Option<String>,
    /// The URL to fetch the input from
    pub input_url: Option<String>,
    /// The token used to authenticate the input URL
    pub input_auth: Option<String>,
    /// The expected output hash
    pub output_hash: String,
    /// The maximum cost of the program
    pub cost: u64,
    /// The maximum memory of the program
    pub memory: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "judger")]
pub enum JudgeSpec {
    IOFast(FastIOJudgeSpec),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JudgeSpecs {
    pub specs: Vec<JudgeSpec>,
}
