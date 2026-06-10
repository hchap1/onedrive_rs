pub mod authentication;
pub mod error;
pub mod api;

#[tokio::main]
async fn main() {
    let permanent_token = authentication::oauth2::wrapper::acquire_refresh_token().await.unwrap();
    let access_token = authentication::oauth2::wrapper::acquire_session_token(permanent_token).await.unwrap().unwrap();
    println!("AccessToken: {}", access_token.access_token);
}
