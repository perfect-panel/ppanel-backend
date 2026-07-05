//! 密码加密与验证工具。
//!
//! 从 `/root/project-moth/server/pkg/tool/encryption.go` 迁移而来。
//! 提供密码哈希（PBKDF2-SHA512）、MD5 编码以及多算法验证。

use pbkdf2::pbkdf2_hmac;
use sha2::{Digest, Sha256, Sha512};

const PBKDF2_SALT_LEN: usize = 16;
const PBKDF2_ITERATIONS: u32 = 100;
const PBKDF2_KEY_LEN: usize = 32;

#[derive(Debug)]
pub enum PasswordError {
    HashError(String),
    ParseError(String),
}

impl std::fmt::Display for PasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasswordError::HashError(msg) => write!(f, "Hash error: {}", msg),
            PasswordError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for PasswordError {}

/// 使用 PBKDF2-SHA512 对密码进行加盐编码。
///
/// 返回格式化字符串：`$pbkdf2-sha512$<salt>$<hash>`。
///
/// # 示例
/// ```
/// let encoded = password::encode_password("mypassword").unwrap();
/// assert!(encoded.starts_with("$pbkdf2-sha512$"));
/// ```
pub fn encode_password(password: &str) -> Result<String, PasswordError> {
    use rand::Rng;

    // 生成随机盐值
    let mut rng = rand::thread_rng();
    let salt: Vec<u8> = (0..PBKDF2_SALT_LEN).map(|_| rng.gen()).collect();

    // 使用 PBKDF2-HMAC-SHA512 对密码进行哈希
    let mut key = vec![0u8; PBKDF2_KEY_LEN];
    pbkdf2_hmac::<Sha512>(password.as_bytes(), &salt, PBKDF2_ITERATIONS, &mut key);

    // 将盐值和密钥编码为十六进制
    let salt_hex = hex::encode(&salt);
    let key_hex = hex::encode(&key);

    Ok(format!("$pbkdf2-sha512${}${}", salt_hex, key_hex))
}

/// 验证密码与编码后的密码哈希是否匹配。
///
/// 预期格式：`$pbkdf2-sha512$<salt>$<hash>`。
///
/// # 示例
/// ```
/// let encoded = password::encode_password("mypassword").unwrap();
/// assert!(password::verify_password("mypassword", &encoded));
/// assert!(!password::verify_password("wrongpass", &encoded));
/// ```
pub fn verify_password(password: &str, encoded: &str) -> bool {
    let parts: Vec<&str> = encoded.split('$').collect();
    if parts.len() < 4 || parts[1] != "pbkdf2-sha512" {
        return false;
    }

    let salt_hex = parts[2];
    let expected_hash_hex = parts[3];

    // 从十六进制解码盐值
    let salt = match hex::decode(salt_hex) {
        Ok(s) => s,
        Err(_) => return false,
    };

    // 计算哈希值
    let mut key = vec![0u8; PBKDF2_KEY_LEN];
    pbkdf2_hmac::<Sha512>(password.as_bytes(), &salt, PBKDF2_ITERATIONS, &mut key);

    // 与期望的哈希值进行比较
    let computed_hash_hex = hex::encode(&key);
    computed_hash_hex == expected_hash_hex
}

/// 计算输入字符串的 MD5 哈希值。
///
/// # 参数
/// * `s` - 输入字符串
/// * `uppercase` - 若为 true 则返回大写十六进制，否则返回小写
///
/// # 示例
/// ```
/// let hash = password::md5_encode("hello", false);
/// assert_eq!(hash, "5d41402abc4b2a76b9719d911017c592");
/// ```
pub fn md5_encode(s: &str, uppercase: bool) -> String {
    let digest = md5::compute(s.as_bytes());
    let result = format!("{:x}", digest);
    if uppercase {
        result.to_uppercase()
    } else {
        result
    }
}

/// 使用多种算法验证密码。
///
/// 支持的算法：
/// - `"md5"`：简单 MD5 哈希
/// - `"sha256"`：简单 SHA-256 哈希
/// - `"md5salt"`：MD5(密码 + 盐值)
/// - `"sha256salt"`：SHA-256(密码 + 盐值)，由 SSPanel 使用
/// - `"default"`：PBKDF2-SHA512（PPanel 默认）
/// - `"bcrypt"`：Bcrypt 哈希
///
/// # 参数
/// * `algo` - 算法名称
/// * `salt` - 盐值字符串（用于 `*salt` 算法）
/// * `password` - 明文密码
/// * `hash` - 期望的哈希值
///
/// # 示例
/// ```
/// let result = password::multi_password_verify("md5", "", "hello", "5d41402abc4b2a76b9719d911017c592");
/// assert!(result);
/// ```
pub fn multi_password_verify(algo: &str, salt: &str, password: &str, hash: &str) -> bool {
    match algo {
        "md5" => {
            let digest = md5::compute(password.as_bytes());
            let computed = format!("{:x}", digest);
            computed == hash
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            let computed = hex::encode(hasher.finalize());
            computed == hash
        }
        "md5salt" => {
            let input = format!("{}{}", password, salt);
            let digest = md5::compute(input.as_bytes());
            let computed = format!("{:x}", digest);
            computed == hash
        }
        "sha256salt" => {
            let input = format!("{}{}", password, salt);
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());
            let computed = hex::encode(hasher.finalize());
            computed == hash
        }
        "default" => verify_password(password, hash),
        "bcrypt" => bcrypt::verify(password, hash).unwrap_or(false),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_and_verify_password() {
        let password = "test_password_123";
        let encoded = encode_password(password).unwrap();

        eprintln!("Encoded password: {}", encoded);

        assert!(encoded.starts_with("$pbkdf2-sha512$"));
        assert!(verify_password(password, &encoded));
        assert!(!verify_password("wrong_password", &encoded));
    }

    #[test]
    fn test_md5_encode() {
        let input = "hello";
        let lowercase = md5_encode(input, false);
        let uppercase = md5_encode(input, true);

        assert_eq!(lowercase, "5d41402abc4b2a76b9719d911017c592");
        assert_eq!(uppercase, "5D41402ABC4B2A76B9719D911017C592");
    }

    #[test]
    fn test_multi_password_verify_md5() {
        let password = "hello";
        let hash = "5d41402abc4b2a76b9719d911017c592";
        assert!(multi_password_verify("md5", "", password, hash));
        assert!(!multi_password_verify("md5", "", "wrong", hash));
    }

    #[test]
    fn test_multi_password_verify_sha256() {
        let password = "hello";
        let hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
        assert!(multi_password_verify("sha256", "", password, hash));
    }

    #[test]
    fn test_multi_password_verify_md5salt() {
        let password = "hello";
        let salt = "world";
        // MD5("helloworld") = fc5e038d38a57032085441e7fe7010b0
        let hash = "fc5e038d38a57032085441e7fe7010b0";
        assert!(multi_password_verify("md5salt", salt, password, hash));
    }

    #[test]
    fn test_multi_password_verify_bcrypt() {
        let password = "test123";
        // 预先为 "test123" 生成的 bcrypt 哈希值
        let hash = bcrypt::hash(password, 4).unwrap();
        assert!(multi_password_verify("bcrypt", "", password, &hash));
        assert!(!multi_password_verify("bcrypt", "", "wrong", &hash));
    }

    #[test]
    fn test_multi_password_verify_default() {
        let password = "test123";
        let encoded = encode_password(password).unwrap();
        assert!(multi_password_verify("default", "", password, &encoded));
        assert!(!multi_password_verify("default", "", "wrong", &encoded));
    }
}
