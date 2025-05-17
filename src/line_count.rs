pub fn get_line_count(contents: &str) -> Result<u32, <u32 as TryFrom<usize>>::Error> {
    contents.lines().count().try_into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_get_line_count_single_line() {
        let contents = "Hello, world!";
        assert_eq!(get_line_count(contents).unwrap(), 1);
    }

    #[rstest]
    #[case(2)]
    #[case(5)]
    #[case(10)]
    fn test_get_line_count_multiple_lines(#[case] num_lines: u32) {
        let contents = std::iter::repeat("Hello, world!")
            .take(num_lines as usize)
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(get_line_count(contents.as_str()).unwrap(), num_lines);
    }
}
