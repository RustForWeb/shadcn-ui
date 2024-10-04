use std::{collections::HashMap, env, fs, path::Path};

use anyhow::Result;
use shadcn_registry::{
    registry_styles::STYLES,
    schema::{
        RegistryEntry, RegistryItemCssVars, RegistryItemFile, RegistryItemTailwind,
        RegistryItemTailwindConfig, RegistryItemType, Style,
    },
    REGISTRY,
};

const REGISTRY_INDEX_WHITELIST: [RegistryItemType; 5] = [
    RegistryItemType::Block,
    RegistryItemType::Hook,
    RegistryItemType::Lib,
    RegistryItemType::Theme,
    RegistryItemType::Ui,
];

/// Build `registry/index.json` and `__registry__/index.json`.
fn build_registry(output_path: &Path) -> Result<()> {
    let mut index: HashMap<Style, HashMap<String, RegistryEntry>> = HashMap::new();

    for style in STYLES {
        index.insert(style.name, HashMap::new());

        for _item in REGISTRY.iter() {
            // TODO
        }
    }

    let items = REGISTRY
        .iter()
        .filter(|item| item.r#type == RegistryItemType::Ui)
        .collect::<Vec<_>>();
    let registry_json = serde_json::to_string_pretty(&items)?;
    let path = output_path.join("r/index.json");
    fs::write(&path, registry_json)?;

    let index_json = serde_json::to_string_pretty(&items)?;
    let path = output_path.join("__registry__/index.json");
    fs::write(&path, index_json)?;

    Ok(())
}

/// Build `registry/styles/[style]/[name].json` and `registry/styles/index.json`.
fn build_styles(_input_path: &Path, output_path: &Path) -> Result<()> {
    for style in STYLES {
        let target_path = output_path.join("r/styles").join(style.name.to_string());

        // Create directory if it doesn't exist.
        if !target_path.exists() {
            fs::create_dir_all(&target_path)?;
        }

        for item in REGISTRY.iter() {
            if !REGISTRY_INDEX_WHITELIST.contains(&item.r#type) {
                continue;
            }

            let mut payload_files = None;
            if let Some(item_files) = &item.files {
                let mut files: Vec<RegistryItemFile> = vec![];
                for file in item_files {
                    // TODO
                    // let content =
                    //     fs::read_to_string(input_path.join(style.name.to_string()).join(&file.path))?;
                    let content = "".to_string();

                    // TODO: Strip certain declarations?

                    files.push(RegistryItemFile {
                        content: Some(content),
                        ..file.clone()
                    });
                }
                payload_files = Some(files);
            }

            let payload = RegistryEntry {
                source: None,
                category: None,
                subcategory: None,
                chunks: None,
                files: payload_files,
                ..item.clone()
            };
            let payload_json = serde_json::to_string_pretty(&payload)?;
            fs::write(
                target_path.join(format!("{}.json", item.name)),
                payload_json,
            )?;
        }
    }

    let styles_json = serde_json::to_string_pretty(&STYLES)?;
    fs::write(output_path.join("r/styles/index.json"), styles_json)?;

    Ok(())
}

/// Build `registry/styles/[name]/index.json`.
fn build_styles_index(output_path: &Path) -> Result<()> {
    for style in STYLES {
        let target_path = output_path.join("r/styles").join(style.name.to_string());

        // TODO: Rustify dependencies

        let mut dependencies: Vec<String> = vec![
            "tailwindcss-animate".into(),
            "class-variance-authority".into(),
            "lucide-react".into(),
        ];

        // TODO: Remove this when we migrate to lucide-react.
        if style.name == Style::NewYork {
            dependencies.push("@radix-ui/react-icons".into());
        }

        let payload = RegistryEntry {
            name: style.name.to_string(),
            r#type: RegistryItemType::Style,
            description: None,
            dependencies: Some(dependencies),
            dev_dependencies: None,
            registry_dependencies: Some(vec!["utils".into()]),
            files: Some(vec![]),
            tailwind: Some(RegistryItemTailwind {
                config: RegistryItemTailwindConfig {
                    content: None,
                    plugins: Some(vec!["require(\"tailwindcss-animate\")".into()]),
                },
            }),
            css_vars: Some(RegistryItemCssVars {
                light: None,
                dark: None,
            }),
            source: None,
            category: None,
            subcategory: None,
            chunks: None,
            docs: None,
        };

        let payload_json = serde_json::to_string_pretty(&payload)?;
        fs::write(target_path.join("index.json"), payload_json)?;
    }

    Ok(())
}

/// Build `registry/colors/index.json`.
fn build_themes(_output_path: &Path) -> Result<()> {
    // TODO

    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    let input_path = env::current_dir()?;
    let output_path = env::current_dir()?.join("dist");

    if output_path.exists() {
        fs::remove_dir_all(&output_path)?;
    }
    fs::create_dir_all(&output_path)?;

    let path = output_path.join("r");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    let path = output_path.join("__registry__");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    build_registry(&output_path)?;
    build_styles(&input_path, &output_path)?;
    build_styles_index(&output_path)?;
    build_themes(&output_path)?;

    log::info!("✅ Done!");

    Ok(())
}
