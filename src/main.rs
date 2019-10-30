// Intended to be a drop-in replacement for
// https://github.com/aidenlab/juicer/blob/encode/CPU/common/fragment.pl

// Currently the IO works, but the binary search results are completely wrong?

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::fs::File;
use std::collections::HashMap;
use structopt::StructOpt;

// Adapted from Rust CLI tutorial
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    outfile: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    site_file: std::path::PathBuf,
}

fn bsearch(target_value: i64, positions: &Vec<i64>) -> i64 {
    // Assumes that the positions are already sorted
    let mut lower_index: i64 = 0;
    let mut upper_index = positions.len() as i64 - 1;
    let mut current_index;
    while lower_index <= upper_index {
        current_index = (lower_index + upper_index) / 2;
        let current_value = positions[current_index as usize];
        if current_value < target_value {
            lower_index = current_index + 1;
        } else if current_value > target_value {
            upper_index = current_index - 1;
        } else {
            return current_index + 1;
        }
    }
    return lower_index;
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let site_file = File::open(&args.site_file)?;
    let site_file_reader = BufReader::new(site_file);
    let mut sites_by_chr = HashMap::new();

    for line in site_file_reader.lines() {
        // https://users.rust-lang.org/t/borrowed-value-does-not-live-long-enough/7225/2
        let mut split_line  = line.as_ref().unwrap().split_whitespace();
        let chr = split_line.next().unwrap();
        let values: Vec<i64> = split_line.map(|x| x.parse::<i64>().unwrap()).collect();
        if chr == "14" {
            sites_by_chr.insert(
                format!("{}{}", chr, "m"),
                // Can't use values twice, so need to copy
                values.clone(),
            );
            sites_by_chr.insert(
                format!("{}{}", chr, "p"),
                values,
            );
        } else {
            sites_by_chr.insert(
                chr.to_string(),
                values,
            );
        }
    }

    let infile = File::open(&args.infile)?;
    let infile_reader = BufReader::new(infile);
    let outfile = File::create(&args.outfile)?;
    let mut outfile_writer = BufWriter::new(outfile);

    for line in infile_reader.lines() {
        let split_line = line.as_ref().unwrap().split_whitespace().collect::<Vec<_>>();

        let index1 = bsearch(
            split_line[2].parse::<i64>().unwrap(),
            &sites_by_chr[split_line[1]],
        );
        let index2 = bsearch(
            split_line[5].parse::<i64>().unwrap(),
            &sites_by_chr[split_line[4]],
        );
        outfile_writer.write(
            format!(
                "{} {} {} {} {} \n",
                &split_line[0..3].join(" "),
                index1,
                &split_line[3..6].join(" "),
                index2,
                &split_line[6..].join(" "),
            ).as_bytes()
        )?;
    }

    Ok(())
}
