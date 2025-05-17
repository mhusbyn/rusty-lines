use clap::Parser;

/// A simple program to count the number of lines in a file.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// The file to count lines in.
    file_path: std::path::PathBuf,
}

impl CliArgs {
    pub fn get_file_path(&self) -> &std::path::Path {
        self.file_path.as_path()
    }
}

fn main() {
    let args = CliArgs::parse();
    println!("{}", rusty_lines::run());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_args_get_file_path() {
        let path = std::path::PathBuf::from("test.txt");
        let args = CliArgs {
            file_path: path.clone(),
        };

        assert_eq!(args.get_file_path(), path.as_path());
    }
}
