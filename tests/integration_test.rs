#[test]
fn test_it_returns_line_count() {
    let test_file = std::path::Path::new("tests/count_me.rs");
    assert_eq!(
        rusty_lines::run(test_file),
        format!("{}\t3", test_file.display())
    )
}

#[test]
fn test_it_errors_for_wrong_file_extension() {
    let test_file = std::path::Path::new("tests/test_text_file.txt");
    assert_eq!(
        rusty_lines::run(test_file),
        format!("'{}' is not a Rust file!", test_file.display())
    )
}
