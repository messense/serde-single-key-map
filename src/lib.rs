//! Unwrap a single key map with serde.
//! ## Installation
//!
//! Add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! serde-single-key-map = "0.1"
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! #[derive(Debug, Deserialize)]
//! struct Project {
//!     name: String,
//!     #[serde(deserialize_with = "serde_single_key_map::deserialize")]
//!     items: Vec<Item>,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct Item {
//!     name: String,
//!     source: String,
//! }
//!
//! fn main() {
//!     let s = r#"{
//!         "name": "test",
//!         "items": {
//!             "item": [
//!             {
//!                 "name": "name",
//!                 "source": "name.rs"
//!             }
//!             ]
//!         }
//!         }"#;
//!     let project: Project = serde_json::from_str(s).expect("deserialize failed");
//!     assert_eq!(project.name, "test");
//!     assert_eq!(project.items.len(), 1);
//!     let item = &project.items[0];
//!     assert_eq!(item.name, "name");
//!     assert_eq!(item.source, "name.rs");
//! }
//! ```
//!
use std::fmt;
use std::marker::PhantomData;

use serde::de::{self, Deserializer, DeserializeOwned};

pub fn deserialize<'de, V, D>(d: D) -> Result<V, D::Error>
where
    D: Deserializer<'de>,
    V: DeserializeOwned,
{
    struct SingleKeyMapVisitor<V: DeserializeOwned>(PhantomData<V>);

    impl<'de, V> de::Visitor<'de> for SingleKeyMapVisitor<V>
    where
        V: DeserializeOwned,
    {
        type Value = V;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }

        #[inline]
        fn visit_map<T>(self, mut visitor: T) -> Result<V, T::Error>
        where
            T: de::MapAccess<'de>,
        {
            let item: Option<(String, V)> = visitor.next_entry()?;
            if let Some((_, value)) = item {
               return Ok(value);
            }
            Err(de::Error::custom("No single key value in map"))
        }
    }
    d.deserialize_map(SingleKeyMapVisitor(PhantomData))
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Project {
        name: String,
        #[serde(deserialize_with = "crate::deserialize")]
        items: Vec<Item>,
    }

    #[derive(Debug, Deserialize)]
    struct Item {
        name: String,
        source: String,
    }

    #[test]
    fn test_deserialize() {
        let s = r#"{
            "name": "test",
            "items": {
              "item": [
                {
                  "name": "name",
                  "source": "name.rs"
                }
              ]
            }
          }"#;
        let project: Project = serde_json::from_str(s).expect("deserialize failed");
        assert_eq!(project.name, "test");
        assert_eq!(project.items.len(), 1);
        let item = &project.items[0];
        assert_eq!(item.name, "name");
        assert_eq!(item.source, "name.rs");
    }

    #[test]
    #[should_panic]
    fn test_deserialize_with_multiple_keys() {
        let s = r#"{
            "name": "test",
            "items": {
              "item": [
                {
                  "name": "name",
                  "source": "name.rs"
                }
              ],
              "extra": "test"
            }
          }"#;
        let _project: Project = serde_json::from_str(s).expect("deserialize failed");
    }

    #[test]
    #[should_panic]
    fn test_deserialize_with_empty_map() {
        let s = r#"{
            "name": "test",
            "items": {}
          }"#;
        let _project: Project = serde_json::from_str(s).expect("deserialize failed");
    }
}
