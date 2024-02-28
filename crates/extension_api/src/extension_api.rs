pub struct Guest;
pub use wit::*;

pub type Result<T, E = String> = core::result::Result<T, E>;

pub trait Extension: Send + Sync {
    fn get_language_server_command(
        &self,
        config: wit::LanguageServerConfig,
        worktree: &wit::Worktree,
    ) -> Result<Command>;
}

pub fn download_file(
    url: &str,
    output_filename: &str,
    file_type: wit::DownloadedFileType,
) -> Result<()> {
    wit::download_file(url, output_filename, file_type).map(|_| ())
}

#[macro_export]
macro_rules! register_extension {
    ($extension:path) => {
        #[export_name = "init-extension"]
        pub extern "C" fn __init_extension() {
            zed_extension_api::register_extension(&$extension);
        }
    };
}

#[doc(hidden)]
pub fn register_extension(extension: &'static dyn Extension) {
    unsafe { EXTENSION = Some(extension) };
}

fn extension() -> &'static dyn Extension {
    unsafe { EXTENSION.unwrap() }
}

static mut EXTENSION: Option<&'static dyn Extension> = None;

mod wit {
    wit_bindgen::generate!({
        exports: { world: super::Component },
        skip: ["init-extension"]
    });
}

struct Component;

impl wit::Guest for Component {
    fn get_language_server_command(
        config: wit::LanguageServerConfig,
        worktree: &wit::Worktree,
    ) -> Result<wit::Command> {
        extension().get_language_server_command(config, worktree)
    }
}
