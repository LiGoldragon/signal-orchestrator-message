#[test]
fn historical_schema_and_text_surfaces_are_absent() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!root.join("schema").join("signal.schema").exists());

    for source in [include_str!("../Cargo.toml"), include_str!("../src/lib.rs")] {
        for forbidden in [format!("{}{}", "no", "ta"), ".schema".to_owned()] {
            assert!(!source.to_ascii_lowercase().contains(&forbidden));
        }
    }
}
