use serde::Deserialize;
use reqwest::Client;

// Constant endpoints
const BUNDLES_ENDPOINT: &str = "https://graph.microsoft.com/v1.0/me/drive/bundles";
const BUNDLES_FILTER: &str = "$filter=bundle/album%20ne%20null";

/// Paginated collections in a common endpoint
#[derive(Debug, Deserialize)]
struct GraphCollection<T> {
    #[serde(rename = "value")]
    value: Vec<T>,

    #[serde(rename = "@odata.nextLink")]
    next_link: Option<String>
}

/// DriveItem, parent class of Album
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DriveItem {

    // Stable and persistent ID
    id: String,
    name: String,

    // Cummulative size of all items in the album, bytes
    size: Option<i64>,
    web_url: Option<String>,
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
