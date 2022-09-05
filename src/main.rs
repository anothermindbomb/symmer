use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Some code to experiment with creating symlinks.

    // We use a file of legitimate filenames and break them up into path and filename, then use the file type to determine what
    // sample we're going to symlink to.

    let driverfilename = r"C:\Users\Steve\Desktop\Swordfish\swordfishlivedocs.txt";
    let driverhandle = File::open(driverfilename)?;
    let reader = BufReader::new(driverhandle);
    for line in reader.lines() {
        let line = line.unwrap();
        let path = Path::new(&line);
        let extension = path.extension();
        println!("{:?}, {:?}", path, extension);
    }

    Ok(())
}
