use serde::de::DeserializeOwned;

#[link(wasm_import_module = "ito:core/env")]
unsafe extern "C" {
    pub fn get_languages() -> i64;
}

pub fn get_preferred_languages() -> crate::Result<Vec<String>> {
    let packed = unsafe { get_languages() };
    if packed == 0 {
        return Ok(Vec::new());
    }
    unsafe { from_packed_ptr(packed) }
}

unsafe fn from_packed_ptr<T: DeserializeOwned>(packed: i64) -> crate::Result<T> {
    let ptr = (packed >> 32) as i32;
    let len = (packed & 0xFFFFFFFF) as i32;
    
    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let res = postcard::from_bytes(slice).map_err(crate::Error::Postcard);
    
    unsafe { crate::ffi_alloc::dealloc(ptr as *mut u8, len as usize) };
    res
}
