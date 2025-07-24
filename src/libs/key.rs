#[allow(unused)]
use ed25519_dalek::SigningKey;
use rand::TryRngCore;
use rand::rngs::OsRng;
use std::path::PathBuf;
use std::sync::OnceLock;

static PUBKEY: OnceLock<SigningKey> = OnceLock::new();

pub fn get_key() -> SigningKey {
    PUBKEY.get_or_init(get_key_from_file).clone()
}

fn get_config_dir() -> PathBuf {
    PathBuf::from("./config")
}

use anyhow::Result;
use std::fs::{self, File};
use std::io::{Read, Write};

fn get_key_from_file() -> SigningKey {
    let config_dir = get_config_dir();
    let key_file_path = config_dir.join("keys.json");

    if !key_file_path.exists() {
        return init_key().expect("初始化key失败");
    }

    let mut file = File::open(key_file_path).expect("无法打开key文件");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("无法阅读key文件");

    let key_bytes: [u8; 32] = serde_json::from_str(&contents).expect("无法解析key文件到json");

    SigningKey::from_bytes(&key_bytes)
}

fn init_key() -> Result<SigningKey> {
    let config_dir = get_config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }
    let key_file_path = config_dir.join("keys.json");

    let secret_bytes: [u8; 32] = {
        let mut csprng = OsRng;
        let mut buf = [0_u8; 32];
        csprng.try_fill_bytes(&mut buf)?;
        buf
    };

    let mut file = File::create(key_file_path)?;
    file.write_all(serde_json::to_string_pretty(&secret_bytes)?.as_bytes())?;

    Ok(SigningKey::from_bytes(&secret_bytes))
}
