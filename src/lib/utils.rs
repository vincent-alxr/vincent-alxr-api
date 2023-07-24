use md5::Md5;
use md5::Digest;

pub fn hash_password(password: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(password);
    let result = hasher.finalize();
    format!("{:x}", result) // Convert the MD5 hash to a hexadecimal string
}