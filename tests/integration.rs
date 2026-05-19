use rstest::rstest;
use std::process::Command;

fn run_binary(input: &str, output_dir: &std::path::Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_openapi2skill"))
        .arg(input)
        .arg("--output-dir")
        .arg(output_dir)
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
    let output = run_binary(path.to_str().unwrap(), tmp.path());
    assert!(
        output.status.success(),
        "{path:?}: binary exited with failure:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        tmp.path().join("SKILL.md").exists(),
        "{path:?}: missing SKILL.md"
    );
}

#[test]
fn test_generates_expected_files() {
    let tmp = tempfile::tempdir().unwrap();
    let output = run_binary("tests/assets/openapi.json", tmp.path());
    assert!(
        output.status.success(),
        "binary exited with failure:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let p = tmp.path();

    assert!(p.join("SKILL.md").exists(), "missing SKILL.md");

    assert!(
        p.join("endpoints/pet/index.md").exists(),
        "missing endpoints/pet/index.md"
    );
    assert!(
        p.join("endpoints/pet/get-pet-find-by-status-multiple-examples.md")
            .exists(),
        "missing endpoints/pet/get-pet-find-by-status-multiple-examples.md"
    );
    assert!(
        p.join("endpoints/pet/get-pet-find-by-status-single-example.md")
            .exists(),
        "missing endpoints/pet/get-pet-find-by-status-single-example.md"
    );
    assert!(
        p.join("endpoints/pet/post-pet.md").exists(),
        "missing endpoints/pet/post-pet.md"
    );

    assert!(
        p.join("schemas/index.md").exists(),
        "missing schemas/index.md"
    );
    assert!(p.join("schemas/pet.md").exists(), "missing schemas/pet.md");
    assert!(
        p.join("schemas/category.md").exists(),
        "missing schemas/category.md"
    );
    assert!(p.join("schemas/tag.md").exists(), "missing schemas/tag.md");
}

#[test]
fn test_no_auth_dir_when_no_schemes() {
    let tmp = tempfile::tempdir().unwrap();
    let output = run_binary("tests/assets/openapi.json", tmp.path());
    assert!(
        output.status.success(),
        "binary exited with failure:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !tmp.path().join("authentication").exists(),
        "authentication dir should not be created when no security schemes are defined"
    );
}
