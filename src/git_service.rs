#![warn(deprecated)]
extern crate chrono;
extern crate git2;
use crate::*;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use clap::Parser;
use git2::BranchType;
use git2::{Error, Repository};
use std::collections::HashSet;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

pub fn start_git() {
    let args = Config::parse();
    let _excluded_commits: Vec<git2::Oid> = vec![];
    let repo = match load_repository(&args.repo) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Fail to open the repository {}", e);
            process::exit(0);
        }
    };
    match (
        &args.commit,
        &args.commits,
        &args.commits_file,
        &args.commit_since,
        &args.commit_until,
        &args.commit_from,
        &args.commit_to,
        args.uncommitted,
    ) {
        (Some(commit), _, _, _, _, _, _, _) => {
            if let Err(e) = handle_single_commit(repo, commit) {
                eprintln!("Application error: {}", e);
                process::exit(0);
            }
        }
        (_, Some(commits), _, _, _, _, _, _) => {
            let commit_ids: Vec<&str> = commits.split(',').collect();
            if let Err(e) = handle_multiple_commits(repo, &commit_ids) {
                eprintln!("Application error: {}", e);
                process::exit(0);
            }
        }
        (_, _, Some(file_path), _, _, _, _, _) => {
            if let Err(e) = handle_commits_file(repo, file_path) {
                eprintln!("Application error: {}", e);
                process::exit(0);
            }
        }
        (_, _, _, Some(since), Some(until), _, _, _) => {
            if let Err(e) = handle_commit_range_by_time(repo, since, until) {
                eprintln!("Application error: {}", e);
                process::exit(0);
            }
        }
        (_, _, _, _, _, Some(commit_from), Some(commit_to), _) => {
            if let Err(e) = handle_commit_range(repo, Some(commit_from.clone()), Some(commit_to.clone())) {
                eprintln!("Application error: {}", e);
                process::exit(0);
            }
        }
        (_, _, _, _, _, _, _, true) => {
            if let Err(e) = handle_uncommitted_files(repo) {
                eprintln!("Application error: {}", e);
                process::exit(0);
            }
        }
        (_, _, _, _, _, _, _, false) => {
            // 处理未提供未提交选项的情况
        }
    }
    
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
    
    

    // // // 扫描一个commit的内容
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
    // // // 扫描多个commit
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
    // ////////////////////////////////////////////////////////////////
    //  match (&args.commit_since, &args.commit_until) {
    //     (Some(since), Some(until)) => {
    //         if let Err(e) = handle_commit_range_by_time(repo, since,until) {
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
    //     }
    //     _ => {

    //     }
    // }
    // match args.uncommitted {
    //     true => {
    //         if let Err(e) = handle_uncommitted_files(repo) {
    //             eprintln!("Application error: {}", e);
    //             process::exit(0);
    //         }
    //     }
    //     false => {
    //         // 处理未提供未提交选项的情况
    //     }
    // }
    

}
// 扫描一个commit的内容
pub fn handle_single_commit(repo: Repository, commit_id: &str) -> Result<(), git2::Error> {
    let commit = repo.find_commit(git2::Oid::from_str(commit_id)?)?;
    let commit_info = config_commit_info(&repo, &commit)?;

    let commits_list = vec![commit_info];

    
    handle_commit_info(&commits_list);
    Ok(())
}
// 扫描多个commit的内容
pub fn handle_multiple_commits(repo: Repository, commit_ids: &[&str]) -> Result<(), git2::Error> {
    let mut commits_list=vec![];
   
    for commit_id in commit_ids {
        let commit = repo.find_commit(git2::Oid::from_str(commit_id)?)?;
        let commit_info = config_commit_info(&repo, &commit)?;
        commits_list.push(commit_info);
    }
   
    handle_commit_info(&commits_list);
    Ok(())
}
// 扫描commit文件
pub fn handle_commits_file(repo: Repository, file_name: &str)-> Result<(), git2::Error> {
    let file = fs::File::open(file_name).expect("Failed to open commits file");
    let reader = BufReader::new(file);
    let mut commits: Vec<String> = Vec::new();

    for line in reader.lines().flatten() {
        commits.push(line);
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
        let start_time = match parse_start_date_to_datetime(since, "start") {
            Ok(datetime) => datetime.with_timezone(&FixedOffset::east(0)),
            Err(err) => {
                eprintln!("时间格式错误 error: {}", err);
                process::exit(0);
            }
        };

        let end_time = match parse_start_date_to_datetime(until, "until") {
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
        let start_time = DateTime::parse_from_rfc3339(since).unwrap();
        let end_time = DateTime::parse_from_rfc3339(until).unwrap();

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
            let commit_info = config_commit_info(repo, &commit)?;
            commits.push(commit_info);
        }
    }
    handle_commit_info(&commits);
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
            let commit_info = config_commit_info(&repo, &commit)?;
            commits.push(commit_info);
            // commits_ids.push(commit_oid.to_string());
        }
    }
    handle_commit_info(&commits);
    Ok(())
    // Ok(commits)
}

pub fn handle_commit_info(commit_info_list: &[CommitInfo]) {
    println!("Total Commits: {}", commit_info_list.len());
    println!("Commit history:");
    for commit_info in commit_info_list {
        println!("repo: {}", commit_info.repo);
        println!("commit: {}", commit_info.commit);
        println!("author: {}", commit_info.author);
        println!("email: {}", commit_info.email);
        println!("commitMessage: {}", commit_info.commit_message);
        println!("tags: {:?}", commit_info.tags.to_owned());
        println!("Operation: {}", commit_info.operation);
        println!("date: {}", commit_info.date.format("%Y-%m-%dT%H:%M:%S%z"));
        println!("Files:");
        for (file, _content) in &commit_info.files {
            println!("File: {}", file);
            // println!("Content:\n{}", content);
            println!("----------------------");
            }
            println!("======================");
        }
}
pub fn handle_commit_range(repo: Repository, commit_from: Option<String>, commit_to: Option<String>) -> Result<(), git2::Error> {
    let all_commits = {
        let repo_ref = &repo;
        match load_all_commits(repo_ref) {
            Ok(all_commits) => all_commits,
            Err(e) => {
                eprintln!("获取提交列表失败：{}", e);
                return Ok(());
            }
        }
    };

    let results = load_commits_by_conditions(commit_from, commit_to, &all_commits);
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
