use crate::models::{FilterItem, HomeLayout, Listing, Manga, MangaPageResult, Chapter, Page};

pub trait MangaProvider {
    fn get_home() -> HomeLayout {
        HomeLayout { components: Vec::new() }
    }
    
    fn get_manga_list(listing: Listing, page: i32) -> MangaPageResult;
    
    fn get_search_manga_list(query: String, page: i32, filters: Vec<FilterItem>) -> MangaPageResult;
    
    fn get_manga_update(manga: Manga, needs_details: bool, needs_chapters: bool) -> Manga;
    
    fn get_page_list(manga: Manga, chapter: Chapter) -> Vec<Page>;
}

#[macro_export]
macro_rules! export_plugin {
    ($type:ty) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn get_home() -> i64 {
            let res = <$type as $crate::provider::MangaProvider>::get_home();
            let bytes = $crate::postcard::to_allocvec(&res).unwrap().into_boxed_slice();
            let ptr = bytes.as_ptr() as u64;
            let len = bytes.len() as u64;
            let _ = Box::into_raw(bytes);
            ((ptr << 32) | len) as i64
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn get_manga_list(
            listing_ptr: i32,
            listing_len: i32,
            page: i32,
        ) -> i64 {
            let slice = unsafe { core::slice::from_raw_parts(listing_ptr as *const u8, listing_len as usize) };
            let listing: $crate::models::Listing = $crate::postcard::from_bytes(slice).unwrap();
            let res = <$type as $crate::provider::MangaProvider>::get_manga_list(listing, page);
            let bytes = $crate::postcard::to_allocvec(&res).unwrap().into_boxed_slice();
            let ptr = bytes.as_ptr() as u64;
            let len = bytes.len() as u64;
            let _ = Box::into_raw(bytes); // Allow Swift to safely read without deallocation
            ((ptr << 32) | len) as i64
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn get_search_manga_list(
            query_ptr: i32, query_len: i32,
            page: i32,
            filters_ptr: i32, filters_len: i32,
        ) -> i64 {
            let q_slice = unsafe { core::slice::from_raw_parts(query_ptr as *const u8, query_len as usize) };
            let query = String::from_utf8_lossy(q_slice).into_owned();

            let f_slice = unsafe { core::slice::from_raw_parts(filters_ptr as *const u8, filters_len as usize) };
            let filters: Vec<$crate::models::FilterItem> = if filters_len == 0 {
                Vec::new()
            } else {
                $crate::postcard::from_bytes(f_slice).unwrap()
            };

            let res = <$type as $crate::provider::MangaProvider>::get_search_manga_list(query, page, filters);
            let bytes = $crate::postcard::to_allocvec(&res).unwrap().into_boxed_slice();
            let ptr = bytes.as_ptr() as u64;
            let len = bytes.len() as u64;
            let _ = Box::into_raw(bytes);
            ((ptr << 32) | len) as i64
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn get_manga_update(
            manga_ptr: i32, manga_len: i32,
            needs_details: i32, needs_chapters: i32,
        ) -> i64 {
            let slice = unsafe { core::slice::from_raw_parts(manga_ptr as *const u8, manga_len as usize) };
            
            let manga: $crate::models::Manga = match $crate::postcard::from_bytes(slice) {
                Ok(m) => m,
                Err(e) => {
                    let msg = format!("Postcard decoding error in get_manga_update: {}", e);
                    unsafe { $crate::host::print(msg.as_ptr() as i32, msg.len() as i32); }
                    panic!("Postcard decoding error in get_manga_update");
                }
            };
            let res = <$type as $crate::provider::MangaProvider>::get_manga_update(
                manga,
                needs_details != 0,
                needs_chapters != 0
            );
            let bytes = $crate::postcard::to_allocvec(&res).unwrap().into_boxed_slice();
            let ptr = bytes.as_ptr() as u64;
            let len = bytes.len() as u64;
            let _ = Box::into_raw(bytes);
            ((ptr << 32) | len) as i64
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn get_page_list(
            manga_ptr: i32, manga_len: i32,
            chapter_ptr: i32, chapter_len: i32,
        ) -> i64 {
            let m_slice = unsafe { core::slice::from_raw_parts(manga_ptr as *const u8, manga_len as usize) };
            let manga: $crate::models::Manga = $crate::postcard::from_bytes(m_slice).unwrap();

            let c_slice = unsafe { core::slice::from_raw_parts(chapter_ptr as *const u8, chapter_len as usize) };
            let chapter: $crate::models::Chapter = $crate::postcard::from_bytes(c_slice).unwrap();

            let res = <$type as $crate::provider::MangaProvider>::get_page_list(manga, chapter);
            let bytes = $crate::postcard::to_allocvec(&res).unwrap().into_boxed_slice();
            let ptr = bytes.as_ptr() as u64;
            let len = bytes.len() as u64;
            let _ = Box::into_raw(bytes);
            ((ptr << 32) | len) as i64
        }
    };
}
