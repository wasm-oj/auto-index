use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    /// The budget allocated for executing the solution.
    pub budget: u64,

    /// The maximum amount of memory that can be used during execution.
    pub memory: u64,

    /// The score assigned to the solution based on its performance.
    pub score: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Testcase {
    /// File system layers for the test case.
    pub fs: HashMap<String, HashMap<String, String>>,

    /// The score assigned to this particular test case.
    pub score: i64,

    /// Indicates if this test case is a sample or not.
    /// Sample test cases are generally provided for illustration.
    pub sample: Option<bool>,

    /// A textual description or note related to the test case.
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    /// The unique identifier for the problem.
    pub id: String,

    /// An array of tags associated with the problem,
    /// generally used for categorization and search.
    pub tags: Vec<String>,

    /// The name or title of the problem.
    pub name: String,

    /// A detailed textual description of the problem.
    pub description: String,

    /// An array of policies related to the problem,
    /// which dictate constraints like memory and budget.
    pub policies: Vec<Policy>,

    /// An array of test cases associated with the problem.
    pub testcases: Vec<Testcase>,
}

/// Represents a raw programming problem spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialProblem {
    /// An array of tags associated with the problem.
    pub tags: Option<Vec<String>>,

    /// The name or title of the problem.
    pub name: String,

    /// A detailed textual description of the problem.
    pub description: String,

    /// An array of policies related to the problem.
    pub policy: Vec<Policy>,

    /// An array of test cases associated with the problem.
    pub testcase: Vec<Testcase>,
}

impl From<PartialProblem> for Problem {
    fn from(partial: PartialProblem) -> Self {
        Problem {
            id: "".to_owned(),
            tags: partial.tags.unwrap_or(Vec::new()),
            name: partial.name,
            description: partial.description,
            policies: partial.policy,
            testcases: partial.testcase,
        }
    }
}
