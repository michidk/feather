use feather_core::prelude::*;
use std::fs::read_to_string;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub io: IO,
    pub proxy: Proxy,
    pub server: Server,
    pub gameplay: Gameplay,
    pub log: Log,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IO {
    pub compression_threshold: i32,
    pub io_worker_threads: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proxy {
    pub proxy_mode: ProxyMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub online_mode: bool,
    pub motd: String,
    pub max_players: i32,
    pub default_gamemode: Gamemode,
    pub difficulty: Difficulty,
    pub view_distance: u8,
    pub address: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gameplay {
    pub monster_spawning: bool,
    pub animal_spawning: bool,
    pub pvp: bool,
    pub nerf_spawner_mobs: bool,
    pub pvp_style: PvpStyle,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub log_level: String,
}

pub fn load() -> Result<Config, ()> {
    let config_content = read_to_string("config.toml").expect("Could not load configuration file");

    let config: Config = toml::from_str(&config_content).expect("Invalid configuration file");

    Ok(config)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProxyMode {
    None,
    Bungee,
    Velocity,
}
