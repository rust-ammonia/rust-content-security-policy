#[cfg(feature = "version-sync")]
#[test]
fn test_readme_deps() {
    use version_sync::assert_markdown_deps_updated;
    assert_markdown_deps_updated!("README.md");
}

#[cfg(not(feature = "version-sync"))]
#[test]
#[ignore]
fn test_readme_deps() {
    assert!(false)
}
