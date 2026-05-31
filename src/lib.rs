use zed_extension_api::{self as zed};

struct MiseExtension;

impl zed::Extension for MiseExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(MiseExtension);
