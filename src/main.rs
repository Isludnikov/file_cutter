use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Instant;

const SIZE_LIMIT: u64 = 70 * 1024 * 1024;
fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("No command line parameter!");
        println!("Usage: file_cutter.exe file_to_cut.csv");
        exit(1);
    }
    let start = Instant::now();
    let path = Path::new(&args[1]);
    let mut lines = 0_u64;
    let mut size = 0_u64;
    let mut total_size = 0_u64;
    let mut part = 1_u64;
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut writer = BufWriter::new(File::create(append_postfix_to_file_name(path, "_part1"))?);
    for line in reader.lines() {
        lines += 1;
        let line = line?;
        let line_size = line.as_bytes().len() as u64;
        size += line_size;
        total_size += line_size;
        writeln!(writer, "{}", line)?;
        if size >= SIZE_LIMIT {
            part += 1;
            size = 0;
            writer.flush()?;
            let name = format!("_part{}", part);
            writer = BufWriter::new(File::create(append_postfix_to_file_name(path, &name))?);
        }
    }
    writer.flush()?;
    let duration = start.elapsed();
    println!("Total parts - {}", part);
    println!("Total lines - {}", lines);
    println!("Total size(bytes) - {}", total_size);
    println!(
        "Average line size(bytes) - {}",
        (total_size as f64) / (lines as f64)
    );
    println!("Time elapsed - {:?}", duration);
    println!("Task completed!");
    Ok(())
}
fn append_postfix_to_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let old_name = path.file_stem().unwrap().to_str().unwrap();
    let mut result = path.to_owned();
    let new_name = format!("{}{}", old_name, name);
    result.set_file_name(new_name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}
