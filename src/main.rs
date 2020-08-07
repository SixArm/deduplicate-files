use std::collections::{HashMap,HashSet};
use std::{env,fs};
use std::fs::File;
use walkdir::{WalkDir,DirEntry};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut len_to_entries: HashMap<u64, Vec<DirEntry>> = HashMap::new();
    println!("{}", args[1]);
    for entry in WalkDir::new(&args[1])
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file()) {
        let metadata = entry.metadata().expect("metadata");
        let len = metadata.len();
        println!("{} len:{}", entry.path().display(), metadata.len());
        len_to_entries.entry(len).or_insert(Vec::<DirEntry>::new()).push(entry);    
    }
    
    // for entry in fs::read_dir(current_dir)? {
    //     let entry = entry?;
    //     let path = entry.path();

    //     let metadata = fs::metadata(&path)?;
    //     let last_modified = metadata.modified()?.elapsed()?.as_secs();

    //     if last_modified < 24 * 3600 && metadata.is_file() {
    //         println!(
    //             "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
    //             last_modified,
    //             metadata.permissions().readonly(),
    //             metadata.len(),
    //             path.file_name().ok_or("No filename")?
    //         );
    //     }
    // }
}
