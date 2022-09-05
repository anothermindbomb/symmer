use std::fs::create_dir_all;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::os::windows::fs::symlink_file;
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
        let fullpath = Path::new(&line);
        let basepath = fullpath.parent().unwrap().to_str().unwrap();
        let _filename = fullpath.file_name().unwrap().to_str().unwrap();
        let extension = fullpath.extension().unwrap().to_str().unwrap();
        // dbg!(basepath, filename, extension);

        // So we know the path, the filename and the extension we're dealing with.
        // Create the path if it doesn't already exist.
        create_dir_all(&basepath)?;
        let mut targetfile = r#"J:\\SampleDocs\SampleDocument."#.to_owned();
        targetfile += extension;
        // dbg!(filename, targetfile);
        symlink_file(fullpath, targetfile)?;
    }
    Ok(())
}
