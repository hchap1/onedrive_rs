use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::error::{Error, Res};
use crate::api::{OnedriveError, make_request};

// Constant endpoints
const PHOTOS_ENDPOINT: &str = "https://graph.microsoft.com/v1.0/me/drive/items/";

/// Paginated collections in a common endpoint
#[derive(Debug, Deserialize)]
struct GraphCollection<T> {
    value: Vec<T>,

    #[serde(rename = "@odata.nextLink")]
    next_link: Option<String>
}

/// DriveItem, parent class of Photo
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveItem {

    // Stable and persistent ID
    id: String,
    name: String,
    created_date_time: String,
    location: Option<Location>,
    image: Option<Image>,
    photo: Option<Photo>,
    size: usize
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    taken_date_time: String
}

#[derive(Debug, Deserialize)]
pub struct Image {
    width: usize,
    height: usize
}


#[derive(Debug, Clone, Deserialize)]
struct Location {
    latitude: f64,
    longitude: f64,
    altitude: f64
}

#[derive(Debug, Clone)]
pub struct PhotoMetaData {
    pub id: String,
    pub name: String,
    pub created_date_time: DateTime<FixedOffset>,
    pub width: usize,
    pub height: usize,
    pub location: Option<Location>,
    pub size: usize
}

impl TryFrom<DriveItem> for PhotoMetaData {
    type Error = Error;
    fn try_from(drive_item: DriveItem) -> Res<PhotoMetaData> {
        let datetime_string = match drive_item.photo {
            Some(photo) => photo.taken_date_time,
            None => drive_item.created_date_time
        };

        let datetime = DateTime::parse_from_rfc3339(&datetime_string)?;
        let image = drive_item.image.ok_or(Error::from(OnedriveError::MissingImageMetadata))?;

        Ok(
            PhotoMetaData {
                id: drive_item.id,
                name: drive_item.name,
                created_date_time: datetime,
                width: image.width,
                height: image.height,
                location: drive_item.location,
                size: drive_item.size
            }
        )
    }
}

/// Retrieve a singular photo by ID
pub async fn get_photo(photo_id: String, access_token: String) -> Res<PhotoMetaData> {
    Ok(
        make_request::<DriveItem>(
            &format!("{PHOTOS_ENDPOINT}{photo_id}"),
            access_token,
            vec![]
        ).await?
        .try_into()?
    )
}

/// Retrieve photos from an album
pub async fn get_photos(album_id: String, access_token: String) -> Res<Vec<PhotoMetaData>> {
    let mut drive_items: Vec<DriveItem> = Vec::new();
    let mut next_url: Option<String> = Some(format!("{PHOTOS_ENDPOINT}{album_id}/children"));

    while let Some(url) = &next_url {
        let graph_collection = make_request::<GraphCollection<DriveItem>>(
            &url,
            access_token.clone(),
            vec![]
        ).await?;

        // Parse pagination
        next_url = graph_collection.next_link;
        drive_items.extend(graph_collection.value);
    }

    Ok(
        drive_items
            .into_iter()
            .filter_map(
                |drive_item| drive_item
                    .try_into()
                    .ok()
            ).collect()
    )
}
