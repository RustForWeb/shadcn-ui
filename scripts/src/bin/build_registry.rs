use std::{collections::HashMap, env, fs, path::Path};

use anyhow::Result;
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use shadcn_registry::{
    REGISTRY,
    registry_base_colors::BASE_COLORS,
    registry_colors::{COLOR_MAPPING, COLORS, Color},
    registry_frameworks::FRAMEWORKS,
    registry_styles::STYLES,
    schema::{
        Mode, RegistryEntry, RegistryItemFile, RegistryItemTailwind, RegistryItemTailwindConfig,
        RegistryItemType, Style,
    },
};

const REGISTRY_INDEX_WHITELIST: [RegistryItemType; 5] = [
    RegistryItemType::Block,
    RegistryItemType::Hook,
    RegistryItemType::Lib,
    RegistryItemType::Theme,
    RegistryItemType::Ui,
];

/// Build `registry/frameworks/index.json`.
fn build_frameworks(output_path: &Path) -> Result<()> {
    fs::create_dir_all(output_path.join("r/frameworks"))?;

    let frameworks_json = serde_json::to_string_pretty(&*FRAMEWORKS)?;
    let path = output_path.join("r/frameworks/index.json");
    fs::write(&path, frameworks_json)?;

    for framework in FRAMEWORKS.iter() {
        let path = output_path.join(format!("r/frameworks/{}", framework.name));
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
    }

    Ok(())
}

/// Build `registry/frameworks/[framework]/index.json`.
fn build_registry(output_path: &Path) -> Result<()> {
    for (framework, registry) in REGISTRY.iter() {
        let items = registry
            .iter()
            .filter(|item| item.r#type == RegistryItemType::Ui)
            .collect::<Vec<_>>();

        let registry_json = serde_json::to_string_pretty(&items)?;
        let path = output_path.join(format!("r/frameworks/{framework}/index.json"));
        fs::write(&path, registry_json)?;
    }

    Ok(())
}

/// Build `registry/frameworks/[framework]/styles/[style]/[name].json` and `registry/frameworks/[framework]/styles/index.json`.
fn build_styles(input_path: &Path, output_path: &Path) -> Result<()> {
    for (framework, registry) in REGISTRY.iter() {
        let target_path = output_path.join(format!("r/frameworks/{framework}"));

        for style in STYLES {
            let target_path = target_path.join(format!("styles/{}", style.name));

            // Create directory if it doesn't exist.
            if !target_path.exists() {
                fs::create_dir_all(&target_path)?;
            }

            for item in registry {
                if !REGISTRY_INDEX_WHITELIST.contains(&item.r#type) {
                    continue;
                }

                let mut payload_files = None;
                if let Some(item_files) = &item.files {
                    let mut files: Vec<RegistryItemFile> = vec![];
                    for file in item_files {
                        let path = input_path.join(format!(
                            "{}/{}/src/{}.rs",
                            framework,
                            item.name,
                            style.name.to_string().to_case(Case::Snake)
                        ));
                        log::info!("{path:?}");
                        let content = fs::read_to_string(path)?;

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
        fs::write(target_path.join("styles/index.json"), styles_json)?;
    }

    Ok(())
}

/// Build `registry/frameworks/[framework]/styles/[name]/index.json`.
fn build_styles_index(output_path: &Path) -> Result<()> {
    for framework in FRAMEWORKS.iter() {
        for style in STYLES {
            let target_path = output_path.join(format!(
                "r/frameworks/{}/styles/{}",
                framework.name, style.name
            ));

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

    const BASE_STYLES: &str = include_str!("templates/base_styles.css");
    const BASE_STYLES_WITH_VARIABLES: &str =
        include_str!("templates/base_styles_with_variables.css");

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
                let color = color_data.get(resolved_base).and_then(|color| match scale {
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
                });
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
            &json!({
                "colors": &base.css_vars
            }),
        )?;

        let base_json = serde_json::to_string_pretty(&base)?;
        fs::write(
            output_path.join(format!("r/colors/{base_color}.json")),
            base_json,
        )?;

        const THEME_STYLES_WITH_VARIABLES: &str =
            include_str!("templates/theme_styles_with_variables.css");

        let mut theme_css = vec![];
        for theme in BASE_COLORS.iter() {
            theme_css.push(handlebars.render_template(
                THEME_STYLES_WITH_VARIABLES,
                &json!({
                    "colors": theme.css_vars,
                    "theme": theme.name
                }),
            )?);
        }

        fs::write(output_path.join("r/themes.css"), theme_css.join("\n"))?;

        #[derive(Clone, Debug, Default, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Payload {
            name: String,
            label: String,
            css_vars: HashMap<Mode, HashMap<String, String>>,
        }

        let target_path = output_path.join("r/themes");
        if target_path.exists() {
            fs::remove_dir_all(&target_path)?;
        }
        fs::create_dir_all(&target_path)?;

        for base_color in ["slate", "gray", "zinc", "neutral", "stone"] {
            let mut css_vars = HashMap::new();

            for (mode, values) in COLOR_MAPPING.iter() {
                let mut vars = HashMap::new();

                for (key, value) in values {
                    let resolved_color = base_color_regex
                        .replace_all(value, format!("{base_color}-"))
                        .to_string();
                    vars.insert(key.clone(), resolved_color.clone());

                    let mut split = resolved_color.split('-');
                    let resolved_base =
                        split.next().expect("Split should have at least one match.");
                    let scale = split.next().and_then(|scale| scale.parse::<usize>().ok());
                    let color = color_data.get(resolved_base).and_then(|color| match scale {
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
                    });
                    if let Some(color) = color {
                        vars.insert(key.clone(), color);
                    }
                }

                css_vars.insert(*mode, vars);
            }

            let payload = Payload {
                name: base_color.to_string(),
                label: format!("{}{}", &base_color[0..1].to_uppercase(), &base_color[1..]),
                css_vars,
            };

            let payload_json = serde_json::to_string_pretty(&payload)?;
            fs::write(
                target_path.join(format!("{}.json", payload.name)),
                payload_json,
            )?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let input_path = env::current_dir()?.join("packages");
    let output_path = env::current_dir()?.join("dist");

    if output_path.exists() {
        fs::remove_dir_all(&output_path)?;
    }
    fs::create_dir_all(&output_path)?;

    let path = output_path.join("r");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    build_frameworks(&output_path)?;
    build_registry(&output_path)?;
    build_styles(&input_path, &output_path)?;
    build_styles_index(&output_path)?;
    build_themes(&output_path)?;

    log::info!("✅ Done!");

    Ok(())
}
