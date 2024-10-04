use std::sync::LazyLock;

use crate::schema::Registry;

pub static HOOKS: LazyLock<Registry> = LazyLock::new(Vec::new);
