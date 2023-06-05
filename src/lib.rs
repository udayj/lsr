use clap::{App, Arg};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
type MyResult<T> = Result<T, Box< dyn Error>>;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    long: bool,
    hidden: bool
}

pub fn get_args() -> MyResult<Config> {

    let matches = App::new("lsr")
                    .version("0.1.0")
                    .author("udayj")
                    .about("Rust ls")
                    .arg(
                        Arg::with_name("paths")
                            .value_name("PATH")
                            .help("Paths to list")
                            .multiple(true)
                            .default_value(".")
                    )
                    .arg(
                        Arg::with_name("long")
                            .short("l")
                            .long("long")
                            .help("Show long listing")
                            .takes_value(false)
                    )
                    .arg(
                        Arg::with_name("all")
                            .short("a")
                            .long("all")
                            .help("Show all files")
                            .takes_value(false)
                    )
                    .get_matches();
    Ok (
        Config {
            paths: matches.values_of_lossy("paths").unwrap(),
            long: matches.is_present("long"),
            hidden: matches.is_present("all")
        }
    )
}

//function that takes a &[String], show_hidden and return MyResult<Vec<PathBuf>> with list of path bufs corresponding to the paths in argument &[String]
//go through list of string, convert each string to PathBuf
// If path is not valid, return error
// else if path is a file add it to the vector to be returned (if its a hidden file i.e starts with "." then add only if show_hidden is true)
// else if path is a directory, read the contents of the directory
// for every entry add it to vector to be returned (for hidden entries i.e. those beginning with "." add only if show_hidden is true)
fn find_files(paths: &[String], show_hidden: bool) -> MyResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    for path in paths {
        let path = PathBuf::from(path);
        if !path.exists() {
            return Err(From::from(format!("{}: No such file or directory", path.display())));
        }
        if path.is_file() {
            if show_hidden || !is_hidden(&path) {
                files.push(path);
            }
        } else if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if show_hidden || !is_hidden(&path) {
                    files.push(path);
                }
            }
        }
    }
    Ok(files)
}

// function to check whether a path:PathBuf represents a hidden dir entry
fn is_hidden(path: &PathBuf) -> bool {
    path.file_name()
        .map(|file_name| file_name.to_string_lossy().starts_with('.'))
        .unwrap_or(false)
}