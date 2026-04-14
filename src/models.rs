use serde::{Deserialize, Serialize};

pub mod anime;
pub mod manga;
pub mod novel;

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
    pub headers: Option<std::collections::HashMap<String, String>>,
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
    Manga(manga::Manga),
    Anime(anime::Anime),
    Novel(novel::Novel),
    Listing(Listing),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    pub title: String,
    pub value: Option<LinkValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MangaWithChapter {
    pub manga: manga::Manga,
    pub chapter: manga::Chapter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimeWithEpisode {
    pub anime: anime::Anime,
    pub episode: anime::Episode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NovelWithChapter {
    pub novel: novel::Novel,
    pub chapter: novel::Chapter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HomeComponentValue {
    Scroller(Vec<manga::Manga>, Option<Listing>),
    MangaList(bool, Option<i32>, Vec<manga::Manga>, Option<Listing>),
    MangaChapterList(Option<i32>, Vec<MangaWithChapter>, Option<Listing>),
    AnimeScroller(Vec<anime::Anime>, Option<Listing>),
    AnimeList(bool, Option<i32>, Vec<anime::Anime>, Option<Listing>),
    AnimeEpisodeList(Option<i32>, Vec<AnimeWithEpisode>, Option<Listing>),
    BigScroller(Vec<manga::Manga>, Option<f32>),
    AnimeBigScroller(Vec<anime::Anime>, Option<f32>),
    NovelScroller(Vec<novel::Novel>, Option<Listing>),
    NovelList(bool, Option<i32>, Vec<novel::Novel>, Option<Listing>),
    NovelChapterList(Option<i32>, Vec<NovelWithChapter>, Option<Listing>),
    NovelBigScroller(Vec<novel::Novel>, Option<f32>),
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SettingType {
    Toggle,
    Text,
    Picker,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Setting {
    Toggle {
        id: String,
        name: String,
        summary: Option<String>,
        default_value: bool,
    },
    Text {
        id: String,
        name: String,
        summary: Option<String>,
        default_value: String,
    },
    Picker {
        id: String,
        name: String,
        summary: Option<String>,
        options: Vec<String>,
        default_value: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsSchema {
    pub settings: Vec<Setting>,
}
