mod static_handler;
mod url_shortener;
mod utils;

pub use static_handler::{root_handler, static_file_handler};
pub use url_shortener::url_redirect_handler;
pub use utils::version_handler;
pub use utils::healthz_handler;