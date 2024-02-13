use clap::Parser;
use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = ".config/bm/config.json";

#[derive(Debug, Serialize, Deserialize)]
struct Bookmark {
    name: String,
    url: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    bookmarks: Vec<Bookmark>,
}

impl Config {
    fn path() -> std::path::PathBuf {
        std::path::Path::new(&std::env::var("HOME").unwrap())
            .join(CONFIG_PATH)
            .to_path_buf()
    }

    fn load() -> Self {
        match std::fs::read_to_string(Self::path()) {
            Ok(raw_cfg) => serde_json::from_str(&raw_cfg).unwrap(),
            Err(_) => Self::default(),
        }
    }

    fn save(&self) -> std::io::Result<()> {
        let fpath = Self::path();
        std::fs::create_dir_all(fpath.parent().unwrap())?;
        let raw_cfg = serde_json::to_string_pretty(self).expect("serialization should succeed");
        std::fs::write(fpath, raw_cfg)
    }
}

#[derive(clap::Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    fn run(self, cfg: &mut Config) {
        match self.command {
            Command::Add { name, url } => {
                cfg.bookmarks.push(Bookmark { name, url });
                cfg.save().expect("save should succeed");
            }
            Command::List => {
                cfg.bookmarks.sort_by_key(|bm| bm.name.clone());
                for bookmark in &cfg.bookmarks {
                    println!("{}", bookmark.name)
                }
            }
            Command::Open { name } => match cfg.bookmarks.iter().find(|bm| bm.name == name) {
                Some(bookmark) => {
                    let _ = std::process::Command::new("xdg-open")
                        .arg(&bookmark.url)
                        .spawn();
                }
                None => println!("could not find \"{}\"", name),
            },
            Command::Remove { name } => {
                if let Some(idx) = cfg
                    .bookmarks
                    .iter()
                    .enumerate()
                    .find(|(_, bm)| bm.name == name)
                    .map(|(idx, _)| idx)
                {
                    cfg.bookmarks.remove(idx);
                    cfg.save().expect("save should succeed");
                }
            }
        }
    }
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Add { name: String, url: String },
    Remove { name: String },
    List,
    Open { name: String },
}

fn main() {
    let mut cfg = Config::load();
    Cli::parse().run(&mut cfg)
}
