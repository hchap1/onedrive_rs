use serde::Deserialize;

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
    name: String,

    folder: Option<Folder>,
    bundle: Option<Bundle>,
}

#[derive(Debug, Deserialize)]
struct Folder {
    #[serde(rename = "childCount")]
    child_count: usize
}

#[derive(Debug, Clone)]
pub struct AlbumMetaData {
    pub id: String,
    pub name: String,
    pub num_items: usize,
    pub cover_image_id: Option<String>
}

#[derive(Debug, Deserialize)]
struct Bundle {
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
pub async fn get_albums(access_token: String) -> Res<Vec<AlbumMetaData>> {
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

    Ok(
        albums
            .into_iter()
            .filter_map(
                |drive_item| {

                    let num_items = drive_item.folder?.child_count;
                    let cover_image_item_id = drive_item.bundle?.album?.cover_image_item_id;
                    
                    Some(
                        AlbumMetaData {
                            id: drive_item.id,
                            name: drive_item.name,
                            num_items: num_items as usize,
                            cover_image_id: cover_image_item_id
                        }
                    )
                }
            ).collect()
    )
}
