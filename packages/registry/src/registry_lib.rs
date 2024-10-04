use std::sync::LazyLock;

use crate::schema::{Registry, RegistryEntry, RegistryItemFile, RegistryItemType};

// TODO: Rustify dependencies and files

pub static LIB: LazyLock<Registry> = LazyLock::new(|| {
    vec![RegistryEntry {
        name: "utils".into(),
        r#type: RegistryItemType::Lib,
        description: None,
        dependencies: Some(vec!["clsx".into(), "tailwind-merge".into()]),
        dev_dependencies: None,
        registry_dependencies: None,
        files: Some(vec![RegistryItemFile {
            path: "lib/utils.ts".into(),
            content: None,
            r#type: RegistryItemType::Lib,
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
