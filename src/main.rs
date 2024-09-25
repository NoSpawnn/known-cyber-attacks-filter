use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter},
};

fn main() -> std::io::Result<()> {
    let attacks_folder = "./known-cyber-attacks";
    let ignored = HashSet::from(["Template", ".git"]);

    let readme_paths = fs::read_dir(attacks_folder)?.filter_map(|entry| {
        entry.ok().and_then(|entry| {
            let t = entry.file_type().unwrap();

            if !t.is_dir() || ignored.contains(entry.file_name().to_str()?) {
                None
            } else {
                Some(entry.path().join("README.md"))
            }
        })
    });

    let mut titles = <Vec<String>>::new();
    let out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("Compile.md")?;
    let mut writer = BufWriter::new(out_file);
    readme_paths.for_each(|readme_path| {
        let title = &readme_path
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .split_once(' ')
            .unwrap_or(("", ""))
            .1;
        titles.push(title.to_string());

        if let Ok(file) = File::open(&readme_path) {
            let mut reader = BufReader::new(file);
            let _ = io::copy(&mut reader, &mut writer);
        }
    });

    Ok(())
}
