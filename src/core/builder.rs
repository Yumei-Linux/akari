use std::path::PathBuf;

use crate::utils::exec;
use super::base::Base;

pub struct Builder {
    pub root_path: PathBuf,
    pub sources_path: PathBuf
}

impl Builder {
    pub fn new(root_path: PathBuf, sources_path: PathBuf) -> Self {
        Self {
            root_path,
            sources_path
        }
    }

    /// Creates a new `Builder` by using a `Base` as reference.
    /// Note that this function will move your `Base` instance so you'll loose it.
    pub fn from_base(base: Base) -> Self {
        Builder::new(base.root_path, base.sources_path)
    }

    /// The run function calls the main.sh entry script of a builder pass
    /// Note that the given `pass` argument represents the dirname of the pass-builder
    /// provided by the `akari-builders` package.
    pub fn run(&self, pass: &str) {
        let mut entry_script = PathBuf::from(&self.sources_path);
        entry_script.push("builders");
        entry_script.push("pass1");
        entry_script.push("main.sh");

        if !entry_script.is_file() {
            println!("[Fatal]: Cannot invoke entry script for pass: {}. This error shouldn't happen.", pass);
            std::process::exit(1);
        }

        let root = self.root_path.as_path()
            .to_string_lossy()
            .to_string();

        let entry_script_path = entry_script.as_path()
            .to_string_lossy()
            .to_string();

        println!("[I] Executing entry script for pass: {} at {}", pass, entry_script_path);
        exec::exec(format!("su -c yumei -c \"cd {0}/sources/builders/{1}; bash main.sh {0}\"", root, pass));
    }

    /// This function installs https://github.com/Yumei-Linux/akari-builders.git into
    /// <yumei-root>/sources/builders, the builders are a collection of shell scripts
    /// which should be executed in order to build every package.
    pub fn setup_builders(&self) {
        let mut builders_path = PathBuf::from(&self.sources_path);
        builders_path.push("builders");

        if builders_path.is_dir() {
            return;
        }

        let str_path = builders_path.as_path()
            .to_string_lossy()
            .to_string();

        if builders_path.is_dir() {
            std::fs::remove_dir_all(builders_path).unwrap_or_else(|error| {
                println!(
                    "Cannot remove directory: {}: {}",
                    str_path,
                    error.to_string()
                );
            });
        }

        println!("[I] Downloading builders at {}", str_path);
        println!("  * Using git, be sure to have it installed");
        exec::exec(format!("git clone https://github.com/Yumei-Linux/akari-builders.git {}", str_path));
        exec::exec(format!("chown -R yumei {}", str_path));
    }
}