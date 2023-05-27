#![warn(unused_assignments)]
extern crate chrono;
extern crate git2;
use crate::*;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use git2::{Error, Repository};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub fn config_repo_name(repo: &Repository) -> Result<String, Error> {
    let repo_path = repo.path();
    let repo_dir = repo_path
        .parent()
        .ok_or_else(|| Error::from_str("Invalid repository path"))?;
    let repo_name = repo_dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    if repo_name.ends_with(".git") {
        Ok(repo_name[..repo_name.len() - 4].to_string())
    } else {
        Ok(repo_name)
    }
}

pub fn traverse_tree(
    repo: &Repository,
    tree: &git2::Tree,
    path: &str,
    files: &mut Vec<(String, String)>,
) -> Result<(), Error> {
    for entry in tree.iter() {
        let entry_path = format!("{}/{}", path, entry.name().unwrap());
        if entry.kind() == Some(git2::ObjectType::Blob) {
            let blob = repo.find_blob(entry.id())?;
            let content = String::from_utf8_lossy(blob.content());
            files.push((entry_path, content.to_string()));
        } else if entry.kind() == Some(git2::ObjectType::Tree) {
            let subtree = repo.find_tree(entry.id())?;
            traverse_tree(repo, &subtree, &entry_path, files)?;
        }
    }
    Ok(())
}
pub fn config_commit_info(repo: &Repository, commit: &git2::Commit) -> Result<CommitInfo, Error> {
    let commit_id = commit.id();
    let author = commit.author();
    let email = author.email().unwrap_or("").to_string();
    let commit_message = commit.message().unwrap_or("").to_string();
    let date = Utc.timestamp(commit.time().seconds(), 0);
    let offset = FixedOffset::west(commit.time().offset_minutes() * 60);
    let mut files = Vec::new();
    let repo_name = config_repo_name(repo)?;
    let tags = vec![];
    //todo
    let operation ="addition".to_owned();
    // Retrieve the tree of the commit
    let tree = commit.tree()?;
    // Traverse the tree to get the file paths and content
    traverse_tree(repo, &tree, "", &mut files)?;

    let commit_info = CommitInfo {
        repo: repo_name,
        commit: commit_id,
        author: author.name().unwrap_or("").to_string(),
        email,
        commit_message,
        date: offset.from_utc_datetime(&date.naive_utc()),
        files,
        tags,
        operation,
    };

    Ok(commit_info)
}

pub fn load_all_commits(repo: &Repository) -> Result<Vec<String>, git2::Error> {
 
    let mut revwalk = repo.revwalk()?;
    
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;
    
    let mut commits = Vec::new();
    
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let commit_id = commit.id().to_string();
        commits.push(commit_id);
    }
    
    Ok(commits)
}

pub fn load_commits_by_conditions(commit_from: Option<String>, commit_to: Option<String>, commits: &[String]) -> Vec<String> {
    match (commit_from, commit_to) {
        (Some(start_commit), Some(end_commit)) => {
            let start_index = commits.iter().position(|commit| *commit == start_commit);
            let end_index = commits.iter().position(|commit| *commit == end_commit);

            if let (Some(start), Some(end)) = (start_index, end_index) {
                if start <= end {
                    commits[start..=end].to_vec()
                } else {
                    Vec::new() // Return an empty vector if start_commit is after end_commit
                }
            } else {
                Vec::new() // Return an empty vector if either commit is not found
            }
        }
        _ => Vec::new(), // Return an empty vector if either commit_from or commit_to is None
    }
}

pub fn load_repository(repo_path: &str) -> Result<Repository, Box<dyn std::error::Error>> {
    let repo = match Repository::open(repo_path) {
        Ok(repo) => {
            println!("Successfullt open {}", repo.path().display());
            repo
        }
        Err(e) => {
            eprintln!("Fail to load repo{}", e);
            return Err(Box::new(e));
        }
    };

    Ok(repo)
}

pub fn config_printed_repo_info(repo: &Repository) {
    // Get all object IDs in the repository
    let object_ids = load_all_object_ids(repo).unwrap();

    // Initialize counters for different object types
    let mut total_count = 0;
    let mut delta_count = 0;
    let mut reused_count = 0;

    // Iterate over each object ID
    for object_id in object_ids {
        // Find the Git object using its ID
        let object = repo.find_object(object_id, None).unwrap();

        // Increment the total count
        total_count += 1;

        // Check if the object is of type Commit
        if object.kind() == Some(git2::ObjectType::Commit) {
            // Increment the delta count
            delta_count += 1;
        }
    }

    // Calculate the reused count (assuming you have the logic for it)
    reused_count = total_count - delta_count;

    // Print the information
    println!("Enumerating objects: {} done.", total_count);
    println!(
        "Total {} (delta {}), reused {} , pack-reused 0",
        total_count, delta_count, reused_count
    );
}

pub fn load_all_object_ids(repo: &Repository) -> Result<Vec<git2::Oid>, git2::Error> {
    let mut object_ids = Vec::new();
    let odb = repo.odb()?;

    odb.foreach(|id| {
        object_ids.push(*id);
        true
    })?;

    Ok(object_ids)
}

pub fn parse_start_date_to_datetime(input: &str, mytype: &str) -> Result<DateTime<Utc>, &'static str> {
    let date = NaiveDate::parse_from_str(input, "%Y-%m-%d").map_err(|_| "Invalid date format")?;
    let time: NaiveTime;
    if mytype == "start" {
        if let Some(t) = NaiveTime::from_hms_opt(0, 0, 0) {
            time = t;
        } else {
            return Err("Invalid time format");
        }
    } else  if let Some(t) = NaiveTime::from_hms_opt(23, 59, 59) {
                time = t;
    } else {
           return Err("Invalid time format");
    }
        
    let datetime = NaiveDateTime::new(date, time);
    let datetime_utc = DateTime::from_utc(datetime, Utc);
    Ok(datetime_utc)
}
pub fn is_valid_date_format(input: &str) -> bool {
    if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        // 日期解析成功
        let formatted = date.format("%Y-%m-%d").to_string();
        return formatted == input;
    }
    false
}

pub fn load_commit_tags(repo: &Repository, commit_id: git2::Oid) -> Result<Vec<String>, Error> {
    let tags = repo.tag_names(None)?;
    let mut commit_tags = Vec::new();

    for tag_name in tags.iter().flatten() {
        let target_id = repo.revparse_single(tag_name)?.peel_to_commit()?.id();
        if target_id == commit_id {
            commit_tags.push(tag_name.to_string());
        }
    }
    

    Ok(commit_tags)
}