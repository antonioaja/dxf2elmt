extern crate tempfile;

use anyhow::Context;
use std::fs::File;
use std::io::*;
use tempfile::tempfile;

pub fn create_file(verbose_output: bool, info: bool, file_name: &str) -> File {
    let mut out_file = tempfile().context("Could not create temporary file");
    if !verbose_output {
        out_file = File::create(format!("{}.elmt", &file_name[0..file_name.len() - 4]))
            .context("Could not create output file");
        println!(
            "{}.elmt was created... \nNow converting {}...",
            &file_name[0..file_name.len() - 4],
            file_name
        );
    }

    return out_file.context("Could not return output file").unwrap();
}

pub fn verbose_print(mut out_file: std::fs::File) -> File {
    out_file
        .seek(SeekFrom::Start(0))
        .context("Could not find beginning of output file")
        .unwrap();
    let mut v_contents = String::new();
    out_file
        .read_to_string(&mut v_contents)
        .context("Could not read output file to a string")
        .unwrap();
    print!("{}", v_contents);

    return out_file;
}
