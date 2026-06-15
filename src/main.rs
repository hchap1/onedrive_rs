pub mod authentication;
pub mod error;
pub mod api;

#[tokio::main]
async fn main() {
    let permanent_token = authentication::oauth2::wrapper::acquire_refresh_token().await.unwrap();
    let access_token = authentication::oauth2::wrapper::acquire_session_token(permanent_token).await.unwrap().unwrap();
    println!("AccessToken: {}", access_token.access_token);

    let albums = api::albums::get_albums(access_token.access_token.clone()).await.unwrap();
    let album = albums.get(0).unwrap();

    println!("{} {} {} {:?}", album.name, album.id, album.num_items, album.cover_image_id);

    let photos = api::photos::get_photos(album.id.clone(), access_token.access_token.clone()).await.unwrap();
    println!("Retrived {} photos.", photos.len());

    for photo in &photos {
        println!("Photo: {} {} {} {}w {}h {}b", photo.id, photo.name, photo.created_date_time, photo.width, photo.height, photo.size);
    }
}
