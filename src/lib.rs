mod line_count;

fn create_line_count_str(path: &std::path::Path, line_count: u32) -> String {
    format!("{}\t{line_count}", path.display())
}

pub fn run(path: &std::path::Path) -> String {
    let file_str = std::fs::read_to_string(path).unwrap();
    let line_count = line_count::get_line_count(file_str.as_str()).unwrap();
    create_line_count_str(path, line_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_line_count_str() {
        let path = std::path::Path::new("test.txt");
        let line_count = 10;
        assert_eq!(
            create_line_count_str(path, line_count),
            format!("{}\t10", path.display())
        );
    }
}
