
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

/// u8 integerers 0 - 255
/// usize represent the full range of memory addresses and array indices on a particular architecture
/// 
/// OWNERSHIP AND BORROWING in Rust
/// & for borrowing

fn string_to_bytes(s: &str){
    let bytes = s.as_bytes():
    return bytes
}

fn get_mean(s: &[u8]) -> f64{
    let values: Vec<f64> = s.iter().map(|&x| f64::from(x)).collect();
    let sum = values.iter().sum::<f64>() as f64;
    let mean = sum / (values.len() as f64);
    return mean;
}

fn get_sum(s: &[u8]) -> f64{
    let values: Vec<f64> = s.iter().map(|&x| f64::from(x)).collect();
    let sum = values.iter().sum::<f64>() as f64;
    return sum;
}


/// & is for borrowing a variable
fn trim_read(seqline: &str, end3: &u8, end5: &u8) -> String{
    /// * is used to dereference
    let (_, right) = seqline.split_at(*end5 as usize);
    
    let mut trimed_line = String::from("");
    
    if *end3 != 0 {
        let (trimmed, _) = right.split_at(right.len() - *end3 as usize);
        trimed_line = String::from(trimmed);
    } else {
        trimed_line = String::from(right);
    }
    
    return trimed_line;        
}

fn trim_line_by_line(line_list: [&str; 4], end3:fffg u8, end5: u8) {
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

/// detect of phred scale
fn detect_quality(qual_arr: &[u8]) ->  Result<usize, &'static str> {
    let sum = get_sum(*qual_arr);
    let max = *qual_arr.iter().max().unwrap();
    let phred_scale: usize;
    
    if sum/qual_arr.len() < 75 {
        phred_scale = 33;
    }
    else if qual_arr >= 75{
        phred_scale = 64;
    }
    else {
        return Err("Error detecting phred scale.");
    }
    
    return phred_scale;
}

fn trim_quality(seqline: &str, qual_arr: &str, phred: &u8){
    unimplemented!()
}

fn filter_quality(qual_arr: &[u8] , quality: &u8, phred: &u8) -> bool{
    let phred_ascii = *quality + *phred;
    let pass = false;
    if s.len() != 0 {
        let mean = get_mean(&*s);
        if mean > phred_ascii as f64{
            let pass = true;
        }
    }  
    return pass;
}

fn filter_bases_length(seqline: &str, n_bases: &u8, threshold_reads: &u8) -> bool{ 
    let unknown_character = 'n';
    let count = seqline.chars().filter(|&c| c == unknown_character).count();
    let mut pass = false;

    if count < *n_bases as usize && seqline.len() > *threshold_reads as usize {
        let pass = true;
    }
    return pass;
}
  

fn write_outputfile(file_list:, outputfile:impl AsRef<Path>, qual: &u8. phred:&u8. n_bases: &u8. length: &u8){
    unimplemented!() 
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