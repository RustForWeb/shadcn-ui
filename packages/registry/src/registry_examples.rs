use std::sync::LazyLock;

use crate::schema::Registry;

pub static EXAMPLES: LazyLock<Registry> = LazyLock::new(Vec::new);
