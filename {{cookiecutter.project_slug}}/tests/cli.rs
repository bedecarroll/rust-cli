use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
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

#[test]
fn greet_command_uses_config_default_when_name_missing() -> color_eyre::Result<()> {
    let temp = TempDir::new()?;
    let xdg_config = temp.child("config");
    xdg_config.create_dir_all()?;

    let project_dir = xdg_config.child("{{ cookiecutter.project_slug }}");
    project_dir.create_dir_all()?;
    project_dir
        .child("config.toml")
        .write_str(
            r#"
[greet]
default_name = "Config User"
"#,
        )?;

    let mut cmd = Command::cargo_bin("{{ cookiecutter.binary_name }}")?;
    cmd.env("XDG_CONFIG_HOME", xdg_config.path())
        .arg("greet")
        .assert()
        .success()
        .stdout(predicate::str::contains("Config User"));

    temp.close()?;
    Ok(())
}

#[test]
fn completions_command_writes_script() -> color_eyre::Result<()> {
    let temp = TempDir::new()?;
    let output_dir = temp.child("completions");
    output_dir.create_dir_all()?;

    let mut cmd = Command::cargo_bin("{{ cookiecutter.binary_name }}")?;
    cmd.args([
        "completions",
        "bash",
        "--dir",
        output_dir.path().to_str().unwrap(),
    ])
    .assert()
    .success();

    output_dir.assert(predicate::path::exists());
    let entries = std::fs::read_dir(output_dir.path())?
        .filter_map(std::result::Result::ok)
        .count();
    assert!(entries > 0, "expected at least one completion file to be generated");

    temp.close()?;
    Ok(())
}

#[test]
fn manpage_command_writes_file() -> color_eyre::Result<()> {
    let temp = TempDir::new()?;
    let output_dir = temp.child("man");
    output_dir.create_dir_all()?;

    let mut cmd = Command::cargo_bin("{{ cookiecutter.binary_name }}")?;
    cmd.args(["manpage", "--dir", output_dir.path().to_str().unwrap()])
        .assert()
        .success();

    let man_file = output_dir.child("{{ cookiecutter.binary_name }}.1");
    man_file.assert(predicate::path::exists());
    man_file.assert(predicate::str::contains(".TH"));

    temp.close()?;
    Ok(())
}

#[test]
fn conf_d_overrides_base_config_in_lexical_order() -> color_eyre::Result<()> {
    let temp = TempDir::new()?;
    let xdg_config = temp.child("config");
    xdg_config.create_dir_all()?;
    let project_dir = xdg_config.child("{{ cookiecutter.project_slug }}");
    project_dir.create_dir_all()?;

    project_dir
        .child("config.toml")
        .write_str(
            r#"
[greet]
default_name = "Base"
"#,
        )?;

    let conf_d = project_dir.child("conf.d");
    conf_d.create_dir_all()?;
    conf_d
        .child("10-early.toml")
        .write_str(
            r#"
[greet]
default_name = "Early"
"#,
        )?;
    conf_d
        .child("20-final.toml")
        .write_str(
            r#"
[greet]
default_name = "Final"
"#,
        )?;

    let mut cmd = Command::cargo_bin("{{ cookiecutter.binary_name }}")?;
    cmd.env("XDG_CONFIG_HOME", xdg_config.path())
        .arg("greet")
        .assert()
        .success()
        .stdout(predicate::str::contains("Final"));

    temp.close()?;
    Ok(())
}
