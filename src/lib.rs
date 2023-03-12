mod platform;
pub use platform::App;

// passthru static
#[inline(always)]
pub fn list() -> Result<impl Iterator<Item = App>, Box<dyn std::error::Error>> {
    App::list()
}
