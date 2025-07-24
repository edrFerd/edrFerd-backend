use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use std::env::home_dir;
use std::path::PathBuf;
use std::sync::OnceLock;
static PUBKEY: OnceLock<SigningKey> = OnceLock::new();
pub fn get_pubkey() -> SigningKey {
    PUBKEY.get_or_init(get_key).clone()
}
fn get_config_dir() -> PathBuf {
    let path = home_dir().unwrap().join(".config").join("edrFerd");
    path
}

use serde_json::json;
use std::fs::{self, File};
use std::io::{Read, Write};

fn get_key() -> SigningKey {
    let config_dir = get_config_dir();
    let key_file_path = config_dir.join("keys.json");

    if !key_file_path.exists() {
        return init_key().expect("初始化key失败");
    }

    let mut file = File::open(key_file_path).expect("无法打开key文件");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("无法阅读key文件");

    let json_data: serde_json::Value =
        serde_json::from_str(&contents).expect("无法解析key文件到json");
    let private_key_b64 = json_data["private_key"].a().expect("私钥未找到");
    let public_key_b64 = json_data["public_key"].as_str().expect("公钥未找到");
    let key_bytes = base64(private_key_b64).expect("Failed to decode private key from base64");
    SigningKey::from_bytes(&key_bytes)
}

pub fn init_key() -> Result<SigningKey, Box<dyn std::error::Error>> {
    let config_dir = get_config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }
    let key_file_path = config_dir.join("keys.json");

    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);

    let key_bytes = signing_key.to_bytes();
    let key_base64 = base64::encode(key_bytes);

    let json_data = json!({ "private_key": key_base64 });

    let mut file = File::create(key_file_path)?;
    file.write_all(json_data.to_string().as_bytes())?;

    Ok(signing_key)
}
