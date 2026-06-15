# onedrive-albums

A Rust client for accessing OneDrive photos and albums via the Microsoft Graph API.

`onedrive-albums` provides:

- An OAuth2 authentication flow for acquiring and refreshing access tokens
- Retrieval of the authenticated user's **albums**
- Retrieval of **photos within an album**
- Retrieval of a **single photo** by ID

## Status

This crate is under active development. APIs may change without notice until a `1.0` release.

## Usage

```rust
pub mod authentication;
pub mod error;
pub mod api;

#[tokio::main]
async fn main() {
    // Acquire a long-lived refresh token (interactive on first run)
    let permanent_token = authentication::oauth2::wrapper::acquire_refresh_token()
        .await
        .unwrap();

    // Exchange it for a short-lived access token
    let access_token = authentication::oauth2::wrapper::acquire_session_token(permanent_token)
        .await
        .unwrap()
        .unwrap();

    println!("AccessToken: {}", access_token.access_token);

    // List the user's albums
    let albums = api::albums::get_albums(access_token.access_token.clone())
        .await
        .unwrap();

    let album = albums.get(0).unwrap();
    println!(
        "{} {} {} {:?}",
        album.name, album.id, album.num_items, album.cover_image_id
    );

    // Retrieve all photos within an album
    let photos = api::photos::get_photos(album.id.clone(), access_token.access_token.clone())
        .await
        .unwrap();

    println!("Retrieved {} photos.", photos.len());

    for photo in &photos {
        println!(
            "Photo: {} {} {} {}w {}h {}b",
            photo.id, photo.name, photo.created_date_time, photo.width, photo.height, photo.size
        );
    }
}
```

## Authentication

Authentication is handled via the OAuth2 authorization code flow against the Microsoft identity platform:

1. `acquire_refresh_token()` — performs the initial interactive login and returns a refresh token that can be persisted for future sessions.
2. `acquire_session_token(refresh_token)` — exchanges the refresh token for a short-lived access token used to authorize Microsoft Graph requests.

You'll need to register an application in the [Azure portal](https://portal.azure.com) with the appropriate Microsoft Graph delegated permissions (`Files.Read`, `Files.Read.All`, or similar, depending on the scopes you require).

## API Overview

### Albums

```rust
api::albums::get_albums(access_token: String) -> Res<Vec<Album>>
```

Returns the authenticated user's albums, including name, ID, item count, and cover image reference.

### Photos in an Album

```rust
api::photos::get_photos(album_id: String, access_token: String) -> Res<Vec<PhotoMetaData>>
```

Returns metadata for all photos within the given album, handling pagination transparently.

### Single Photo

```rust
api::photos::get_photo(photo_id: String, access_token: String) -> Res<PhotoMetaData>
```

Retrieves metadata for a single photo by its OneDrive item ID.

### PhotoMetaData

Each photo is returned as a `PhotoMetaData` struct containing:

- `id` — OneDrive item ID
- `name` — file name
- `created_date_time` — capture or creation timestamp
- `width` / `height` — image dimensions
- `location` — optional GPS coordinates (latitude, longitude, altitude)
- `size` — file size in bytes

## Requirements

- Rust 2021 edition or later
- Tokio async runtime
- A registered Azure AD application with Microsoft Graph delegated permissions

## License

License to be determined.

## Contributing

Issues and pull requests are welcome.
