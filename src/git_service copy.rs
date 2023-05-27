extern crate chrono;
#[warn(unused_imports)]
#[warn(deprecated)]
#[warn(unused_variables)]
#[warn(dead_code)]
#[warn(unused_variables)]
extern crate git2;
use crate::Config;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use clap::Parser;
use git2::BranchType;
use git2::{Error, Repository,StatusOptions};
use std::collections::HashSet;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::process;
use std::fs::File;
use std::io::Read;
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
pub fn start_git() {
    let args = Config::parse();
    let excluded_commits: Vec<git2::Oid> = vec![];
    let repo = match open_repository(&args.repo) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Fail to open the repository {}", e);
            process::exit(0);
        }
    };
    // match (&args.commit, &args.commits, &args.commits_file, &args.commit_since, &args.commit_until, &args.commit_from, &args.commit_to) {
    //     (Some(commit), None, None, None, None, None, None) => {

    //         if let Err(e) = handle_single_commit(repo, commit) {
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
    //     }
    //     (None, Some(commits), None, None, None, None, None) => {
    //         let commit_ids: Vec<&str> = commits.split(',').collect();
    //         if let Err(e) = handle_multiple_commits(repo, &commit_ids) {
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
    //     }
    //     (None, None, Some(file_path), None, None, None, None) => {
    //         if let Err(e) = handle_commits_file(repo, &file_path) {
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
    //     }
    //     (None, None, None, Some(since), Some(until), None, None) => {
    //         if let Err(e) = handle_commit_range_by_time(repo, since, until) {
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
    //     }
    //     (None, None, None, None, None, Some(commit_from), Some(commit_to)) => {
    //         if let Err(e) = handle_commit_range(repo, Some(commit_from.clone()), Some(commit_to.clone())){
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
    //     }
    //     _ => {
    //         // Handle the case where no relevant arguments are provided
    //     }
    // }
    
    

    // // 扫描一个commit的内容
    // match &args.commit {
    //     Some(commit) => {
    //         if let Err(e) = handle_single_commit(repo, commit) {
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
    //     }
    //     None => {
            
    //     }
    // }
    // // 扫描多个commit
    // match &args.commits {
    //     Some(commits) => {
    //         let commit_ids: Vec<&str> = commits.split(',').collect();
    //         if let Err(e) = handle_multiple_commits(repo, &commit_ids) {
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
    //     }
    //     None => {

    //     }
    // }
    // // 扫描commit file
    // match &args.commits_file {
    //     Some(file_path) => {
    //         if let Err(e) = handle_commits_file(repo, &file_path) {
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
    //     }
    //     None => {
    //     }
    // }

    // match (&args.commit_since, &args.commit_until) {
    //     (Some(since), Some(until)) => {
    //         if let Err(e) = handle_commit_range_by_time(repo, since,until) {
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
    //     }
    //     _ => {

    //     }
    // }

    // match (&args.commit_from, &args.commit_to) {
    //     (Some(commit_from), Some(commit_to)) => {
    //         if let Err(e) = handle_commit_range(repo, Some(commit_from.clone()), Some(commit_to.clone())){
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
            
    //     }
    //     _ => {
    //     }
    // }
    ////////////////////////////////////////////////////////////////
     // match (&args.commit_since, &args.commit_until) {
    //     (Some(since), Some(until)) => {
    //         if let Err(e) = handle_commit_range_by_time(repo, since,until) {
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
    //     }
    //     _ => {

    //     }
    // }
    match args.uncommitted {
        true => {
            if let Err(e) = handle_uncommitted_files(repo) {
                eprintln!("Application error: {}", e);
                process::exit(0);
            }
        }
        false => {
            // 处理未提供未提交选项的情况
        }
    }
    

}
// 扫描一个commit的内容
pub fn handle_single_commit(repo: Repository, commit_id: &str) -> Result<(), git2::Error> {
    let commit = repo.find_commit(git2::Oid::from_str(commit_id)?)?;
    let commit_info = get_commit_info(&repo, &commit)?;
    print_commit_info(&commit_info);
    Ok(())
}
// 扫描多个commit的内容
pub fn handle_multiple_commits(repo: Repository, commit_ids: &[&str]) -> Result<(), git2::Error> {
    println!("Total Commits: {}", commit_ids.len());

    println!("Commit history:");
    for commit_id in commit_ids {
        let commit = repo.find_commit(git2::Oid::from_str(commit_id)?)?;
        let commit_info = get_commit_info(&repo, &commit)?;
        print_commit_info(&commit_info);
    }
    Ok(())
}
// 扫描commit文件
pub fn handle_commits_file(repo: Repository, file_name: &str)-> Result<(), git2::Error> {
    let file = fs::File::open(file_name).expect("Failed to open commits file");
    let reader = BufReader::new(file);
    let mut commits: Vec<String> = Vec::new();

    for line in reader.lines() {
        if let Ok(commit_line) = line {
            commits.push(commit_line);
        }
    }

    let commit_ids: Vec<&str> = commits.iter().map(|s| s.as_str()).collect();

    handle_multiple_commits(repo, &commit_ids)
}
// 扫描commit，根据时间
fn handle_commit_range_by_time(repo: Repository, since: &str, until: &str) -> Result<(), git2::Error>{
    //TODO
    let excluded_commits: Vec<git2::Oid> = vec![];
    let is_since_rfc3339 = DateTime::parse_from_rfc3339(since).is_ok();
    let is_until_rfc3339 = DateTime::parse_from_rfc3339(until).is_ok();
    
    let is_since_date = is_valid_date_format(since);
    let is_until_date = is_valid_date_format(until);

    if is_since_date && is_until_date {
        let start_time = match parse_start_date_to_datetime(&since, "start") {
            Ok(datetime) => datetime.with_timezone(&FixedOffset::east(0)),
            Err(err) => {
                eprintln!("时间格式错误 error: {}", err);
                process::exit(0);
            }
        };

        let end_time = match parse_start_date_to_datetime(&until, "until") {
            Ok(datetime) => datetime.with_timezone(&FixedOffset::east(0)),
            Err(err) => {
                eprintln!("时间格式错误 error: {}", err);
                process::exit(0);
            }
        };

        // 调用处理函数并传递日期参数
         handle_multiple_commits_by_time(&repo, &excluded_commits, start_time, end_time)
         
        // 处理两者都是 %Y-%m-%d 格式的情况
    } else if is_since_rfc3339 && is_until_rfc3339 {
        // 处理两者都是 RFC 3339 格式的情况
        let start_time = DateTime::parse_from_rfc3339(&since).unwrap();
        let end_time = DateTime::parse_from_rfc3339(&until).unwrap();

        // 调用处理函数并传递日期参数
        handle_multiple_commits_by_time(&repo, &excluded_commits, start_time, end_time)
        
    } else {
        eprintln!("Application error: 格式不正确");
        process::exit(0);
        // 处理其他情况
    }
}

