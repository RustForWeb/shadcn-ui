use std::{collections::HashMap, sync::LazyLock};

use crate::schema::{FrameworkName, Registry, RegistryEntry, RegistryItemFile, RegistryItemType};

pub static LIB: LazyLock<HashMap<FrameworkName, Registry>> = LazyLock::new(|| {
    HashMap::from([
        (
            FrameworkName::Dioxus,
            vec![
                // RegistryEntry {
                //     name: "utils".into(),
                //     r#type: RegistryItemType::Lib,
                //     description: None,
                //     dependencies: Some(vec!["tailwind_fuse".into()]),
                //     dev_dependencies: None,
                //     registry_dependencies: None,
                //     files: Some(vec![RegistryItemFile {
                //         path: "lib/utils.rs".into(),
                //         content: None,
                //         r#type: RegistryItemType::Lib,
                //         target: None,
                //     }]),
                //     tailwind: None,
                //     css_vars: None,
                //     source: None,
                //     category: None,
                //     subcategory: None,
                //     chunks: None,
                //     docs: None,
                // }
            ],
        ),
        (
            FrameworkName::Leptos,
            vec![RegistryEntry {
                name: "utils".into(),
                r#type: RegistryItemType::Lib,
                description: None,
                dependencies: Some(vec!["tailwind_fuse".into()]),
                dev_dependencies: None,
                registry_dependencies: None,
                files: Some(vec![RegistryItemFile {
                    path: "lib/utils.rs".into(),
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
            }],
        ),
        (
            FrameworkName::Yew,
            vec![
                // RegistryEntry {
                //     name: "utils".into(),
                //     r#type: RegistryItemType::Lib,
                //     description: None,
                //     dependencies: Some(vec!["tailwind_fuse".into()]),
                //     dev_dependencies: None,
                //     registry_dependencies: None,
                //     files: Some(vec![RegistryItemFile {
                //         path: "lib/utils.rs".into(),
                //         content: None,
                //         r#type: RegistryItemType::Lib,
                //         target: None,
                //     }]),
                //     tailwind: None,
                //     css_vars: None,
                //     source: None,
                //     category: None,
                //     subcategory: None,
                //     chunks: None,
                //     docs: None,
                // }
            ],
        ),
    ])
});
