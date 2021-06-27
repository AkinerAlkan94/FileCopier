use walkdir::{WalkDir, DirEntry};
use std::{env, fs};
use fs_extra::file::{copy, CopyOptions};
use std::thread::Thread;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let src = args.get(1).unwrap().to_string();
    let dst = args.get(2).unwrap().to_string();
    println!("Source Folder {}", src);
    println!("Destination Folder {}", dst);
    for mut e in WalkDir::new(src.clone()).into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            copyFile(&src, dst.clone(), &mut e);
        }
    }
}

fn copyFile(src: &String, dst: String, e: &mut DirEntry) {
    let suffix = e.path().to_str().unwrap().replace(src, "");
    let realDst = dst.clone() + &suffix;
    let alterations = realDst.clone();
    let mut dests: Vec<&str> = alterations.split("\\").collect();
    dests.pop().unwrap();
    let realdst = dests.join("\\");
    fs::create_dir_all(realdst.clone()).unwrap();
    let x = e.path().to_str().unwrap();
    let options = CopyOptions::new();

    if(!Path::new(&realDst).exists()){
        let result = copy(x, realDst.clone(), &options);
        if(result.is_err()){
            println!("Problem Occurred on File: {}",realDst);
            std::thread::sleep_ms(1000);
            copyFile(src,dst,e);
        }
        if(result.is_ok()){
            println!("Copied File: {}",realDst);
        }
    }
}
