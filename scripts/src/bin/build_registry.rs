use std::{collections::HashMap, env, fs, path::Path};

use anyhow::Result;
use handlebars::Handlebars;
use regex::Regex;
use serde::{Deserialize, Serialize};
use shadcn_registry::{
    registry_colors::{Color, COLORS, COLOR_MAPPING},
    registry_styles::STYLES,
    schema::{
        Mode, RegistryEntry, RegistryItemFile, RegistryItemTailwind, RegistryItemTailwindConfig,
        RegistryItemType, Style,
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
            css_vars: Some(HashMap::new()),
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

/// Build `registry/colors/index.json` and `registry/colors/[base].json`.
fn build_themes(output_path: &Path) -> Result<()> {
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum JsonColor {
        String(String),
        Value(JsonColorValue),
        Values(Vec<JsonColorScaleValue>),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct JsonColorValue {
        pub hex: String,
        pub rgb: String,
        pub hsl: String,
        pub rgb_channel: String,
        pub hsl_channel: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct JsonColorScaleValue {
        pub scale: usize,
        pub hex: String,
        pub rgb: String,
        pub hsl: String,
        pub rgb_channel: String,
        pub hsl_channel: String,
    }

    let colors_target_path = output_path.join("r/colors");
    if colors_target_path.exists() {
        fs::remove_dir_all(&colors_target_path)?;
    }
    fs::create_dir_all(&colors_target_path)?;

    let rgb_regex = Regex::new(r"^rgb\((\d+),(\d+),(\d+)\)$").expect("Regex should be valid.");
    let hsl_regex =
        Regex::new(r"^hsl\(([\d.]+),([\d.]+%),([\d.]+%)\)$").expect("Regex should be valid.");

    let mut color_data: HashMap<String, JsonColor> = HashMap::new();
    for (color, value) in COLORS.iter() {
        color_data.insert(
            color.clone(),
            match value {
                Color::String(value) => JsonColor::String(value.clone()),
                Color::Value(value) => JsonColor::Value(JsonColorValue {
                    hex: value.hex.clone(),
                    rgb: value.rgb.clone(),
                    hsl: value.hsl.clone(),
                    rgb_channel: rgb_regex.replace(&value.rgb, "$1 $2 $3").to_string(),
                    hsl_channel: hsl_regex.replace(&value.hsl, "$1 $2 $3").to_string(),
                }),
                Color::Values(values) => JsonColor::Values(
                    values
                        .iter()
                        .map(|value| JsonColorScaleValue {
                            scale: value.scale,
                            hex: value.hex.clone(),
                            rgb: value.rgb.clone(),
                            hsl: value.hsl.clone(),
                            rgb_channel: rgb_regex.replace(&value.rgb, "$1 $2 $3").to_string(),
                            hsl_channel: hsl_regex.replace(&value.hsl, "$1 $2 $3").to_string(),
                        })
                        .collect::<Vec<_>>(),
                ),
            },
        );
    }

    let color_data_json = serde_json::to_string_pretty(&color_data)?;
    fs::write(colors_target_path.join("index.json"), color_data_json)?;

    let handlebars = Handlebars::new();

    const BASE_STYLES: &str = include_str!("templates/base_style.css");
    const BASE_STYLES_WITH_VARIABLES: &str =
        include_str!("templates/base_style_with_variables.css");

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct BaseColor {
        inline_colors: HashMap<Mode, HashMap<String, String>>,
        css_vars: HashMap<Mode, HashMap<String, String>>,
        inline_colors_template: String,
        css_vars_template: String,
    }

    let base_color_regex = Regex::new(r"\{\{base\}\}-").expect("Regex should be valid.");

    for base_color in ["slate", "gray", "zinc", "neutral", "stone"] {
        let mut base = BaseColor::default();

        for (mode, values) in COLOR_MAPPING.iter() {
            let mut inline_colors = HashMap::new();
            let mut css_vars = HashMap::new();

            for (key, value) in values {
                // Chart colors do not have a 1-to-1 mapping with Tailwind colors.
                if key.starts_with("chart-") {
                    css_vars.insert(key.clone(), value.clone());
                    continue;
                }

                let resolved_color = base_color_regex
                    .replace_all(value, format!("{base_color}-"))
                    .to_string();
                inline_colors.insert(key.clone(), resolved_color.clone());

                let mut split = resolved_color.split('-');
                let resolved_base = split.next().expect("Split should have at least one match.");
                let scale = split.next().and_then(|scale| scale.parse::<usize>().ok());
                let color = color_data.get(resolved_base).expect("msg");
                let color = match scale {
                    Some(scale) => match color {
                        JsonColor::Values(values) => values.iter().find_map(|value| {
                            (value.scale == scale).then_some(value.hsl_channel.clone())
                        }),
                        _ => unreachable!("Color must be a scale."),
                    },
                    None => match color {
                        JsonColor::Value(value) => Some(value.hsl_channel.clone()),
                        _ => unreachable!("Color must not be a string or a scale."),
                    },
                };
                if let Some(color) = color {
                    css_vars.insert(key.clone(), color);
                }
            }

            base.inline_colors.insert(*mode, inline_colors);
            base.css_vars.insert(*mode, css_vars);
        }

        // Build CSS vars.
        base.inline_colors_template = handlebars.render_template(BASE_STYLES, &())?;
        base.css_vars_template = handlebars.render_template(
            BASE_STYLES_WITH_VARIABLES,
            &HashMap::from([("colors", &base.css_vars)]),
        )?;

        let base_json = serde_json::to_string_pretty(&base)?;
        fs::write(
            output_path.join(format!("r/colors/{base_color}.json")),
            base_json,
        )?;

        // TODO
    }

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
