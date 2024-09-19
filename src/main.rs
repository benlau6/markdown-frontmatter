use markdown_frontmatter::frontmatter::{
    add_frontmatter, delete_frontmatter, find_frontmatter, has_frontmatter,
};
use std::{fs, path::PathBuf};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let folder: PathBuf = args.get(1).expect("No folder provided").into();
    let allow_deletion = args.get(2).map_or(false, |arg| arg == "--delete");

    if !folder.is_dir() {
        panic!("Not a directory");
    }
    println!("Adding frontmatter to markdown files in {:?}", &folder);

    add_frontmatter_to_md(folder, allow_deletion)
}

fn add_frontmatter_to_md(folder: PathBuf, allow_deletion: bool) -> std::io::Result<()> {
    for entry in fs::read_dir(folder)? {
        let dir = entry?;
        let path = dir.path();

        if path.extension().unwrap_or_default() != "md" {
            continue;
        }
        let metadata = fs::metadata(&path)?;

        if metadata.file_type().is_dir() {
            add_frontmatter_to_md(path, allow_deletion)?;
            continue;
        }
        if metadata.file_type().is_symlink() {
            continue;
        }

        // cannot process a file that needs to be processed, so break the program
        assert!(!metadata.permissions().readonly());

        let mut content = fs::read_to_string(&path)?;
        if has_frontmatter(&content) {
            if allow_deletion {
                content = delete_frontmatter(&content);
                println!("Deleted frontmatter");
            } else {
                println!("Frontmatter already exists in {:?}", &path);
                continue;
            }
        }

        let file_name = path
            .file_name()
            .expect("Not a filename")
            .to_str()
            .expect("Not a valid UTF-8 string");

        let frontmatter = find_frontmatter(&content, metadata)?;

        println!("Adding frontmatter\n{}to {}", frontmatter, file_name);
        fs::write(&path, add_frontmatter(&content, frontmatter, false))?;
    }
    Ok(())
}
