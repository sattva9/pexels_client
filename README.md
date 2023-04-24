# Pexels Client

The pexels_client crate is a wrapper around Pexels API.

[![Crates.io](https://img.shields.io/crates/v/http.svg)](https://crates.io/crates/pexels_client)
[![Documentation](https://docs.rs/http/badge.svg)][dox]

More information about this crate can be found in the [crate
documentation][dox].

[dox]: https://docs.rs/pexels_client

## Usage

To use `pexels_client`, first add this to your `Cargo.toml`:

```toml
[dependencies]
pexels_client = "0.1.0"
```

## Examples

Create a Pexels Client:

```rust
use pexels_client::PexelsClient;

fn main() {
    let client = PexelsClient::new("auth_key".to_string()).unwrap();
}
```

Search for Photos:

```rust
use pexels_client::{PexelsClient, photos::PhotoSearchQuery};

async fn test() {
    let client = PexelsClient::new("auth_key".to_string()).unwrap();
    let response = client
        .photo_search(PhotoSearchQuery::new("tigers".to_string()))
        .await
        .unwrap();
}
```

Search for photos with filters:

```rust
use pexels_client::{*, photos::PhotoSearchQuery};

async fn test() {
    let client = PexelsClient::new("auth_key".to_string()).unwrap();
    let query = PhotoSearchQuery::new("tigers".to_string())
        .orientation(Orientation::Portrait)
        .size(Size::Large)
        .color(Color::Blue)
        .per_page(5);
    let response = client.photo_search(query).await.unwrap();
}
```

# License

Licensed under 

- MIT license ([LICENSE-MIT](LICENSE))