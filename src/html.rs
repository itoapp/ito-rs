use crate::host;

pub struct Node {
    id: i32,
}

impl Node {
    pub fn new(html: &[u8]) -> Self {
        let id = unsafe { host::parse(html.as_ptr() as i32, html.len() as i32) };
        Self { id }
    }

    pub fn select(&self, selector: &str) -> Vec<Node> {
        let selector_bytes = selector.as_bytes();
        let packed = unsafe {
            host::select(
                self.id,
                selector_bytes.as_ptr() as i32,
                selector_bytes.len() as i32,
            )
        };
        
        let ptr = (packed >> 32) as i32;
        let len = (packed & 0xFFFFFFFF) as i32;
        
        let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };
        let ids: Vec<i32> = postcard::from_bytes(slice).unwrap();
        
        unsafe { crate::ffi_alloc::dealloc(ptr as *mut u8, len as usize) };
        
        ids.into_iter().map(|id| Node { id }).collect()
    }

    pub fn text(&self) -> String {
        let packed = unsafe { host::text(self.id) };
        let ptr = (packed >> 32) as i32;
        let len = (packed & 0xFFFFFFFF) as i32;
        
        let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };
        let text: String = postcard::from_bytes(slice).unwrap();
        
        unsafe { crate::ffi_alloc::dealloc(ptr as *mut u8, len as usize) };
        text
    }

    pub fn attr(&self, name: &str) -> Option<String> {
        let name_bytes = name.as_bytes();
        let packed = unsafe { host::attr(self.id, name_bytes.as_ptr() as i32, name_bytes.len() as i32) };
        let ptr = (packed >> 32) as i32;
        let len = (packed & 0xFFFFFFFF) as i32;
        
        let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };
        let attr: Option<String> = postcard::from_bytes(slice).unwrap();
        
        unsafe { crate::ffi_alloc::dealloc(ptr as *mut u8, len as usize) };
        attr
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe { host::free(self.id) };
    }
}
