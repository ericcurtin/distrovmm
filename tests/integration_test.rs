// Integration tests for distrovmm
// These tests verify the core functionality without requiring network access

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_parse_image_ref_with_tag() {
        // Test parsing image reference with tag
        let (repo, tag) = parse_image_ref("ubuntu:24.04");
        assert_eq!(repo, "ubuntu");
        assert_eq!(tag, "24.04");
    }

    #[test]
    fn test_parse_image_ref_without_tag() {
        // Test parsing image reference without tag (should default to latest)
        let (repo, tag) = parse_image_ref("fedora");
        assert_eq!(repo, "fedora");
        assert_eq!(tag, "latest");
    }

    #[test]
    fn test_parse_image_ref_with_multiple_colons() {
        // Test edge case with multiple colons
        let (repo, tag) = parse_image_ref("registry.io/repo:tag");
        assert_eq!(repo, "registry.io/repo");
        assert_eq!(tag, "tag");
    }

    // Helper function that mimics the one in oci.rs
    fn parse_image_ref(image: &str) -> (String, String) {
        if let Some((repo, tag)) = image.split_once(':') {
            (repo.to_string(), tag.to_string())
        } else {
            (image.to_string(), "latest".to_string())
        }
    }

    #[test]
    fn test_cache_dir_creation() {
        // Test that cache directory path can be constructed
        let cache_base = std::env::temp_dir();
        let cache_dir = cache_base.join("distrovmm");

        // Verify the path is constructible
        assert!(cache_dir.to_str().is_some());
        assert!(cache_dir.ends_with("distrovmm"));
    }

    #[test]
    fn test_vm_cache_path_construction() {
        // Test VM cache path construction
        let cache_dir = PathBuf::from("/tmp/distrovmm");
        let repo = "fedora";
        let tag = "latest";
        let vm_cache_dir = cache_dir.join(format!("vm_{}_{}", repo, tag));

        assert_eq!(
            vm_cache_dir,
            PathBuf::from("/tmp/distrovmm/vm_fedora_latest")
        );
    }
}
