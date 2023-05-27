#![warn(clippy::new_without_default)]
use clap::Parser;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};
/// Represents the configuration for sensleaks tool.
#[derive(Parser, Debug)]
#[command(
    author = "Chen Yijun",
    version = "0.1.1",
    about = "sensleaks-rs",
    long_about = "sensleaks: A tool to detect sensitive information in Git repository",
    after_help = "Repository: https://github.com/open-rust-initiative/sensleak-rs"
)]
pub struct Config {
    /// Target repository.
    #[arg(short, long)]
    pub repo: String,

    /// sha of commit to scan or "latest" to scan the last commit of the repository
    #[arg(long)]
    pub commit: Option<String>,

    /// comma separated list of a commits to scan
    #[arg(long)]
    pub commits: Option<String>,

    /// file of new line separated list of a commits to scan
    #[arg(long)]
    pub commits_file: Option<String>,

    /// Scan commits more recent than a specific date. Ex: '2006-01-02' or '2023-01-02T15:04:05-0700' format.
    #[arg(long)]
    pub commit_since: Option<String>,

    /// Scan commits older than a specific date. Ex: '2006-01-02' or '2006-10-02T15:04:05-0700' format.
    #[arg(long)]
    pub commit_until: Option<String>,

    /// Commit to start scan from
    #[arg(long)]
    pub commit_from: Option<String>,

    /// Commit to stop scan
    #[arg(long)]
    pub commit_to: Option<String>,

    /// Branch to scan
    #[arg(long)]
    pub branch: Option<String>,

    /// run gitleaks on uncommitted code
    #[arg(long,default_value = "true")]
    pub uncommitted: bool ,

    /// user to scan
    #[arg(long)]
    pub user: Option<String>,
    // /// Config path..
    // #[arg(short = 'c', long, default_value = "gitleaks.toml")]
    // pub config: String,

    // /// Path to write json leaks file.
    // #[arg(short = 'o', long, default_value = "")]
    // pub report: String,

    // /// Show verbose output from scan.
    // #[arg(short = 'v', long, default_value = "false")]
    // pub verbose: bool,

    // /// Pretty print json if leaks are present.
    // #[arg(short = 'e', long, default_value = "false")]
    // pub pretty: bool,
}

#[derive(Debug)]
pub struct CommitInfo {
    pub repo: String,
    pub commit: git2::Oid,
    pub author: String,
    pub email: String,
    pub commit_message: String,
    pub date: DateTime<FixedOffset>,
    pub files: Vec<(String, String)>,
    pub tags: Vec<String>,
    pub operation: String,
}
/// # An array of tables that contain information that define instructions on how to detect secrets.
#[derive(Debug)]
pub struct Rule {
    /// Short human readable description of the rule.
    pub description: String,

    /// Unique identifier for this rule.
    pub id: String,

    /// Regular expression used to detect secrets.
    pub regex: String,

    /// Float representing the minimum shannon entropy a regex group must have to be considered a secret.
    pub entropy: Option<f64>,

    /// Keywords are used for pre-regex check filtering. Rules that contain keywords will perform a quick string compare check to make sure the keyword(s) are in the content being scanned. Ideally these values should either be part of the idenitifer or unique strings specific to the rule's regex
    pub keywords: Vec<String>,

    /// You can include an allowlist table for a single rule to reduce false positives or ignore commits with known/rotated secrets.
    pub allowlist: Option<Allowlist>,
}

impl Rule {
    pub fn new() -> Rule{
        Rule{
            description: String::from("11"),
            id:  String::from("11"),
            regex:  String::from("11"),
            entropy: Some(3.1),
            keywords: Vec::new(),
            allowlist: None
        }
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self::new()
    }
}

/// Skip the allowlist file
#[derive(Debug, Deserialize)]
pub struct Allowlist {
    /// Skip the paths.
    pub paths: Vec<String>,

    /// Skip the commits.
    pub commits: Vec<String>,

    /// Acceptable values for regexTarget are "match" and "line".
    pub regex_target: String,

    /// Skip the secrets that satisfy the regexes.
    pub regexes: Vec<String>,

    /// Skip the secrets that contain the stopwords.
    pub stopwords: Vec<String>,
}
impl Allowlist {
    pub fn new() -> Allowlist {
        Allowlist {
            paths: Vec::new(),
            commits: Vec::new(),
            regex_target: String::from("match"),
            regexes: Vec::new(),
            stopwords: Vec::new(),
        }
    }
}
impl Default for Allowlist {
    fn default() -> Self {
        Self::new()
    }
}
/// Represents an item in the scanned output.
#[derive(Debug, Serialize, Deserialize)]
pub struct OutputItem {
    /// The line containing the sensitive information.
    pub line: String,

    /// The line number where the sensitive information is found.
    pub line_number: u32,

    /// The sensitive information detected.
    pub secret: String,

    /// The entropy of the sensitive information.
    pub entropy: String,

    /// The commit info.
    pub commit: String,

    /// The repository where the sensitive information is found.
    pub repo: String,

    /// The rule used to detect the sensitive information.
    pub rule: String,

    /// The commit message associated with the sensitive information.
    pub commit_message: String,

    /// The author of the commit.
    pub author: String,

    /// The email of the commit author.
    pub email: String,

    /// The file path where the sensitive information is found.
    pub file: String,

    /// The date of the commit.
    pub date: String,

    /// Tags .
    pub tags: String,

    /// The operation .
    pub operation: String,
}
