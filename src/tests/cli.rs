use super::*;

#[test]
fn query_option_accepts_short_form() -> Result<()> {
    let cli = Cli::parse(["-q".to_owned(), "state hooks".to_owned()])?;

    assert_eq!(cli.query, "state hooks");
    Ok(())
}

#[test]
fn query_option_accepts_attached_short_value() -> Result<()> {
    let cli = Cli::parse(["-qstate hooks".to_owned()])?;

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
fn collapsed_short_flags_are_parsed() -> Result<()> {
    let cli = Cli::parse(["-vu".to_owned()])?;

    assert_eq!((cli.verbose, cli.update), (true, true));
    Ok(())
}

#[test]
fn collapsed_flags_accept_attached_query() -> Result<()> {
    let cli = Cli::parse(["-vqstate hooks".to_owned()])?;

    assert_eq!((cli.verbose, cli.query.as_str()), (true, "state hooks"));
    Ok(())
}

#[test]
fn collapsed_flags_accept_separated_query() -> Result<()> {
    let cli = Cli::parse(["-vq".to_owned(), "state hooks".to_owned()])?;

    assert_eq!((cli.verbose, cli.query.as_str()), (true, "state hooks"));
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
fn missing_short_query_value_is_rejected() {
    let error = Cli::parse(["-q".to_owned()]).expect_err("missing value must fail");

    assert_eq!(error.to_string(), "-q requires a value");
}

#[test]
fn recognized_options_are_rejected_as_separated_query_values() {
    for option in ["-q", "--query"] {
        for value in ["-q", "--query", "-v", "--verbose", "-u", "--update"] {
            let error = Cli::parse([option.to_owned(), value.to_owned()])
                .expect_err("recognized options must not become query values");

            assert_eq!(error.to_string(), format!("{option} requires a value"));
        }
    }
}

#[test]
fn short_option_clusters_are_rejected_as_separated_query_values() {
    for option in ["-q", "--query"] {
        for value in ["-vu", "-vuqstate"] {
            let error = Cli::parse([option.to_owned(), value.to_owned()])
                .expect_err("valid short-option clusters must not become query values");

            assert_eq!(error.to_string(), format!("{option} requires a value"));
        }
    }
}

#[test]
fn unrecognized_dash_prefixed_query_text_is_preserved() -> Result<()> {
    let cli = Cli::parse(["-q".to_owned(), "--force".to_owned()])?;

    assert_eq!(cli.query, "--force");
    Ok(())
}

#[test]
fn unknown_argument_is_rejected() {
    let error = Cli::parse(["--unknown".to_owned()]).expect_err("unknown argument must fail");

    assert_eq!(error.to_string(), "unknown argument: --unknown");
}
