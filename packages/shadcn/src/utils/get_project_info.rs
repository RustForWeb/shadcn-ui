use std::path::PathBuf;

use anyhow::Result;

pub struct ProjectInfo {
    // framework: Framework,
    is_src_dir: bool,
    // is_rsc: bool,
    // is_tsx: bool,
    tailwind_config_file: Option<PathBuf>,
    tailwind_css_file: Option<PathBuf>,
    alias_prefix: Option<String>,
}

pub async fn get_project_info(cwd: PathBuf) -> Result<ProjectInfo> {
    // TODO

    let r#type = ProjectInfo {
        is_src_dir: false,
        tailwind_config_file: None,
        tailwind_css_file: None,
        alias_prefix: None,
    };

    Ok(r#type)
}
