
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#![allow(unused)]
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
/// https://rust-cli.github.io/book/tutorial/cli-args.html
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    /// filename for the input FASTQ file
    path: std::path::PathBuf,
    /// filename for the trimmed FASTQ file
    out: String,
    /// Number of bases trimmed on 3' end
    end3: u8,
    /// Number of bases trimmed on 5' end
    end5: u8,
    /// Specify min quality of reads
    qual: u8,
    /// Minimum length of reads
    length: u8,
    /// Maximum of unknown bases
    nbases: u8,
}

/// detect of phred scale
fn detect_quality() {

}

/// & is for borrowing a variable

fn trim_read(seqline: &String, end3: &u8, end5: &u8) -> String{
    /// * is used to dereference
    let (_, right) = seqline.split_at(*end5 as usize);
    
    let mut trimed_line = String::from("");
    
    if *end3 != 0 {
        let (trimmed, _) = right.split_at(right.len() - 3);
        trimed_line = String::from(trimmed);
    } else {
        trimed_line = String::from(right);
    }
    
    return trimed_line;        
}

fn trim_line_by_line(line_list: [&str; 4], end3: u8, end5: u8) {
    let mut pos_seq: u8 = 1;
    let mut pos_qual: u8 = 3;
    ///let mut trimmed: u8 = 0;

    while pos_qual < line_list.len() as u8 {
        let new = trim_read(&line_list[pos_seq as usize], &end3, &end5);
        println!("{}", new);
        pos_qual += 1;
        pos_seq += 1;
    }
}


^

fn lines_from_file(filename:impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let args = Cli::parse();
    let lines = lines_from_file(&args.path);
    let qual = detect_quality(&lines[101])

}