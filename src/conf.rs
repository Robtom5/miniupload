use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

const ENV_TARGET: &str = "MINIUPLOAD_TARGET";
const ENV_FOLDER: &str = "MINIUPLOAD_FOLDER";

#[derive(Serialize, Deserialize)]
pub struct ToolConfig {
    version: u8,
    target: String,
    folder: String,
}

impl ToolConfig {
    pub fn from_app(app_name: &str) -> Result<Self> {
        Ok(confy::load(app_name, None)?)
    }

    pub fn get_folder(&self) -> Result<String> {
        env::var(ENV_FOLDER).or_else(|_| Ok(self.folder.clone()))
    }

    pub fn get_target(&self) -> Result<String> {
        env::var(ENV_TARGET).or_else(|_| Ok(self.target.clone()))
    }

    pub fn get_upload_target(&self) -> Result<String> {
        let mut upload_target = self.get_target()?;
        upload_target.push_str("upload?path=/");
        Ok(upload_target)
    }

    pub fn save_app(&self, app_name: &str) -> Result<()> {
        Ok(confy::store(app_name, None, self)?)
    }

    pub fn update_target(&mut self, new_target: String) {
        self.target = new_target;
    }

    pub fn update_folder(&mut self, new_folder: String) {
        self.folder = new_folder;
    }
}

impl std::default::Default for ToolConfig {
    fn default() -> Self {
        Self {
            version: 0,
            target: "".into(),
            folder: "".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process;

    #[derive(Default)]
    struct MultiAssert<'a> {
        checks: Vec<(&'a str, bool)>,
    }

    impl<'a> MultiAssert<'a> {
        fn add_subtest_eq(&mut self, test: &'a str, val1: String, val2: String) {
            self.checks.push((test, val1 == val2));
        }

        fn assert_subtests(&self) {
            let mut all_passed = true;
            let mut failed_tests: String = "Following subtests failed: \n".to_string();

            for (test, passed) in &self.checks {
                all_passed &= passed;
                match passed {
                    true => {}
                    false => {
                        failed_tests.push_str(test);
                        failed_tests.push_str("\n");
                    }
                }
            }
            assert!(all_passed, "{}", failed_tests)
        }
    }

    #[test]
    fn verify_get_behaviour() -> Result<()> {
        // Concurrency of Rust tests makes unit tests very difficult
        // So we have created a mechanism for subtests
        let app_name = format!("unit_test_app_{}", process::id());

        let mut setup_conf: ToolConfig = Default::default();
        let mut subtests: MultiAssert = Default::default();

        let target = "MY_TARGET";
        let folder = "MY_FOLDER";
        let env_target = "MY_ENV_TARGET";
        let env_folder = "MY_ENV_FOLDER";

        setup_conf.update_target(target.to_string());
        setup_conf.update_folder(folder.to_string());
        setup_conf.save_app(&app_name)?;

        let conf: ToolConfig = ToolConfig::from_app(&app_name)?;

        env::set_var(ENV_TARGET, env_target);
        env::set_var(ENV_FOLDER, env_folder);

        subtests.add_subtest_eq(
            "Target read from env",
            conf.get_target()?,
            env_target.to_string(),
        );
        subtests.add_subtest_eq(
            "Folder read from env",
            conf.get_folder()?,
            env_folder.to_string(),
        );

        std::env::remove_var(ENV_TARGET);
        std::env::remove_var(ENV_FOLDER);

        subtests.add_subtest_eq(
            "Target read from file",
            conf.get_target()?,
            target.to_string(),
        );
        subtests.add_subtest_eq(
            "Folder read from file",
            conf.get_folder()?,
            folder.to_string(),
        );

        subtests.assert_subtests();

        let file = confy::get_configuration_file_path(&app_name, None)
            .expect("Error getting configuration path");

        std::fs::remove_dir_all(file.parent().unwrap())?;

        Ok(())
    }

    #[test]
    fn verify_toolconfig_default_version() {
        let default_cfg: ToolConfig = Default::default();
        assert_eq!(default_cfg.version, 0)
    }

    #[test]
    fn verify_toolconfig_default_target() {
        let default_cfg: ToolConfig = Default::default();
        assert_eq!(default_cfg.target, "".to_string())
    }

    #[test]
    fn verify_toolconfig_default_folder() {
        let default_cfg: ToolConfig = Default::default();
        assert_eq!(default_cfg.folder, "".to_string())
    }

    #[test]
    fn verify_get_upload_target_appends_expected() -> Result<()> {
        let default_cfg: ToolConfig = Default::default();
        assert_eq!(
            default_cfg.get_upload_target()?,
            "upload?path=/".to_string()
        );
        Ok(())
    }
}
