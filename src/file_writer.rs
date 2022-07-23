extern crate tempfile;

use std::fs::File;
use tempfile::tempfile;
use std::io::*;

pub fn create_file(verbose_output: bool, file_name: &str) -> Result<File> {
    let mut out_file = tempfile().unwrap();
    if !verbose_output {
        out_file = File::create(format!("{}.elmt", &file_name[0..file_name.len() - 4]))?;
        println!(
            "{}.elmt was created... \nNow converting {}...",
            &file_name[0..file_name.len() - 4],
            file_name
        );
    }
    Ok(out_file)
}

pub fn verbose_print(mut out_file: std::fs::File) -> Result<File> {
    out_file.seek(SeekFrom::Start(0))?;
    let mut v_contents = String::new();
    out_file.read_to_string(&mut v_contents)?;
    print!("{}", v_contents);
    Ok(out_file)
}