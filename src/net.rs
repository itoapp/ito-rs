use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::host;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetRequest {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetResponse {
    pub status: i32,
    pub headers: HashMap<String, String>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

pub struct Request {
    req: NetRequest,
}

impl Request {
    pub fn new<S: Into<String>, M: Into<String>>(url: S, method: M) -> Self 
    {
        Self {
            req: NetRequest {
                url: url.into(),
                method: method.into(),
                headers: HashMap::new(),
                body: None,
            },
        }
    }

    pub fn get<S: Into<String>>(url: S) -> Self {
        Self::new(url, "GET")
    }

    pub fn post<S: Into<String>>(url: S) -> Self {
        Self::new(url, "POST")
    }

    pub fn header(&mut self, key: &str, value: &str) -> &mut Self {
        self.req.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn body(&mut self, body: &[u8]) -> &mut Self {
        self.req.body = Some(body.to_vec());
        self
    }

    pub fn send(&self) -> NetResponse {
        let req_bytes = postcard::to_allocvec(&self.req).unwrap();
        let len = unsafe { host::fetch(req_bytes.as_ptr() as i32, req_bytes.len() as i32) };
        
        let mut response_buf = Vec::<u8>::with_capacity(len as usize);
        let ptr = response_buf.as_mut_ptr();
        std::mem::forget(response_buf);

        unsafe { host::fetch_read(ptr as i32) };

        let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };
        
        let res: NetResponse = match postcard::from_bytes(slice) {
            Ok(r) => r,
            Err(e) => {
                let mut hex_string = String::new();
                for b in slice {
                    use core::fmt::Write;
                    write!(&mut hex_string, "{:02x}", b).unwrap();
                }
                let msg = format!("[DEBUG] NetResponse Decoding Error (len {}): {} | Hex: {}", len, e, hex_string);
                unsafe { host::print(msg.as_ptr() as i32, msg.len() as i32); }
                panic!("Failed to decode NetResponse");
            }
        };
        
        unsafe { crate::ffi_alloc::dealloc(ptr as *mut u8, len as usize) };
        res
    }
}
