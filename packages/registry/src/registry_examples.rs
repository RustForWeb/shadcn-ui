use std::{collections::HashMap, sync::LazyLock};

use crate::schema::{FrameworkName, Registry};

pub static EXAMPLES: LazyLock<HashMap<FrameworkName, Registry>> = LazyLock::new(|| {
    HashMap::from([
        (FrameworkName::Dioxus, vec![]),
        (FrameworkName::Leptos, vec![]),
        (FrameworkName::Yew, vec![]),
    ])
});
