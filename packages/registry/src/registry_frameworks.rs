use std::sync::LazyLock;

use crate::schema::{Framework, FrameworkName};

pub static FRAMEWORKS: LazyLock<Vec<Framework>> = LazyLock::new(|| {
    vec![
        Framework {
            name: FrameworkName::Dioxus,
            label: "Dioxus".into(),
            detect_dependencies: vec!["dioxus".into()],
        },
        Framework {
            name: FrameworkName::Leptos,
            label: "Leptos".into(),
            detect_dependencies: vec!["leptos".into()],
        },
        Framework {
            name: FrameworkName::Yew,
            label: "Yew".into(),
            detect_dependencies: vec!["yew".into()],
        },
    ]
});
