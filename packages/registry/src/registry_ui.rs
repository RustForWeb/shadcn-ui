use std::{collections::HashMap, sync::LazyLock};

use crate::schema::{FrameworkName, Registry};

pub static UI: LazyLock<HashMap<FrameworkName, Registry>> = LazyLock::new(|| {
    HashMap::from([
        (FrameworkName::Dioxus, vec![]),
        (
            FrameworkName::Leptos,
            vec![],
            // RegistryEntry {
            //     name: "button".into(),
            //     r#type: RegistryItemType::Ui,
            //     description: None,
            //     dependencies: Some(vec!["radix-leptos-slot".into()]),
            //     dev_dependencies: None,
            //     registry_dependencies: None,
            //     files: Some(vec![RegistryItemFile {
            //         path: "ui/button.rs".into(),
            //         content: None,
            //         r#type: RegistryItemType::Ui,
            //         target: None,
            //     }]),
            //     tailwind: None,
            //     css_vars: None,
            //     source: None,
            //     category: None,
            //     subcategory: None,
            //     chunks: None,
            //     docs: None,
            // },
        ),
        (FrameworkName::Yew, vec![]),
    ])
});
