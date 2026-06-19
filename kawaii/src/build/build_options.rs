#[derive(serde::Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct BuildOptions {
    #[serde(default)]
    pub link_args: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::project_info::ProjectInfo;

    #[test]
    fn defaults_build_options_when_missing() {
        let project: ProjectInfo = toml::from_str(
            r#"
            name = "hello"
            "#,
        )
        .unwrap();

        assert!(project.build_options.link_args.is_empty());
    }

    #[test]
    fn parses_hyphenated_link_args() {
        let project: ProjectInfo = toml::from_str(
            r#"
            name = "hello"

            [build]
            link-args = ["-nostdlib", "-T", "linker.ld"]
            "#,
        )
        .unwrap();

        assert_eq!(
            project.build_options.link_args,
            ["-nostdlib", "-T", "linker.ld"]
        );
    }
}
