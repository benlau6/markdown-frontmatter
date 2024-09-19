# Markdown Frontmatter

This program is a simple implementation of a markdown frontmatter parser. It reads a markdown file, extracts the frontmatter, and appends the frontmatter to the start of the file.

## Why this program?

Astro is a great static website framework to present projects, blog, and notes. However, Astro uses a markdown parser `remark` to render markdown files, which requires frontmatter to be present in the markdown files for its enhanced functionality. This program helps to add frontmatter to the markdown files.

## Usage

To run the program, use the following command:

Note: It will modify the existing markdown files by appending the frontmatter to the start of the file. It will skip the files that already have frontmatter.

```bash
cargo run <path to folder containing markdown files>
```

To run the program with delete option, use the following command:

Note: Please be careful when using the delete option. It will delete the existing frontmatter

```bash
cargo run <path to folder containing markdown files> --delete
```
