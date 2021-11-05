use std::{fs, io};
use std::path::Path;
use clap::{App, Arg};
use std::sync::Mutex;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
pub struct Files {
    depth: i64,
    parent: String,
    name: String,
    hidden: bool,
}

#[derive(Debug)]
pub struct Dirs {
    depth: i64,
    parent: String,
    path: String,
    name: String,
    hidden: bool,
}

lazy_static! {
    static ref FILES: Mutex<Vec<Files>> = Mutex::new(vec![]);
    static ref DIRS: Mutex<Vec<Dirs>> = Mutex::new(vec![]);
}

fn main() {
    let mut base = false;
    let mut recursion = false;
    let mut hidden = false;
    let mut display_files = false;
    let mut limit = "1000";
    let mut max_depth = "1000";
    let mut sort = "ascending";
    let matches = App::new("count-files")
        .arg(
            Arg::with_name("base")
            .help("Base directory")
            .takes_value(true)
            .long("base")
            .short("b")
            )
        .arg(
            Arg::with_name("limit")
            .help("Limit number of results to display")
            .takes_value(true)
            .short("l")
            )
        .arg(
            Arg::with_name("max-depth")
            .help("Maximum directory depth")
            .takes_value(true)
            .long("max-depth")
            .short("m")
            )
        .arg(
            Arg::with_name("recursion")
            .help("Recursively find files")
            .long("recursion")
            .short("r")
        )
        .arg(
            Arg::with_name("include-hidden")
            .help("Include hidden files")
            .long("include-hidden")
            .short("i")
        )
        .arg(
            Arg::with_name("sort")
            .help("Sorting algorithm")
            .takes_value(true)
            .possible_values(&["ascending", "descending", 
                "alphabetical", "reverse-alphabetical"])
            .long("sort")
            .short("s")
            )
        .arg(
            Arg::with_name("display")
            .help("Display files")
            .long("display")
            .short("d")
            )
        .get_matches();

    if matches.is_present("recursion") {
        recursion = true;
    }
    if matches.is_present("include-hidden") {
        hidden = true;
    }
    if matches.is_present("display") {
        display_files = true;
    }
    if matches.is_present("limit") {
        limit = matches.value_of("limit").unwrap();
    }
    if matches.is_present("max-depth") {
        max_depth = matches.value_of("max-depth").unwrap();
    }
    if matches.is_present("sort") {
        sort = matches.value_of("sort").unwrap();
    }
    if matches.is_present("base") {
        if let Some(ref location) = matches.value_of("base") {
            let dir = location;
            scan_dir(0, max_depth.parse::<i64>().unwrap(), dir.to_string(), hidden, recursion)
        }
        base = true;
    }
    if base != true {
        scan_dir(0, max_depth.parse::<i64>().unwrap(), ".".to_string(), hidden, recursion)
    }
    show_results(sort.to_string(), limit.parse::<i64>().unwrap(), hidden, display_files)
}

