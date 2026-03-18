#[link(wasm_import_module = "ito:core/net")]
unsafe extern "C" {
    pub fn fetch(req_ptr: i32, req_len: i32) -> i32;
    pub fn fetch_read(ptr: i32);
}

#[link(wasm_import_module = "ito:core/html")]
unsafe extern "C" {
    pub fn parse(html_ptr: i32, html_len: i32) -> i32;
    pub fn select(element_id: i32, selector_ptr: i32, selector_len: i32) -> i64;
    pub fn text(element_id: i32) -> i64;
    pub fn attr(element_id: i32, name_ptr: i32, name_len: i32) -> i64;
    pub fn free(element_id: i32);
}

#[link(wasm_import_module = "ito:core/js")]
unsafe extern "C" {
    pub fn evaluate(script_ptr: i32, script_len: i32) -> i64;
}

#[link(wasm_import_module = "ito:core/std")]
unsafe extern "C" {
    #[link_name = "print"]
    pub fn sys_print(msg_ptr: i32, msg_len: i32);
}

#[link(wasm_import_module = "ito:core/defaults")]
unsafe extern "C" {
    pub fn set(key_ptr: i32, key_len: i32, val_ptr: i32, val_len: i32);
    pub fn get(key_ptr: i32, key_len: i32) -> i64;
    pub fn remove(key_ptr: i32, key_len: i32);
}

pub fn print(msg: &str) {
    let msg_bytes = msg.as_bytes();
    unsafe {
        sys_print(msg_bytes.as_ptr() as i32, msg_bytes.len() as i32);
    }
}
