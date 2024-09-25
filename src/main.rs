use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter},
};

fn main() -> std::io::Result<()> {
    const FOLDER: &str = "./known-cyber-attacks";
    let ignored = HashSet::from(["README.md", "Template", ".git"]);

    let out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("Compile.md")?;
    let mut writer = BufWriter::new(out_file);
    let paths = fs::read_dir(FOLDER)?.filter_map(|entry| {
        entry.ok().and_then(
            |entry| match ignored.contains(entry.file_name().to_str()?) {
                true => None,
                false => Some(entry.path().join("README.md")),
            },
        )
    });

    paths.for_each(|readme_path| {
        if let Ok(file) = File::open(&readme_path) {
            let mut reader = BufReader::new(file);
            let _ = io::copy(&mut reader, &mut writer);
        }
    });

    Ok(())
}
