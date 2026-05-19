use rstest::rstest;
use std::process::Command;

fn run_binary(input: &str, output_dir: &std::path::Path) -> std::process::Output {
    run_binary_with_args(input, output_dir, &[])
}

fn run_binary_with_args(
    input: &str,
    output_dir: &std::path::Path,
    extra_args: &[&str],
) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_openapi2skill"))
        .arg(input)
        .arg("--output-dir")
        .arg(output_dir)
        .args(extra_args)
        .output()
        .expect("failed to run openapi2skill binary")
}

#[rstest]
#[test]
fn test_spec_writes_successfully(
    #[files("tests/assets/*.json")]
    #[files("tests/assets/*.yaml")]
    #[files("tests/assets/*.yml")]
    path: std::path::PathBuf,
) {
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("out");
    let output = run_binary(path.to_str().unwrap(), &out);
    assert!(
        output.status.success(),
        "{path:?}: binary exited with failure:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(out.join("SKILL.md").exists(), "{path:?}: missing SKILL.md");
}

#[test]
fn test_generates_expected_files() {
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("out");
    let output = run_binary("tests/assets/openapi.json", &out);
    assert!(
        output.status.success(),
        "binary exited with failure:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(out.join("SKILL.md").exists(), "missing SKILL.md");

    assert!(
        out.join("endpoints/pet/index.md").exists(),
        "missing endpoints/pet/index.md"
    );
    assert!(
        out.join("endpoints/pet/get-pet-find-by-status-multiple-examples.md")
            .exists(),
        "missing endpoints/pet/get-pet-find-by-status-multiple-examples.md"
    );
    assert!(
        out.join("endpoints/pet/get-pet-find-by-status-single-example.md")
            .exists(),
        "missing endpoints/pet/get-pet-find-by-status-single-example.md"
    );
    assert!(
        out.join("endpoints/pet/post-pet.md").exists(),
        "missing endpoints/pet/post-pet.md"
    );

    assert!(
        out.join("schemas/index.md").exists(),
        "missing schemas/index.md"
    );
    assert!(
        out.join("schemas/pet.md").exists(),
        "missing schemas/pet.md"
    );
    assert!(
        out.join("schemas/category.md").exists(),
        "missing schemas/category.md"
    );
    assert!(
        out.join("schemas/tag.md").exists(),
        "missing schemas/tag.md"
    );
}

#[test]
fn test_error_when_output_dir_exists() {
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("out");
    run_binary("tests/assets/openapi.json", &out);
    let output = run_binary("tests/assets/openapi.json", &out);
    assert!(
        !output.status.success(),
        "binary should fail when output dir already exists"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("already exists"),
        "stderr should mention the dir already exists, got: {stderr}"
    );
}

#[test]
fn test_force_overwrites_existing_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("out");
    run_binary("tests/assets/openapi.json", &out);
    let output = run_binary_with_args("tests/assets/openapi.json", &out, &["--force"]);
    assert!(
        output.status.success(),
        "binary should succeed with --force when output dir already exists:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        out.join("SKILL.md").exists(),
        "missing SKILL.md after --force overwrite"
    );
}

#[test]
fn test_no_auth_dir_when_no_schemes() {
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("out");
    let output = run_binary("tests/assets/openapi.json", &out);
    assert!(
        output.status.success(),
        "binary exited with failure:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !out.join("authentication").exists(),
        "authentication dir should not be created when no security schemes are defined"
    );
}
