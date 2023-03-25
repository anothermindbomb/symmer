use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::fs::File;
use std::fs::{create_dir_all, remove_file};
use std::io::prelude::*;
use std::io::BufReader;
use std::os::windows::fs::symlink_file;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Some code to experiment with creating symlinks.

    let args: Vec<String> = env::args().collect();
    let driverfilename: &str = &args[1]; // We expect one argument - the full name of the driver file.

    let driverhandle = File::open(driverfilename)?;
    let fakereader = BufReader::new(&driverhandle);
    let mut line_cnt = 0;

    // Determine how many lines we're going to process by simply counting them all. We don't store them, as
    // there may be many millions - 15 GB text files are not unheard of.
    // There *must* be a better way to do this.
    for _ in fakereader.lines() {
        line_cnt += 1;
    }

    let bar = ProgressBar::new(line_cnt);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {per_sec} {wide_bar} {pos:>7}/{len:7} ETA: {eta} {msg}",
        )
        .unwrap(),
    );

    // We use a file of legitimate filenames and break them up into path and filename, then use the file type to determine what
    // sample we're going to symlink to.
    // let driverfilename = r"C:\Users\Steve\Desktop\Swordfish\swordfishlivedocs.txt";
    // let driverhandle = File::open(driverfilename)?;
    let reader = BufReader::new(&driverhandle);

    for line in reader.lines() {
        let line = line.unwrap();
        let fullpath = Path::new(&line);
        let basepath = fullpath.parent().unwrap().to_str().unwrap();
        let extension = fullpath.extension().unwrap().to_str().unwrap();

        // So we know the path, the filename and the extension we're dealing with.
        // Create the path if it doesn't already exist.
        create_dir_all(basepath)?;

        // Build our target filename up. Raw string to help with the slashes, and we need to own the result.
        let mut targetfile = r#"T:\\SampleDocs\SampleDocument."#.to_owned();
        targetfile += extension; // append the correct file extension.

        if fullpath.exists() {
            // println!("Deleting {}", fullpath.display());
            remove_file(&line)?; // remove the file if it already exists.
        }
        symlink_file(targetfile, line).ok(); // link the real filename to the sampledocument.
        bar.inc(1);
    }
    bar.finish();
    Ok(())
}
