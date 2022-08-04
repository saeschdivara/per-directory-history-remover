use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use std::env;

use dirs;

fn remove_lines_by_search_term(file_path: PathBuf, search_term: &str) {
    let content = match fs::read_to_string(&file_path) {
        Err(e) => panic!("Could not read file: {}", e),
        Ok(result) => result
    };

    let splitted_lines: Vec<&str> = content
        .split_terminator("\n")
        .filter(|line| { !line.contains(search_term) })
        .collect();

    match fs::write(&file_path, splitted_lines.join("\n")) {
        Err(e) => panic!("Could not write file: {}", e),
        Ok(_) => {}
    };
}

fn recursive_history_update(current_directory: PathBuf, search_term: &str) {
    println!("{:?}", current_directory);

    let directory_list = match fs::read_dir(current_directory) {
        Err(e) => panic!("Could not read directory: {}", e),
        Ok(r) => r
    };

    for c in directory_list {
        if let Ok(entry) = c {
            if let Ok(metadata) = fs::metadata(entry.path()) {
                if metadata.is_dir() {
                    recursive_history_update(entry.path(), search_term);
                } else {
                    println!("File: {:?}", entry);
                    remove_lines_by_search_term(entry.path(), search_term)
                }
            }
        }
    }
}

fn main() {
    let now = Instant::now();

    let home_dir = match dirs::home_dir() {
        None => panic!("Home dir cannot be found"),
        Some(result) => result
    };

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Missing search term argument")
    }

    let search_term = args[1].as_str();
    recursive_history_update(home_dir.join(".directory_history"), search_term);

    let elapsed = now.elapsed();
    println!("Execution took: {:.2?}", elapsed);
}
