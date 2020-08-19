pub use log::{debug, info, warn, trace};

pub fn init() {
    env_logger::init();
}
