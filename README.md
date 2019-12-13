# serde-single-key-map

[![GitHub Actions](https://github.com/messense/serde-single-key-map/workflows/CI/badge.svg)](https://github.com/messense/serde-single-key-map/actions?query=workflow%3ACI)
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

## Motivation

The primary use case for this crate is to remove a unnessery intermediary struct when deserializing. For example, consider the following XML:

```xml
<GetBucketTaggingOutput>
   <TagSet>
      <Tag>
         <Key>key1</Key>
         <Value>value1</Value>
      </Tag>
      <Tag>
         <Key>key2</Key>
         <Value>value2</Value>
      </Tag>
   </TagSet>
</GetBucketTaggingOutput>
```

There is only one `TagSet` element in `GetBucketTaggingOutput`, so naturally you'd write the following code to represent it:

```rust
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct GetBucketTaggingOutput {
    #[serde(rename = "TagSet")]
    pub tag_set: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Tag {
    /// Name of the tag.
    #[serde(rename = "Key")]
    pub key: String,
    /// Value of the tag.
    #[serde(rename = "Value")]
    pub value: String,
}
```

But it won't work, you'd have to add a intermediary `TagSet` struct to make it work:

```rust
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct GetBucketTaggingOutput {
    #[serde(rename = "TagSet")]
    tag_set: TagSet,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TagSet {
    #[serde(rename = "Tag")]
    tags: Vec<Tag>,
}
```

with `serde-single-key-map` you can make it work with a single line of change:

```diff
 #[derive(Serialize, Deserialize, Debug, PartialEq)]
 struct GetBucketTaggingOutput {
+    #[serde(deserialize_with = "serde_single_key_map::deserialize")]
     #[serde(rename = "TagSet")]
     pub tag_set: Vec<Tag>,
 }
```


## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
