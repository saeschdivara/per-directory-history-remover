use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;

use dirs;
use encoding::{DecoderTrap, Encoding};
use encoding::all::UTF_8;

fn get_file_as_byte_vec(filename: &PathBuf) -> std::io::Result<Vec<u8>> {
    let mut f = match fs::File::open(&filename) {
        Err(e) => {
            println!("Error during file open: {}", e);
            return Err(e);
        }
        Ok(result) => result
    };
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    Ok(buffer)
}

fn remove_lines_by_search_term(file_path: PathBuf, search_term: &str) {

    // this is pure to support .zsh_history since it contains data which is giving issues to rust
    let data = match get_file_as_byte_vec(&file_path) {
        Err(_) => return,
        Ok(result) => result
    };
    let content = match UTF_8.decode(data.as_slice(), DecoderTrap::Ignore) {
        Err(e) => {
            println!("Could not decode file content as utf-8 string: {}", e);
            return;
        }
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
        Err(e) => {
            println!("Could not read directory: {}", e);
            return;
        }
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

    // replace search term in global history (bash and zsh)
    remove_lines_by_search_term(home_dir.join(".bash_history"), search_term);
    remove_lines_by_search_term(home_dir.join(".zsh_history"), search_term);

    let elapsed = now.elapsed();
    println!("Execution took: {:.2?}", elapsed);
}
