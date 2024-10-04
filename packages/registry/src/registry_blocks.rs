use std::sync::LazyLock;

use crate::schema::Registry;

pub static BLOCKS: LazyLock<Registry> = LazyLock::new(Vec::new);