pub fn handle_multiple_commits_by_time(
    repo: &Repository,
    excluded_commits: &[git2::Oid],
    start_time: DateTime<FixedOffset>,
    end_time: DateTime<FixedOffset>,
) -> Result<(), git2::Error> {
    let head = repo.head()?;
    let obj = head.peel(git2::ObjectType::Commit)?;
    let commit = if let Some(commit) = obj.as_commit() {
        commit.clone()
    } else {
        return Err(Error::from_str("Failed to convert object to commit"));
    };

    let mut revwalk = repo.revwalk()?;
    revwalk.push(commit.id())?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

    let mut commits = Vec::new();

    let excluded_commits: HashSet<_> = excluded_commits.iter().cloned().collect();

    for commit_id in revwalk {
        let oid = commit_id?;
        if excluded_commits.contains(&oid) {
            continue; // Skip excluded commits
        }
        let commit = repo.find_commit(oid)?;
        let commit_time = Utc.timestamp(commit.time().seconds(), 0);
        let commit_offset = FixedOffset::west(commit.time().offset_minutes() * 60);
        let commit_date = commit_offset.from_utc_datetime(&commit_time.naive_utc());

        if commit_date >= start_time && commit_date <= end_time {
            let commit_info = get_commit_info(&repo, &commit)?;
            commits.push(commit_info);
        }
    }

    println!("Total Commits: {}", commits.len());
    println!("Commit history:");
    for commit_info in commits {
        print_commit_info(&commit_info);
    }
    Ok(())
}

pub fn handle_branches_by_name(
    repo: Repository,
    branch_name: &str,
) -> Result<(), Error> {
    let branches = repo.branches(Some(BranchType::Local))?;

    let mut commits = Vec::new();
    // let mut commits_ids = Vec::new();

    for branch in branches {
        let (branch, _) = branch?;
        let branch_reference = branch.into_reference();
        let branch_name_str = branch_reference.name().unwrap_or("");

        if branch_name_str.contains(branch_name) {
            let commit_oid = branch_reference
                .target()
                .ok_or_else(|| Error::from_str("Failed to get branch commit"))?;
            let commit = repo.find_commit(commit_oid)?;
            let commit_info = get_commit_info(&repo, &commit)?;
            commits.push(commit_info);
            // commits_ids.push(commit_oid.to_string());
        }
    }
    for commit in &commits {
        print_commit_info(commit);
    }
    Ok(())
    // Ok(commits)
}

