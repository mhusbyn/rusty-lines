#[test]
fn test_it_returns_line_count() {
    let test_file = std::path::Path::new("tests/test_text_file.txt");
    assert_eq!(
        rusty_lines::run(test_file),
        format!("{}\t1", test_file.display())
    )
}
