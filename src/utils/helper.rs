use serde::Serialize;

pub fn compress_to_json_bytes<T: Serialize>(obj: &T) -> Vec<u8> {
    serde_json::to_vec(obj).unwrap_or_else(|_| Vec::new())
}
