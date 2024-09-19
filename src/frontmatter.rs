use chrono::NaiveDate;
use chrono::{DateTime, Utc};
use std::fmt::Display;
use std::fs::Metadata;

pub struct Frontmatter {
    title: String,
    created: NaiveDate,
    modified: NaiveDate,
}

impl Frontmatter {
    fn without_modified(&self) -> String {
        format!(
            "---\ntitle: {}\npublishDate: {}\n---\n",
            self.title, self.created
        )
    }
}

impl Display for Frontmatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "---\ntitle: {}\npublishDate: {}\nupdatedDate: {}\n---\n",
            self.title, self.created, self.modified
        )
    }
}

pub fn has_frontmatter(content: &str) -> bool {
    content.starts_with("---")
}

pub fn delete_frontmatter(content: &str) -> String {
    let title_line = find_line_of_header(content);
    let lines = content.lines().skip(title_line);
    lines.collect::<Vec<&str>>().join("\n")
}

pub fn add_frontmatter(content: &str, frontmatter: Frontmatter, with_modified: bool) -> String {
    match with_modified {
        true => format!("{}\n{}", frontmatter, content),
        false => format!("{}\n{}", frontmatter.without_modified(), content),
    }
}

pub fn find_frontmatter(content: &str, metadata: Metadata) -> std::io::Result<Frontmatter> {
    let modified: DateTime<Utc> = metadata
        .modified()
        .expect("Modified date not supported on current platform")
        .into();

    let created: DateTime<Utc> = metadata
        .created()
        .expect("Created date not supported on current platform")
        .into();

    Ok(Frontmatter {
        title: parse_title(content),
        created: created.date_naive(),
        modified: modified.date_naive(),
    })
}

fn find_line_of_header(content: &str) -> usize {
    let mut line_idx = 0;
    for line in content.lines() {
        if line.starts_with("# ") {
            break;
        }
        line_idx += 1;
    }
    line_idx
}

fn parse_title(content: &str) -> String {
    let title_line = find_line_of_header(content);
    let title_str = content.lines().nth(title_line).unwrap();
    let title = title_str.trim_start_matches("# ").to_string();
    title
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_frontmatter() {
        let content = "---\ntitle: Hello\ncreated: 2021-01-01\nmodified: 2021-01-01\n---\n";
        assert!(has_frontmatter(content));
    }

    #[test]
    fn test_delete_frontmatter() {
        let content =
            "---\ntitle: Hello\ncreated: 2021-01-01\nmodified: 2021-01-01\n---\n# Hello, World!";
        assert_eq!(delete_frontmatter(content), "# Hello, World!");
    }

    #[test]
    fn test_add_frontmatter_with_modified() {
        let content = "Hello, World!";
        let frontmatter = Frontmatter {
            title: "Hello".to_string(),
            created: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            modified: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        };
        let expected =
            "---\ntitle: Hello\npublishDate: 2021-01-01\nupdatedDate: 2021-01-01\n---\n\nHello, World!";
        assert_eq!(add_frontmatter(content, frontmatter, true), expected);
    }

    #[test]
    fn test_add_frontmatter_without_modified() {
        let content = "Hello, World!";
        let frontmatter = Frontmatter {
            title: "Hello".to_string(),
            created: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            modified: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        };
        let expected = "---\ntitle: Hello\npublishDate: 2021-01-01\n---\n\nHello, World!";
        assert_eq!(add_frontmatter(content, frontmatter, false), expected);
    }

    #[test]
    fn test_find_line_of_header() {
        let content = "# Hello World!";
        assert_eq!(find_line_of_header(content), 0);
    }

    #[test]
    fn test_parse_title() {
        let content = "# Hello World!";
        assert_eq!(parse_title(content), "Hello World!");
    }
}
