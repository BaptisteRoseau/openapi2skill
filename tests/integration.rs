use openapi2skill::writer::openapi2skill;

async fn load_spec(path: &str) -> oas3::OpenApiV3Spec {
    let content = tokio::fs::read_to_string(path).await.unwrap();
    oas3::from_json(&content).unwrap()
}

#[tokio::test]
async fn test_generates_expected_files() {
    let spec = load_spec("tests/assets/openapi.json").await;
    let tmp = tempfile::tempdir().unwrap();

    openapi2skill(&spec, Some(tmp.path())).await.unwrap();

    let p = tmp.path();

    // Root skill file
    assert!(p.join("SKILL.md").exists(), "missing SKILL.md");

    // Endpoint category
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

    // Schemas
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

#[tokio::test]
async fn test_no_auth_dir_when_no_schemes() {
    let spec = load_spec("tests/assets/openapi.json").await;
    let tmp = tempfile::tempdir().unwrap();

    openapi2skill(&spec, Some(tmp.path())).await.unwrap();

    assert!(
        !tmp.path().join("authentication").exists(),
        "authentication dir should not be created when no security schemes are defined"
    );
}
