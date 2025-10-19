use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_help_when_no_command_is_given() -> color_eyre::Result<()> {
    let mut cmd = Command::cargo_bin("{{ cookiecutter.binary_name }}")?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage"));

    Ok(())
}

#[test]
fn greet_command_prints_custom_name() -> color_eyre::Result<()> {
    let mut cmd = Command::cargo_bin("{{ cookiecutter.binary_name }}")?;
    cmd.args(["greet", "--name", "Agent"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Agent"));

    Ok(())
}
