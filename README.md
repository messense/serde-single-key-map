# serde-single-key-map

[![Crates.io](https://img.shields.io/crates/v/serde-single-key-map.svg)](https://crates.io/crates/serde-single-key-map)
[![docs.rs](https://docs.rs/serde-single-key-map/badge.svg)](https://docs.rs/serde-single-key-map/)

Unwrap a single key map with serde.

## Installation

Add it to your `Cargo.toml`:

```toml
[dependencies]
serde-single-key-map = "0.1"
```

## Usage

```rust
#[derive(Debug, Deserialize)]
struct Project {
    name: String,
    #[serde(deserialize_with = "serde_single_key_map::deserialize")]
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    name: String,
    source: String,
}

fn main() {
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
```

## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
