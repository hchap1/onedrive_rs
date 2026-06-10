use std::time::UNIX_EPOCH;
use std::time::SystemTime;

use crate::authentication::oauth2::api::TokenSet;
use crate::authentication::oauth2::refresh::refresh_tokenset;
use crate::authentication::oauth2::hashcodes::generate_csrf;
use crate::authentication::oauth2::hashcodes::generate_pkce;
use crate::authentication::oauth2::api::post_oauth2_code;
use crate::authentication::callback::server::run_server;
use crate::authentication::callback::client::launch_oauth2;
use crate::error::Res;

/// Struct containing semi-permanent refresh token, and UNIX timestamp of expiry
pub struct SemiPermanentToken {
    pub token: String,
    pub expiration: usize
}

/// Acquire a semi-permanent token allowing future sessions to be started without requiring user approval every time
pub async fn acquire_refresh_token() -> Res<SemiPermanentToken> {

    // Perform full authentication flow
    let csrf = generate_csrf();
    let (pkce_verifier, pkce_challenge) = generate_pkce();
    launch_oauth2(csrf.clone(), pkce_challenge).await?;
    let temporary_code = run_server(csrf).await?;
    let tokenset = post_oauth2_code(temporary_code, pkce_verifier).await?;

    // Return necessary data
    Ok(SemiPermanentToken {
        token: tokenset.refresh_token,
        expiration: tokenset.absolute_expiration
    })
}

/// Use the semi-permanent token to acquire a temporary session token to actually interact with the API
/// Returns None if the token has expired, which should be used as a sign the application should trigger the acquire_refresh_token function
/// The new semi-permanent token should overwrite the old one, as should the new expiry
pub async fn acquire_session_token(refresh_token: SemiPermanentToken) -> Option<Res<TokenSet>> {

    if SystemTime::now()
        .duration_since(UNIX_EPOCH).ok()?
        .as_secs() as usize >= refresh_token.expiration
    {
        None
    } else {
        Some(refresh_tokenset(refresh_token.token).await)
    }
}
