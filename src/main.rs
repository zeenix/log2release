use std::{env::args, process::Command};

use regex::Regex;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        println!("Usage: {} REPO_DIR SUB_PROJECT", args[0]);

        return;
    }
    let local_repo = &args[1];
    let subproject = &args[2];
    let subproject_prefix = format!("{}-", subproject);

    // Get the tags from the local repo for the subproject
    let tags_list_cmd_out = Command::new("git")
        .args(&["tag", "--list", "--sort=version:refname"])
        .current_dir(local_repo)
        .output()
        .expect("failed to execute process")
        .stdout;
    let tags_list = String::from_utf8(tags_list_cmd_out).unwrap();
    let mut tags_list: Vec<&str> = tags_list
        .split_whitespace()
        .filter(|tag| tag.starts_with(&subproject_prefix))
        .collect();
    tags_list.reverse();
    let last_release = tags_list[0];
    let range = format!("{}..HEAD", last_release);

    // list all commit hashes since the last release
    let git_log_cmd_out = Command::new("git")
        .args(&[
            "log",
            "--no-color",
            // <subject> <dot> <space> <body> END
            "--pretty=format:%s. %bEND",
            &range,
            &subproject,
        ])
        .current_dir(local_repo)
        .output()
        .expect("failed to execute process")
        .stdout;
    let git_log = String::from_utf8(git_log_cmd_out).unwrap();
    let internal_regex = Regex::new(r"internal.*:").unwrap();
    let prefix_regex = Regex::new(r"(?m)^.*:").unwrap();
    let quote_regex = Regex::new(r"(?m)^>.*$").unwrap();
    let multispace_regex = Regex::new(r"  +").unwrap();
    let logs = git_log.as_str().split("END").filter_map(|s| {
        if s.is_empty() || internal_regex.is_match(s) {
            return None;
        }
        let s = prefix_regex.replace_all(s, "");
        let s = quote_regex.replace_all(&*s, "");
        let s = s.trim().replace("\n", " ");
        let s = multispace_regex.replace_all(&s, " ");

        Some(s.to_string())
    });
    for log in logs {
        println!("* {}", log);
    }
}
