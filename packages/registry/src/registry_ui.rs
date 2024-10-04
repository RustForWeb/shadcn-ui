use std::sync::LazyLock;

use crate::schema::{Registry, RegistryEntry, RegistryItemFile, RegistryItemType};

// TODO: Rustify dependencies and files

pub static UI: LazyLock<Registry> = LazyLock::new(|| {
    vec![RegistryEntry {
        name: "button".into(),
        r#type: RegistryItemType::Ui,
        description: None,
        dependencies: Some(vec!["@radix-ui/react-slot".into()]),
        dev_dependencies: None,
        registry_dependencies: None,
        files: Some(vec![RegistryItemFile {
            path: "ui/button.ts".into(),
            content: None,
            r#type: RegistryItemType::Ui,
            target: None,
        }]),
        tailwind: None,
        css_vars: None,
        source: None,
        category: None,
        subcategory: None,
        chunks: None,
        docs: None,
    }]
});
