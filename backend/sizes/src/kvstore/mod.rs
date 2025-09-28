use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait KvStore: Sized {
    fn get_bytes(&self, table: &str, key: impl AsRef<str>) -> Option<impl AsRef<[u8]>>;

    fn set<V>(&self, table: &str, key: impl AsRef<str>, value: V) -> crate::Result<()>
    where V: AsRef<[u8]>;

    fn get_string (self: &Self, table: &str, key: impl AsRef<str>) -> String {
        self.get_bytes(table, key).map_or(String::new(), |v| String::from_utf8_lossy(v.as_ref()).to_string())
    }

    fn set_json<T>(self: &Self, table: &str, key: impl AsRef<str>, value: &T) -> crate::Result<()>
    where T: Serialize + 'static {
        let data = serde_json::to_string(value);
        if data.is_err() {
            return Err(data.unwrap_err().to_string().into());
        }
        self.set(table, key, data.unwrap())
    }

    fn get_as<R>(self: &Self, table: &str, key: impl AsRef<str>) -> Option<R>
    where R: DeserializeOwned {
        if let Some(bytes) = self.get_bytes(table, key) {
            serde_json::from_slice(bytes.as_ref()).ok()
        } else {
            None
        }
    }

    fn foreach<F>(
        self: &Self,
        table: &str,
        key_prefix: impl AsRef<str>,
        limit: u32, callback: F)
    where F: FnMut(&str, &str);
}