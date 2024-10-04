use std::sync::LazyLock;

use crate::schema::Registry;

pub static CHARTS: LazyLock<Registry> = LazyLock::new(Vec::new);
