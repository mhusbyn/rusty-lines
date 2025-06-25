mod line_count;

fn create_line_count_str(path: &std::path::Path, line_count: u32) -> String {
    format!("{}\t{line_count}", path.display())
}

fn file_extension_is_valid(path: &std::path::Path) -> bool {
    path.extension().and_then(std::ffi::OsStr::to_str) == Some("rs")
}

pub fn run(path: &std::path::Path) -> String {
    if !file_extension_is_valid(path) {
        return format!("'{}' is not a Rust file!", path.display());
    }
    let file_str = std::fs::read_to_string(path).unwrap();
    let line_count = line_count::get_line_count(file_str.as_str()).unwrap();
    create_line_count_str(path, line_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_create_line_count_str() {
        let path = std::path::Path::new("test.txt");
        let line_count = 10;
        assert_eq!(
            create_line_count_str(path, line_count),
            format!("{}\t10", path.display())
        );
    }

    #[rstest]
    #[case(std::path::Path::new("test.txt"), false)]
    #[case(std::path::Path::new("test.py"), false)]
    #[case(std::path::Path::new("test.java"), false)]
    #[case(std::path::Path::new("test.cpp"), false)]
    #[case(std::path::Path::new("test.rs"), true)]
    #[case(std::path::Path::new("some/long/path/test.txt"), false)]
    #[case(std::path::Path::new("some/long/path/test.rs"), true)]
    #[case(std::path::Path::new("/some/long/absolute/path/test.txt"), false)]
    #[case(std::path::Path::new("/some/long/absolute/path/test.rs"), true)]
    fn test_file_extension_is_valid(#[case] path: &std::path::Path, #[case] expected: bool) {
        assert_eq!(file_extension_is_valid(path), expected);
    }
}