// pub fn handle_commits_with_exclusion(
//     repo: Repository,
//     excluded_commits: &[&str],
// ) -> Result<(), git2::Error> {
//     let excluded_oids: Vec<git2::Oid> = excluded_commits
//         .iter()
//         .map(|commit| git2::Oid::from_str(commit).expect("Invalid OID"))
//         .collect();

//     let commits = get_all_commits(&repo, &excluded_oids)?;

//     println!("Total Commits: {}", commits.len());

//     println!("Commit history:");
//     for commit in commits {
//         print_commit_info(&commit);
//     }

//     Ok(())
// }

pub fn get_commit_info(repo: &Repository, commit: &git2::Commit) -> Result<CommitInfo, Error> {
    let commit_id = commit.id();
    let author = commit.author();
    let email = author.email().unwrap_or("").to_string();
    let commit_message = commit.message().unwrap_or("").to_string();
    let date = Utc.timestamp(commit.time().seconds(), 0);
    let offset = FixedOffset::west(commit.time().offset_minutes() * 60);
    let mut files = Vec::new();
    let repo_name = get_repo_name(repo)?;
    let tags = vec![];
    let operation = get_commit_operation(&commit, &repo)?;
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

pub fn get_commit_operation(commit: &git2::Commit, repo: &Repository) -> Result<String, Error> {
    // TODO: find Git tags
    // let mut operation = String::new();
    // let parents_count = commit.parent_count();

    // println!("{}",parents_count);
    // if parents_count == 0 {
    //     operation.push_str("Initial commit");
    // } else if parents_count == 1 {
    //     let parent_commit = commit.parent(0)?;
    //     let parent_tree = parent_commit.tree()?;
    //     let tree = commit.tree()?;

    //     let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
    //     let deltas = diff.deltas();
    //     if deltas.len() == 0 {
    //         operation.push_str("No changes");
    //     } else {

    //         for delta in deltas {
    //             println!("{:?}",delta);
    //             match delta.status() {
    //                 git2::Delta::Added => operation.push_str("Added"),
    //                 git2::Delta::Deleted => operation.push_str("Deleted"),
    //                 git2::Delta::Modified => operation.push_str("Modified"),
    //                 git2::Delta::Renamed => operation.push_str("Renamed"),
    //                 git2::Delta::Copied => operation.push_str("Copied"),
    //                 git2::Delta::Unmodified => operation.push_str("Unmodified"),
    //                 git2::Delta::Ignored => operation.push_str("Ignored"),
    //                 git2::Delta::Untracked => operation.push_str("Untracked"),
    //                 git2::Delta::Typechange => operation.push_str("Typechange"),
    //                 git2::Delta::Unreadable => operation.push_str("Unreadable"),
    //                 git2::Delta::Conflicted => operation.push_str("Conflicted"),
    //             }

    //             operation.push_str(", ");
    //         }

    //         operation.pop(); // Remove trailing comma
    //         operation.pop(); // Remove trailing space
    //     }
    // }
    // Ok(operation)
    Ok("operation".trim().to_string())
}

pub fn get_commit_tags(repo: &Repository, commit_id: git2::Oid) -> Result<Vec<String>, Error> {
    let tags = repo.tag_names(None)?;
    let mut commit_tags = Vec::new();

    for tag in tags.iter() {
        if let Some(tag_name) = tag {
            let target_id = repo.revparse_single(tag_name)?.peel_to_commit()?.id();
            if target_id == commit_id {
                commit_tags.push(tag_name.to_string());
            }
        }
    }

    Ok(commit_tags)
}
pub fn get_repo_name(repo: &Repository) -> Result<String, Error> {
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

// pub fn get_all_commits(
//     repo: &Repository,
//     excluded_commits: &[git2::Oid],
// ) -> Result<Vec<CommitInfo>, Error> {
//     let head = repo.head()?;
//     let obj = head.peel(git2::ObjectType::Commit)?;
//     let commit = if let Some(commit) = obj.as_commit() {
//         commit.clone()
//     } else {
//         return Err(Error::from_str("Failed to convert object to commit"));
//     };

//     let mut revwalk = repo.revwalk()?;
//     revwalk.push(commit.id())?;
//     revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

//     let mut commits = Vec::new();

//     let excluded_commits: HashSet<_> = excluded_commits.iter().cloned().collect();

//     for commit_id in revwalk {
//         let oid = commit_id?;
//         if excluded_commits.contains(&oid) {
//             continue; // Skip excluded commits
//         }
//         let commit = repo.find_commit(oid)?;
//         let commit_info = get_commit_info(&repo, &commit)?;
//         commits.push(commit_info);
//     }

//     Ok(commits)
// }

pub fn print_commit_info(commit_info: &CommitInfo) {
    println!("repo: {}", commit_info.repo);
    println!("commit: {}", commit_info.commit);
    println!("author: {}", commit_info.author);
    println!("email: {}", commit_info.email);
    println!("commitMessage: {}", commit_info.commit_message);
    println!("tags: {:?}", commit_info.tags.to_owned());
    println!("Operation: {}", commit_info.operation);
    println!("date: {}", commit_info.date.format("%Y-%m-%dT%H:%M:%S%z"));
    println!("Files:");
    for (file, content) in &commit_info.files {
        println!("File: {}", file);
        // println!("Content:\n{}", content);
        println!("----------------------");
    }

    println!("======================");
}


pub fn print_git_repository_info(repo: &Repository) {
    // Get all object IDs in the repository
    let object_ids = get_all_object_ids(&repo).unwrap();

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

pub fn get_all_object_ids(repo: &Repository) -> Result<Vec<git2::Oid>, git2::Error> {
    let mut object_ids = Vec::new();
    let odb = repo.odb()?;

    odb.foreach(|id| {
        object_ids.push(*id);
        true
    })?;

    Ok(object_ids)
}
pub fn open_repository(repo_path: &str) -> Result<Repository, Box<dyn std::error::Error>> {
    let repo = match Repository::open(repo_path) {
        Ok(repo) => {
            println!("成功打开仓库：{}", repo.path().display());
            repo
        }
        Err(e) => {
            eprintln!("无法打开仓库：{}", e);
            return Err(Box::new(e));
        }
    };

    Ok(repo)
}
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

fn parse_start_date_to_datetime(input: &str, mytype: &str) -> Result<DateTime<Utc>, &'static str> {
    let date = NaiveDate::parse_from_str(input, "%Y-%m-%d").map_err(|_| "Invalid date format")?;
    let time: NaiveTime;
    if mytype == "start" {
        if let Some(t) = NaiveTime::from_hms_opt(0, 0, 0) {
            time = t;
        } else {
            return Err("Invalid time format");
        }
    } else {
        if let Some(t) = NaiveTime::from_hms_opt(23, 59, 59) {
            time = t;
        } else {
            return Err("Invalid time format");
        }
    }
    let datetime = NaiveDateTime::new(date, time);
    let datetime_utc = DateTime::from_utc(datetime, Utc);
    Ok(datetime_utc)
}
fn is_valid_date_format(input: &str) -> bool {
    if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        // 日期解析成功
        let formatted = date.format("%Y-%m-%d").to_string();
        return formatted == input;
    }
    false
}

pub fn get_commits(commit_from: Option<String>, commit_to: Option<String>, commits: &[String]) -> Vec<String> {
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

pub fn get_all_commits(repo: &Repository) -> Result<Vec<String>, git2::Error> {
    // let repo = Repository::open(repo_path)?;
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
pub fn handle_commit_range(repo: Repository, commit_from: Option<String>, commit_to: Option<String>) -> Result<(), git2::Error> {
    let all_commits = {
        let repo_ref = &repo;
        match get_all_commits(repo_ref) {
            Ok(all_commits) => all_commits,
            Err(e) => {
                eprintln!("获取提交列表失败：{}", e);
                return Ok(());
            }
        }
    };

    let results = get_commits(commit_from, commit_to, &all_commits);
    let commit_ids: Vec<&str> = results.iter().map(|s| s.as_str()).collect();
    handle_multiple_commits(repo, &commit_ids)
}
fn handle_uncommitted_files(repo: Repository)-> Result<(), git2::Error> {

    let statuses = repo.statuses(None).unwrap();

    let mut uncommitted_files = Vec::new();

    for entry in statuses.iter() {
        if entry.status().is_index_modified() || entry.status().is_wt_modified() {
            if let Some(path) = entry.path() {
                uncommitted_files.push(path.to_owned());
            }
        }
    }

    // println!("Uncommitted files:P{:?}",uncommitted_files);
    for file in uncommitted_files {
        println!("{:?}", file);
    }
    // 获取仓库状态
    // let mut options = StatusOptions::new();
    // options.include_untracked(true);

    // let statuses = repo.statuses(Some(&mut options)).expect("Failed to get statuses");

    // // 遍历未提交的文件
    // for entry in statuses.iter() {
       
    //     let status = entry.status();
    //     println!("111111");
    //     if status.is_wt_new() || status.is_wt_modified() {
    //         println!("2222");
    //         let path = entry.path().expect("Failed to get path");
    //         println!("Uncommitted file: {}", path);

    //         // 读取文件内容
    //         let mut file = File::open(path).expect("Failed to open file");
    //         let mut contents = String::new();
    //         file.read_to_string(&mut contents).expect("Failed to read file");

    //         println!("File content:\n{}", contents);

    //     }
    // }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
