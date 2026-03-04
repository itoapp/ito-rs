use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(i32)]
pub enum MangaStatus {
    Unknown = 0,
    Ongoing = 1,
    Completed = 2,
    Cancelled = 3,
    Hiatus = 4,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(i32)]
pub enum ContentRating {
    Safe = 0,
    Suggestive = 1,
    Nsfw = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manga {
    pub key: String,
    pub title: String,
    pub authors: Option<Vec<String>>,
    pub artist: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub cover: Option<String>,
    pub url: Option<String>,
    pub status: MangaStatus,
    pub content_rating: ContentRating,
    pub nsfw: i32,
    pub viewer: i32,
    pub chapters: Option<Vec<Chapter>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MangaPageResult {
    pub entries: Vec<Manga>,
    pub has_next_page: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chapter {
    pub key: String,
    pub title: Option<String>,
    pub volume: Option<f32>,
    pub chapter: Option<f32>,
    pub date_updated: Option<f64>,
    pub scanlator: Option<String>,
    pub url: Option<String>,
    pub lang: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PageContent {
    Url(String),
    Text(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub index: i32,
    pub content: PageContent,
    pub has_description: bool,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Listing {
    pub id: String,
    pub name: String,
    pub kind: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FilterValue {
    Boolean(bool),
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterItem {
    pub type_name: String, // swift `type`
    pub name: String,
    pub value: FilterValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LinkValue {
    Url(String),
    Manga(Manga),
    Listing(Listing),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    pub title: String,
    pub value: Option<LinkValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MangaWithChapter {
    pub manga: Manga,
    pub chapter: Chapter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HomeComponentValue {
    Scroller(Vec<Manga>, Option<Listing>),
    MangaList(bool, Option<i32>, Vec<Manga>, Option<Listing>),
    MangaChapterList(Option<i32>, Vec<MangaWithChapter>, Option<Listing>),
    BigScroller(Vec<Manga>, Option<f32>),
    Filters(Vec<FilterItem>),
    Links(Vec<Link>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomeComponent {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub value: HomeComponentValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomeLayout {
    pub components: Vec<HomeComponent>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_manga() {
        let hex_str = "05665830594a1d5468652047726561746573742045737461746520446576656c6f706572010307424b5f4d6f6f6e0c4b494d204879756e20536f6f0c4c4545204879756e2d4d696e0001e5035768656e20636976696c20656e67696e656572696e672073747564656e74205375686f204b696d2066616c6c732061736c6565702072656164696e6720612066616e74617379206e6f76656c2c2068652077616b657320757020617320612063686172616374657220696e2074686520626f6f6b21205375686f206973206e6f7720696e2074686520626f6479206f66204c6c6f79642046726f6e746572612c2061206c617a79206e6f626c652077686f206c6f76657320746f206472696e6b2c20616e642077686f73652066616d696c7920697320696e2061206d6f756e7461696e206f6620646562742e205573696e672068697320656e67696e656572696e67206b6e6f776c656467652c205375686f2064657369676e7320696e76656e74696f6e7320746f20617665727420746865207465727269626c65206675747572652074686174206c69657320696e207761697420666f722068696d2e2057697468207468652068656c70206f662061206769616e742068616d737465722c2061206b6e696768742c20616e642074686520776f726c64e2809973206d616769632c2063616e205375686f2064696720686973206e65772066616d696c79206f7574206f66206465627420616e64206275696c64206120626574746572206675747572653f010506416374696f6e0746616e7461737906436f6d656479054472616d6109416476656e74757265013468747470733a2f2f617473752e6d6f652f7374617469632f706f73746572732f6458455245774f574d786b763673624a2e6a7067011c68747470733a2f2f617473752e6d6f652f6d616e67612f665830594a0400000000";
        let bytes = hex::decode(hex_str).unwrap();
        match postcard::from_bytes::<'_, Manga>(&bytes) {
            Ok(m) => println!("Success: {:?}", m.title),
            Err(e) => panic!("Error decoding Manga: {:?}", e),
        }
    }
}
