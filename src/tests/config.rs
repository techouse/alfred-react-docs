#[cfg(unix)]
use std::ffi::OsString;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;

use super::*;

#[test]
fn runtime_value_takes_precedence_over_other_sources() -> Result<()> {
    let value = configuration_value(
        "SETTING",
        Ok("runtime".to_owned()),
        Some("dotenv"),
        Some("embedded"),
    )?;

    assert_eq!(value, "runtime");
    Ok(())
}

#[test]
fn dotenv_value_takes_precedence_over_embedded_value() -> Result<()> {
    let value = configuration_value(
        "SETTING",
        Err(VarError::NotPresent),
        Some("dotenv"),
        Some("embedded"),
    )?;

    assert_eq!(value, "dotenv");
    Ok(())
}

#[test]
fn embedded_value_is_used_when_other_sources_are_missing() -> Result<()> {
    let value = configuration_value("SETTING", Err(VarError::NotPresent), None, Some("embedded"))?;

    assert_eq!(value, "embedded");
    Ok(())
}

#[test]
fn empty_runtime_value_is_rejected() {
    let error = configuration_value("SETTING", Ok(String::new()), None, Some("embedded"))
        .expect_err("an empty runtime override must be rejected");

    assert_eq!(error.to_string(), "SETTING must not be empty");
}

#[test]
fn empty_dotenv_value_is_rejected() {
    let error = configuration_value(
        "SETTING",
        Err(VarError::NotPresent),
        Some(""),
        Some("embedded"),
    )
    .expect_err("an empty dotenv value must be rejected");

    assert_eq!(error.to_string(), "SETTING must not be empty");
}

#[test]
fn missing_values_are_rejected() {
    let error = configuration_value("SETTING", Err(VarError::NotPresent), None, None)
        .expect_err("a missing setting must be rejected");

    assert_eq!(
        error.to_string(),
        "SETTING must be set in the environment, .env file, or embedded at build time"
    );
}

#[cfg(unix)]
#[test]
fn non_unicode_runtime_value_is_rejected() {
    let invalid = OsString::from_vec(vec![0xff]);
    let error = configuration_value(
        "SETTING",
        Err(VarError::NotUnicode(invalid)),
        None,
        Some("embedded"),
    )
    .expect_err("non-Unicode values must be rejected");

    assert_eq!(error.to_string(), "SETTING must contain valid Unicode");
}

#[test]
fn missing_dotenv_file_is_empty() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let values = load_dotenv(&directory.path().join("missing.env"))?;

    assert!(values.is_empty());
    Ok(())
}

#[test]
fn dotenv_file_is_loaded_without_mutating_process_environment() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let path = directory.path().join(".env");
    std::fs::write(&path, "SETTING=dotenv\nOTHER=value\n")?;

    let values = load_dotenv(&path)?;

    assert_eq!(
        (values.get("SETTING").map(String::as_str), values.len()),
        (Some("dotenv"), 2)
    );
    Ok(())
}

#[test]
fn malformed_dotenv_file_is_rejected() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let path = directory.path().join(".env");
    std::fs::write(&path, "INVALID LINE\n")?;

    let error = load_dotenv(&path).expect_err("malformed dotenv must fail");

    assert!(error.to_string().contains("failed to parse"));
    Ok(())
}
