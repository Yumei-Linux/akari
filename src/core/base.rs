use std::{fs::create_dir, path::{PathBuf, Path}};
use crate::utils::exec::exec;

const WGET_LISTS: &str = "https://www.linuxfromscratch.org/lfs/downloads/stable-systemd/wget-list";
const MD5SUMS: &str = "https://www.linuxfromscratch.org/lfs/downloads/stable-systemd/md5sums";

pub struct Base {
    pub root_path: PathBuf,
    pub sources_path: PathBuf
}

impl Base {
    pub fn new(root: String) -> Self {
        let root_path = Path::new(&root);
        let mut sources_path_buf = PathBuf::new();

        sources_path_buf.push(root_path);
        sources_path_buf.push("sources");


        Self {
            root_path: PathBuf::from(root_path),
            sources_path: sources_path_buf
        }
    }

    fn download_sources(&self, sources_path_str: String) {
        println!("[I] Downloading sources from the lfs website...");
        println!("  * Downloading wget-lists...");
        exec(format!("wget {}", WGET_LISTS));
        println!("  * Downloading md5sums...");
        exec(format!("wget {} && mv -v ./md5sums {}", MD5SUMS, sources_path_str));
        println!("[I] Downloading sources and verifying them...");
        exec(format!("wget --input-file=wget-list --continue --directory-prefix={}", sources_path_str));
        exec(format!("rm -rvf wget-list"));
        exec(format!("pushd {}; md5sum -c md5sums; popd", sources_path_str));
        exec(format!("chown root:root {}/*", sources_path_str));
    }

    fn bootstrap(&self) {
        let path = self.root_path.as_path()
            .to_string_lossy()
            .to_string();

        println!("[I] Creating initial bootstrap...");
        exec(format!("mkdir -pv {0}/{{etc,var}} {0}/usr/{{bin,lib,sbin}}", path));
        exec(format!("for i in bin lib sbin; do ln -sv usr/$i {}/$i; done", path));
        exec(format!("mkdir -pv {}/{{tools,lib64}}", path));
        exec(String::from("groupadd yumei"));
        exec(String::from("useradd -s /bin/bash -g yumei -m -k /dev/null yumei"));
        println!("  * Write a password for the temporal user called yumei which will be used to compile the base system's tools");
        exec(String::from("passwd yumei"));
        exec(format!("chown -v yumei {}/{{usr{{,/*}},lib,var,etc,bin,sbin,tools}}", path));
        exec(format!("chown -v yumei {}/lib64", path));
    }

    pub fn setup(&self) {
        if !self.sources_path.is_dir() {
            let sources_path_str = self.sources_path.to_string_lossy().to_string();
            println!("[I] Creating sources dir at {}", sources_path_str);
            create_dir(self.sources_path.as_path()).unwrap_or_else(|error| {
                println!("Cannot create sources path: {}", error.to_string());
                std::process::exit(1);
            });

            exec(format!("chmod -v a+wt {}", sources_path_str));
            self.download_sources(sources_path_str);
        }

        self.bootstrap();
    }
}