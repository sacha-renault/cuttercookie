use clap::Parser;

#[derive(Parser)]
#[command(name = "cuttercookie")]
#[command(about = "File system utility tools", long_about = None)]
pub struct Cli {
    /// Directory path to start from
    pub path: String,

    /// Directory that will not be included
    #[arg(long, short, value_delimiter = ',')]
    pub excluded_items: Vec<String>,

    /// flag that exclude the main project directory
    #[arg(long, short)]
    pub no_root: bool
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;

    /// Tests basic path argument parsing without excluded items
    #[test]
    fn test_basic_path_parsing() {
        let args = Cli::parse_from(["cuttercookie", "/test/path"]);
        assert_eq!(args.path, "/test/path");
        assert!(args.excluded_items.is_empty());
    }

    /// Tests parsing of multiple excluded items with the long flag
    #[test]
    fn test_multiple_excluded_items_long() {
        let args = Cli::parse_from([
            "cuttercookie",
            "/test/path",
            "--excluded-items",
            "node_modules,target,.git"
        ]);

        assert_eq!(args.path, "/test/path");
        assert_eq!(
            args.excluded_items,
            vec!["node_modules", "target", ".git"]
        );
    }

    /// Tests parsing of multiple excluded items with the short flag
    #[test]
    fn test_multiple_excluded_items_short() {
        let args = Cli::parse_from([
            "cuttercookie",
            "/test/path",
            "-e",
            "node_modules,target,.git"
        ]);

        assert_eq!(args.path, "/test/path");
        assert_eq!(
            args.excluded_items,
            vec!["node_modules", "target", ".git"]
        );
    }

    /// Tests that path argument is required
    #[test]
    fn test_missing_path() {
        let result = Cli::try_parse_from(["cuttercookie"]);

        match result {
            Ok(_) => panic!("Expected error for missing path argument"),
            Err(err) => {
                assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
            }
        }
    }

    /// Tests parsing with empty excluded items list
    #[test]
    #[should_panic]
    fn test_empty_excluded_items() {
        let args = Cli::parse_from([
            "cuttercookie",
            "/test/path",
            "--excluded-items",
            ""
        ]);

        assert_eq!(args.path, "/test/path");
        assert!(args.excluded_items.is_empty());
    }

    /// Tests handling of spaces in path argument
    #[test]
    fn test_path_with_spaces() {
        let args = Cli::parse_from(["cuttercookie", "/test/path with spaces"]);
        assert_eq!(args.path, "/test/path with spaces");
    }

    /// Tests handling of spaces in excluded items
    #[test]
    fn test_excluded_items_with_spaces() {
        let args = Cli::parse_from([
            "cuttercookie",
            "/test/path",
            "--excluded-items",
            "node modules,target files,.git data"
        ]);

        assert_eq!(args.path, "/test/path");
        assert_eq!(
            args.excluded_items,
            vec!["node modules", "target files", ".git data"]
        );
    }
}