fn scan_dir(depth: i64, max_depth: i64, base: String, hidden: bool, recursion: bool) {
    if depth <= max_depth {
        if Path::new(&base).exists() {
            for entry_res in fs::read_dir(&base).unwrap() {
                let entry = entry_res.unwrap();
                let file_name_buf = entry.file_name();
                let file_name = file_name_buf.to_str().unwrap();
                if hidden && recursion {
                    if entry.file_type().unwrap().is_dir() {
                        let mut dir = entry.path().display().to_string();
                        dir.push('/');
                        let h = file_name.starts_with(".");
                        let d: Dirs = Dirs{depth: depth, parent: base.to_string(), path: dir.clone(), name: file_name.to_string(), hidden: h};
                        DIRS.lock().unwrap().push(d);
                        if !is_folder_empty(&dir).unwrap() {
                            scan_dir(depth+1, max_depth, dir, true, true)
                        }
                    }
                    else {
                        let h = file_name.starts_with(".");
                        let f: Files = Files{depth: depth, parent: base.to_string(), name: file_name.to_string(), hidden: h};
                        FILES.lock().unwrap().push(f);
                    }
                }
                if hidden && !recursion {
                    if entry.file_type().unwrap().is_dir() {

                    }
                    else {
                        let h = file_name.starts_with(".");
                        let f: Files = Files{depth: depth, parent: base.to_string(), name: file_name.to_string(), hidden: h};
                        FILES.lock().unwrap().push(f);
                    }
                }
                if !hidden && !recursion {
                    if entry.file_type().unwrap().is_file() && !file_name.starts_with(".") {
                        let f: Files = Files{depth: depth, parent: base.to_string(), name: file_name.to_string(), hidden: false};
                        FILES.lock().unwrap().push(f);
                    }
                }
                if !hidden && recursion {
                    if entry.file_type().unwrap().is_dir() {
                        if !file_name.starts_with(".") {
                            let mut dir = entry.path().display().to_string();
                            dir.push('/');
                            let d: Dirs = Dirs{depth: depth, parent: base.to_string(), path: dir.clone(), name: file_name.to_string(), hidden: false};
                            DIRS.lock().unwrap().push(d);
                            if !is_folder_empty(&base).unwrap() {
                                scan_dir(depth+1, max_depth, dir, false, true);
                            }
                        }
                    }
                    else {
                        if !file_name.starts_with(".") {
                            let f: Files = Files{depth: depth, parent: base.to_string(), name: file_name.to_string(), hidden: false};
                            FILES.lock().unwrap().push(f);
                        }
                    }
                }
            }
        }
    }
}

fn is_folder_empty(path: impl AsRef<Path>) -> io::Result<bool> {
    Ok(fs::read_dir(path)?.take(1).count() == 0)
}

fn show_results(sort: String, limit: i64, hidden: bool, display_files: bool) {
    let fl = FILES.lock().unwrap();
    let dl = DIRS.lock().unwrap();
    let mut dircount: HashMap<String, i64> = HashMap::new();
    let mut dircount_hidden: HashMap<String, i64> = HashMap::new();
    let mut filecount_hidden: HashMap<String, i64> = HashMap::new();
    for i in dl.iter() {
        dircount.insert(i.name.to_string(), 0);
        if i.hidden {
            dircount_hidden.insert(i.name.to_string(), 0);
        }
    }
    for i in fl.iter() {
        if i.hidden {
            filecount_hidden.insert(i.name.to_string(), 0);
        }
        for j in dl.iter() {
            if i.parent.as_str() == j.path.as_str() {
                *dircount.entry(j.name.to_string()).or_insert(0) += 1;
                // overwrites if identical name with different path
            }
        }
    }
    let mut sortcount: Vec<_> = dircount.iter().collect();
    if sort == "ascending" {
        sortcount.sort_by(|a, b| b.1.cmp(a.1).reverse());
    }
    if sort == "descending" {
        sortcount.sort_by(|a, b| b.1.cmp(a.1));
    }
    if sort == "alphabetical" {
        sortcount.sort();
    }
    if sort == "reverse-alphabetical" {
        sortcount.sort();
        sortcount.reverse();
    }
    let sortcount_len: i64 = sortcount.len() as i64;
    if sortcount_len > 0 {
        if sortcount_len > limit {
            let end = sortcount_len - limit;
            for _i in 0..end {
                sortcount.remove(0);
            }
        }
        if display_files {
            for i in &sortcount {
                println!("{} {}", i.1, i.0);
            }
        }
    }
    if hidden {
        println!("Total files: {}, of which {} are hidden.\nTotal directories: {}, of which {} are hidden.", fl.len(), filecount_hidden.len(), dl.len(), dircount_hidden.len());
    }
    else {
        println!("Total files: {}\nTotal directories: {}", fl.len(), dl.len());
    }
}