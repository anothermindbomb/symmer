use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
// use std::path::*;

fn main() -> std::io::Result<()> {
    // Some code to experiment with creating symlinks.

    // We use a file of legitimate filenames and break them up into path and filename, then use the file type to determine what
    // sample we're going to symlink to.

    let driverfilename = "J:\\SwordfishFilelist.txt".to_string();

    let driverhandle = File::open(driverfilename)?;
    let reader = BufReader::new(driverhandle);

    for line in reader.lines() {
        let file_extension: &String = &line.unwrap().split(".").collect();
        dbg!(file_extension);
    }
    Ok(())
}
