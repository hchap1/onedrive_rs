use serde::Deserialize;
use reqwest::Client;

use crate::error::Res;
use crate::api::make_request;

// Constant endpoints
const BUNDLES_ENDPOINT: &str = "https://graph.microsoft.com/v1.0/me/drive/bundles";
const BUNDLES_FILTER: &str = "$filter=bundle/album%20ne%20null";

/// Paginated collections in a common endpoint
#[derive(Debug, Deserialize)]
struct GraphCollection<T> {
    value: Vec<T>,

    #[serde(rename = "@odata.nextLink")]
    next_link: Option<String>
}

/// DriveItem, parent class of Album
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveItem {

    // Stable and persistent ID
    id: String,
    pub name: String,

    // Cummulative size of all items in the album, bytes
    size: Option<i64>,
    pub web_url: Option<String>,
    bundle: Option<Bundle>,
}

#[derive(Debug, Deserialize)]
struct Bundle {

    // Number of children in the album
    child_count: Option<i32>,

    // If this is an album, this exists
    // Else, it is some other type of bundle
    album: Option<Album>
}

#[derive(Debug, Deserialize)]
struct Album {

    // Apparently unset when picked automatically
    cover_image_item_id: Option<String>
}

/// Retrieve albums from a drive
pub async fn get_albums(access_token: String) -> Res<Vec<DriveItem>> {
    let mut albums: Vec<DriveItem> = Vec::new();
    let mut next_url: Option<String> = Some(BUNDLES_ENDPOINT.to_string());

    while let Some(url) = &next_url {
        let graph_collection = make_request::<GraphCollection<DriveItem>>(
            &url,
            access_token.clone(),
            vec![(String::from("filter"), String::from(BUNDLES_FILTER))]
        ).await?;

        // Parse pagination
        next_url = graph_collection.next_link;
        albums.extend(graph_collection.value);
    }

    Ok(albums)
}
