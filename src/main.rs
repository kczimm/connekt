#![deny(clippy::all)]

use std::env::args;
use std::process::Command;

use home;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    servers: Option<Vec<ServerConfig>>,
}

impl Config {
    fn destination(&self, name: &str) -> Option<String> {
        if let Some(servers) = &self.servers {
            for server in servers {
                if let Some(this_name) = &server.name {
                    if this_name == name {
                        return Some(format!(
                            "{}@{}",
                            server.user.as_ref().unwrap(),
                            server.hostname.as_ref().unwrap(),
                        ));
                    }
                }
            }
        }
        None
    }

    fn connect(&self, name: &str) {
        if let Some(dest) = self.destination(name) {
            Command::new("ssh").arg(dest).status().unwrap();
        } else {
            println!("no server named {} in config", name);
        }
    }
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    name: Option<String>,
    user: Option<String>,
    hostname: Option<String>,
}

static CONFIG: &str = ".connekt.toml";

fn main() {
    let mut home = home::home_dir().unwrap();
    home.push(CONFIG);
    let toml =
        std::fs::read_to_string(&home).expect(&format!("{} not found.", home.to_str().unwrap()));
    let cfg: Config = toml::from_str(&toml).unwrap();
    let name = args().nth(1).unwrap();
    println!("Connecting to {}...", name);
    cfg.connect(&name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_server() {
        let toml_str = r#"
            [[servers]]
            name = "localhost"
            user = "johndoe"
            hostname = "127.0.0.1"
        "#;

        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(
            cfg.destination("localhost"),
            Some(String::from("johndoe@127.0.0.1"))
        )
    }
}
