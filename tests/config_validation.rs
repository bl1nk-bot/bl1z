/// Tests for repository configuration files.
///
/// These tests validate the structure and content of configuration files
/// that affect CI/CD pipeline behavior. They serve as regression guards
/// to prevent accidental misconfiguration.
#[cfg(test)]
mod dependabot_config {
    use std::fs;
    use std::path::Path;

    fn read_dependabot_yml() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".github/dependabot.yml");
        fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read .github/dependabot.yml: {e}"))
    }

    // ── version field ────────────────────────────────────────────────────────

    #[test]
    fn version_is_2() {
        let content = read_dependabot_yml();
        assert!(
            content.starts_with("version: 2"),
            "dependabot.yml must start with 'version: 2' (GitHub requires it at the top level)"
        );
    }

    #[test]
    fn version_2_appears_exactly_once() {
        let content = read_dependabot_yml();
        let count = content.matches("version: 2").count();
        assert_eq!(count, 1, "version: 2 should appear exactly once, found {count}");
    }

    // ── ecosystem: cargo ─────────────────────────────────────────────────────

    #[test]
    fn cargo_ecosystem_is_present() {
        let content = read_dependabot_yml();
        assert!(
            content.contains("package-ecosystem: \"cargo\""),
            "dependabot.yml must configure Cargo dependency updates"
        );
    }

    #[test]
    fn cargo_schedule_is_weekly() {
        let content = read_dependabot_yml();
        // Locate the cargo section and check schedule within it
        let cargo_pos = content
            .find("package-ecosystem: \"cargo\"")
            .expect("cargo ecosystem section not found");
        let cargo_section = &content[cargo_pos..];
        assert!(
            cargo_section.contains("interval: \"weekly\""),
            "Cargo update schedule must be 'weekly'"
        );
    }

    #[test]
    fn cargo_schedule_day_is_monday() {
        let content = read_dependabot_yml();
        let cargo_pos = content
            .find("package-ecosystem: \"cargo\"")
            .expect("cargo ecosystem section not found");
        let cargo_section = &content[cargo_pos..];
        assert!(
            cargo_section.contains("day: \"monday\""),
            "Cargo update schedule day must be 'monday'"
        );
    }

    #[test]
    fn cargo_open_pull_requests_limit_is_10() {
        let content = read_dependabot_yml();
        let cargo_pos = content
            .find("package-ecosystem: \"cargo\"")
            .expect("cargo ecosystem section not found");
        // Grab text up to the next ecosystem entry (or end of file)
        let next_eco = content[cargo_pos + 1..]
            .find("package-ecosystem:")
            .map(|p| cargo_pos + 1 + p)
            .unwrap_or(content.len());
        let cargo_section = &content[cargo_pos..next_eco];
        assert!(
            cargo_section.contains("open-pull-requests-limit: 10"),
            "Cargo open-pull-requests-limit must be 10"
        );
    }

    #[test]
    fn cargo_labels_include_dependencies_and_rust() {
        let content = read_dependabot_yml();
        let cargo_pos = content
            .find("package-ecosystem: \"cargo\"")
            .expect("cargo ecosystem section not found");
        let next_eco = content[cargo_pos + 1..]
            .find("package-ecosystem:")
            .map(|p| cargo_pos + 1 + p)
            .unwrap_or(content.len());
        let cargo_section = &content[cargo_pos..next_eco];
        assert!(
            cargo_section.contains("\"dependencies\""),
            "Cargo updates must be labelled 'dependencies'"
        );
        assert!(
            cargo_section.contains("\"rust\""),
            "Cargo updates must be labelled 'rust'"
        );
    }

    #[test]
    fn cargo_commit_prefix_is_deps() {
        let content = read_dependabot_yml();
        let cargo_pos = content
            .find("package-ecosystem: \"cargo\"")
            .expect("cargo ecosystem section not found");
        let next_eco = content[cargo_pos + 1..]
            .find("package-ecosystem:")
            .map(|p| cargo_pos + 1 + p)
            .unwrap_or(content.len());
        let cargo_section = &content[cargo_pos..next_eco];
        assert!(
            cargo_section.contains("prefix: \"deps\""),
            "Cargo commit-message prefix must be 'deps'"
        );
    }

    #[test]
    fn cargo_has_rust_dependencies_group() {
        let content = read_dependabot_yml();
        let cargo_pos = content
            .find("package-ecosystem: \"cargo\"")
            .expect("cargo ecosystem section not found");
        let next_eco = content[cargo_pos + 1..]
            .find("package-ecosystem:")
            .map(|p| cargo_pos + 1 + p)
            .unwrap_or(content.len());
        let cargo_section = &content[cargo_pos..next_eco];
        assert!(
            cargo_section.contains("rust-dependencies:"),
            "Cargo updates must define a 'rust-dependencies' group"
        );
    }

    #[test]
    fn cargo_group_uses_wildcard_pattern() {
        let content = read_dependabot_yml();
        let cargo_pos = content
            .find("package-ecosystem: \"cargo\"")
            .expect("cargo ecosystem section not found");
        let next_eco = content[cargo_pos + 1..]
            .find("package-ecosystem:")
            .map(|p| cargo_pos + 1 + p)
            .unwrap_or(content.len());
        let cargo_section = &content[cargo_pos..next_eco];
        assert!(
            cargo_section.contains("- \"*\""),
            "rust-dependencies group must use wildcard pattern '*'"
        );
    }

    // ── ecosystem: github-actions ────────────────────────────────────────────

    #[test]
    fn github_actions_ecosystem_is_present() {
        let content = read_dependabot_yml();
        assert!(
            content.contains("package-ecosystem: \"github-actions\""),
            "dependabot.yml must configure GitHub Actions dependency updates"
        );
    }

    #[test]
    fn github_actions_schedule_is_weekly() {
        let content = read_dependabot_yml();
        let gha_pos = content
            .find("package-ecosystem: \"github-actions\"")
            .expect("github-actions ecosystem section not found");
        let gha_section = &content[gha_pos..];
        assert!(
            gha_section.contains("interval: \"weekly\""),
            "GitHub Actions update schedule must be 'weekly'"
        );
    }

    #[test]
    fn github_actions_schedule_day_is_monday() {
        let content = read_dependabot_yml();
        let gha_pos = content
            .find("package-ecosystem: \"github-actions\"")
            .expect("github-actions ecosystem section not found");
        let gha_section = &content[gha_pos..];
        assert!(
            gha_section.contains("day: \"monday\""),
            "GitHub Actions update schedule day must be 'monday'"
        );
    }

    #[test]
    fn github_actions_open_pull_requests_limit_is_5() {
        let content = read_dependabot_yml();
        let gha_pos = content
            .find("package-ecosystem: \"github-actions\"")
            .expect("github-actions ecosystem section not found");
        let gha_section = &content[gha_pos..];
        assert!(
            gha_section.contains("open-pull-requests-limit: 5"),
            "GitHub Actions open-pull-requests-limit must be 5"
        );
    }

    #[test]
    fn github_actions_labels_include_github_actions_and_ci() {
        let content = read_dependabot_yml();
        let gha_pos = content
            .find("package-ecosystem: \"github-actions\"")
            .expect("github-actions ecosystem section not found");
        let gha_section = &content[gha_pos..];
        assert!(
            gha_section.contains("\"github-actions\""),
            "GitHub Actions updates must be labelled 'github-actions'"
        );
        assert!(
            gha_section.contains("\"ci\""),
            "GitHub Actions updates must be labelled 'ci'"
        );
    }

    #[test]
    fn github_actions_commit_prefix_is_ci() {
        let content = read_dependabot_yml();
        let gha_pos = content
            .find("package-ecosystem: \"github-actions\"")
            .expect("github-actions ecosystem section not found");
        let gha_section = &content[gha_pos..];
        assert!(
            gha_section.contains("prefix: \"ci\""),
            "GitHub Actions commit-message prefix must be 'ci'"
        );
    }

    // ── shared reviewer / assignee ───────────────────────────────────────────

    #[test]
    fn reviewer_is_billlzzz26() {
        let content = read_dependabot_yml();
        // Both entries should reference billlzzz26
        let count = content.matches("\"billlzzz26\"").count();
        assert!(
            count >= 2,
            "Expected billlzzz26 to appear in at least 2 places (reviewer + assignee per entry), found {count}"
        );
    }

    #[test]
    fn assignee_is_billlzzz26() {
        let content = read_dependabot_yml();
        // assignees: field should be present for both entries
        let assignee_count = content.matches("assignees:").count();
        assert!(
            assignee_count >= 2,
            "Both update entries must have an assignees field, found {assignee_count}"
        );
    }

    // ── regression: removed gh-aw-actions ignore rule ────────────────────────

    #[test]
    fn no_gh_aw_actions_ignore_rule() {
        let content = read_dependabot_yml();
        assert!(
            !content.contains("gh-aw-actions"),
            "dependabot.yml must not contain gh-aw-actions ignore rules (agentic workflow tooling was removed)"
        );
    }

    #[test]
    fn no_ignore_block_in_github_actions_section() {
        let content = read_dependabot_yml();
        assert!(
            !content.contains("ignore:"),
            "No 'ignore:' blocks expected in dependabot.yml after removal of agentic workflow tooling"
        );
    }

    // ── structural completeness ───────────────────────────────────────────────

    #[test]
    fn exactly_two_update_entries() {
        let content = read_dependabot_yml();
        let count = content.matches("package-ecosystem:").count();
        assert_eq!(
            count, 2,
            "dependabot.yml should have exactly 2 update entries (cargo + github-actions), found {count}"
        );
    }

    #[test]
    fn updates_key_is_present() {
        let content = read_dependabot_yml();
        assert!(
            content.contains("updates:"),
            "dependabot.yml must contain the top-level 'updates:' key"
        );
    }

    #[test]
    fn both_entries_target_root_directory() {
        let content = read_dependabot_yml();
        let count = content.matches("directory: \"/\"").count();
        assert_eq!(
            count, 2,
            "Both update entries must target the root directory '/'"
        );
    }

    #[test]
    fn commit_message_include_scope_for_both_entries() {
        let content = read_dependabot_yml();
        let count = content.matches("include: \"scope\"").count();
        assert_eq!(
            count, 2,
            "Both update entries must set 'include: scope' in commit-message, found {count}"
        );
    }

    // ── time field ───────────────────────────────────────────────────────────

    #[test]
    fn schedule_time_is_09_00() {
        let content = read_dependabot_yml();
        let count = content.matches("time: \"09:00\"").count();
        assert_eq!(
            count, 2,
            "Both entries must schedule updates at '09:00', found {count}"
        );
    }
}
