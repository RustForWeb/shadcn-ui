use std::path::{Path, PathBuf};

use anyhow::Result;

pub struct ProjectInfo {
    // pub framework: Framework,
    pub is_src_dir: bool,
    // pub is_rsc: bool,
    // pub is_tsx: bool,
    pub tailwind_config_file: Option<PathBuf>,
    pub tailwind_css_file: Option<PathBuf>,
    // pub alias_prefix: Option<String>,
}

pub async fn get_project_info(cwd: &Path) -> Result<ProjectInfo> {
    // TODO

    let r#type = ProjectInfo {
        is_src_dir: false,
        tailwind_config_file: None,
        tailwind_css_file: None,
    };

    Ok(r#type)
}
