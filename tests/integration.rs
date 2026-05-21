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
    assert_no_empty_schema(out.join("schemas"));
    assert_no_empty_schema(out.join("endpoints"));
}

/// Non-regression: every emitted schema file must contain a real definition.
/// An empty `{}` jsonc block means the renderer dropped the schema's
/// properties (e.g. failed to merge `allOf` composition).
fn assert_no_empty_schema(dir: std::path::PathBuf) {
    assert!(dir.is_dir());
    for entry in std::fs::read_dir(&dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            assert_no_empty_schema(dir.join(path));
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        if path.file_name().and_then(|s| s.to_str()) == Some("index.md") {
            continue;
        }
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(
            !content.contains("```jsonc\n{}\n```"),
            "file {path:?} has an empty `{{}}` jsonc block:\n{content}"
        );
    }
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
fn test_openapi_3_1_type_arrays_render_as_array_of_types() {
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("out");
    let output = run_binary("tests/assets/31_types.openapi.json", &out);
    assert!(
        output.status.success(),
        "binary exited with failure:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let item_md = std::fs::read_to_string(out.join("schemas/item.md")).unwrap();
    assert!(
        item_md.contains("array[string, null]"),
        "expected `array[string, null]` for nullable string in:\n{item_md}"
    );
    assert!(
        item_md.contains("array[null, integer]"),
        "expected `array[null, integer]` preserving order in:\n{item_md}"
    );
    assert!(
        item_md.contains("array[object, null]"),
        "expected `array[object, null]` for nullable object in:\n{item_md}"
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
