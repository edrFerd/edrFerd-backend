use crate::ARGS;
use ed25519_dalek::SigningKey;
use rand::TryRngCore;
use rand::rngs::OsRng;
use std::path::PathBuf;
use std::sync::OnceLock;
use log::info;

/// 全局签名密钥实例，使用 `OnceLock` 实现懒初始化。
static PUBKEY: OnceLock<SigningKey> = OnceLock::new();

/// 获取全局签名密钥。
///
/// 首次调用时会从文件中加载密钥，如果文件不存在则创建新密钥。
///
/// 返回值：`SigningKey` 签名密钥的克隆
pub fn get_key() -> SigningKey {
    PUBKEY
        .get_or_init(|| {
            if ARGS.get().unwrap().random_key {
                info!("使用随机生成的key");
                return generate_random_key().expect("生成随机key失败");
            }
            get_key_from_file()
        })
        .clone()
}

/// 生成一个随机的签名密钥，不保存到文件。
fn generate_random_key() -> Result<SigningKey> {
    let mut csprng = OsRng;
    let secret_bytes: [u8; 32] = {
        let mut buf = [0_u8; 32];
        csprng.try_fill_bytes(&mut buf)?;
        buf
    };
    Ok(SigningKey::from_bytes(&secret_bytes))
}

/// 获取配置目录路径。
///
/// 返回值：`PathBuf` 配置目录的路径
fn get_config_dir() -> PathBuf {
    PathBuf::from("./config")
}

use anyhow::Result;
use std::fs::{self, File};
use std::io::{Read, Write};

/// 从文件中加载签名密钥。
///
/// 如果密钥文件不存在，则创建新的密钥。
/// 如果文件存在，则读取并解析密钥数据。
///
/// 返回值：`SigningKey` 从文件加载的签名密钥
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

/// 初始化新的签名密钥并保存到文件。
///
/// 生成一个新的随机密钥，创建配置目录（如果不存在），
/// 并将密钥以 JSON 格式保存到文件中。
///
/// 返回值：`Result<SigningKey>` 新创建的签名密钥
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
