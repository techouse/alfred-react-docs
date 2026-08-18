use super::*;

#[test]
fn query_option_accepts_short_form() -> Result<()> {
    let cli = Cli::parse(["-q".to_owned(), "state hooks".to_owned()])?;

    assert_eq!(cli.query, "state hooks");
    Ok(())
}

#[test]
fn query_option_accepts_long_equals_form() -> Result<()> {
    let cli = Cli::parse(["--query=state hooks".to_owned()])?;

    assert_eq!(cli.query, "state hooks");
    Ok(())
}

#[test]
fn flags_are_parsed() -> Result<()> {
    let cli = Cli::parse(["--verbose".to_owned(), "--update".to_owned()])?;

    assert_eq!((cli.verbose, cli.update), (true, true));
    Ok(())
}

#[test]
fn query_is_normalized_like_the_dart_workflow() {
    let cli = Cli {
        query: "  State \t HOOKS\n ".to_owned(),
        ..Cli::default()
    };

    assert_eq!(cli.normalized_query(), "state hooks");
}

#[test]
fn missing_query_value_is_rejected() {
    let error = Cli::parse(["--query".to_owned()]).expect_err("missing value must fail");

    assert_eq!(error.to_string(), "--query requires a value");
}

#[test]
fn unknown_argument_is_rejected() {
    let error = Cli::parse(["--unknown".to_owned()]).expect_err("unknown argument must fail");

    assert_eq!(error.to_string(), "unknown argument: --unknown");
}
