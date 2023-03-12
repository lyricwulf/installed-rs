//! Generate a tree of all installed apps by publisher -> app name.
//! This will create a directory structure in `target/example-apps` with
//! a file for each app containing a metadata dump.

use std::{error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let apps = installed::list()?;
    for app in apps {
        let mut publisher = app.publisher();
        // replace some illegal characters
        let name = app.name().replace(':', "：");
        if name.is_empty() {
            continue;
        }
        if publisher.is_empty() {
            publisher = "Unknown".into();
        }
        let mut target_path = PathBuf::from("target/example-apps").join(&*publisher);
        fs::create_dir_all(&target_path)?;
        target_path = target_path.join(&*name);
        fs::write(&target_path, app.dump())?;
    }
    Ok(())
}
