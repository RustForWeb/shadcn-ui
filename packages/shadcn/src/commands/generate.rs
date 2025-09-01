use anyhow::Result;
use clap::Args;
use shadcn_ui_component_generator::{ComponentConfig, ComponentGenerator, Framework, PropConfig};
use std::collections::HashMap;
use std::path::Path;

#[derive(Args, Debug)]
pub struct GenerateArgs {
    /// Name of the component to generate
    #[arg(short, long)]
    pub name: String,

    /// Target framework (leptos, yew, dioxus)
    #[arg(short, long, default_value = "leptos")]
    pub framework: String,

    /// Base CSS classes for the component
    #[arg(short, long)]
    pub classes: Option<String>,

    /// HTML tag to use for the component
    #[arg(short, long, default_value = "div")]
    pub tag: String,

    /// Output directory (defaults to packages/{framework}/)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Generate both default and new_york themes
    #[arg(long, default_value = "true")]
    pub themes: bool,

    /// Component description
    #[arg(short, long)]
    pub description: Option<String>,
}

pub async fn generate(args: GenerateArgs) -> Result<()> {
    println!("🔧 Generating {} component for {}...", args.name, args.framework);

    // Parse framework
    let framework = match args.framework.to_lowercase().as_str() {
        "leptos" => Framework::Leptos,
        "yew" => Framework::Yew,
        "dioxus" => Framework::Dioxus,
        f => {
            anyhow::bail!("Unsupported framework: {}. Supported: leptos, yew, dioxus", f);
        }
    };

    // Set up theme variants
    let theme_variants = if args.themes {
        vec!["default".to_string(), "new_york".to_string()]
    } else {
        vec!["default".to_string()]
    };

    // Create basic props (can be extended)
    let mut props = HashMap::new();
    props.insert(
        "children".to_string(),
        PropConfig {
            prop_type: match framework {
                Framework::Leptos => "Children".to_string(),
                Framework::Yew => "Html".to_string(),
                Framework::Dioxus => "VNode".to_string(),
            },
            optional: true,
            default_value: None,
            description: Some("Child components".to_string()),
        },
    );

    // Create component configuration
    let config = ComponentConfig {
        name: args.name.clone(),
        framework: framework.clone(),
        theme_variants,
        props,
        dependencies: vec![
            "tailwind_fuse".to_string(),
            match framework {
                Framework::Leptos => "leptos".to_string(),
                Framework::Yew => "yew".to_string(),
                Framework::Dioxus => "dioxus".to_string(),
            },
        ],
    };

    // Determine output directory
    let framework_name = match framework {
        Framework::Leptos => "leptos",
        Framework::Yew => "yew",
        Framework::Dioxus => "dioxus",
    };

    let output_dir = if let Some(output) = args.output {
        Path::new(&output).to_path_buf()
    } else {
        Path::new("packages").join(framework_name).to_path_buf()
    };

    // Generate the component files
    let generator = ComponentGenerator::new()?;
    shadcn_ui_component_generator::generator::Generator::generate_component_files(&config, &output_dir)?;

    println!("✅ Successfully generated {} component!", args.name);
    println!("📁 Files created in: {}", output_dir.join(&args.name).display());
    println!("📝 Next steps:");
    println!("   1. Add the component to your workspace members in Cargo.toml");
    println!("   2. Customize the component implementation");
    println!("   3. Add tests in the component directory");

    Ok(())
}