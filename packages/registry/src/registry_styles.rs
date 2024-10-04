use serde::{Deserialize, Serialize};

use crate::schema::Style;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StyleDefinition {
    pub name: Style,
    pub label: &'static str,
}

pub const STYLES: [StyleDefinition; 2] = [
    StyleDefinition {
        name: Style::NewYork,
        label: "New York",
    },
    StyleDefinition {
        name: Style::Default,
        label: "Default",
    },
];
