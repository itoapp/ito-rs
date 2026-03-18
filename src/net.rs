use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::host;
use crate::{Result, Error};

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

    pub fn send(&self) -> Result<NetResponse> {
        let req_bytes = postcard::to_allocvec(&self.req)?;
        let len = unsafe { host::fetch(req_bytes.as_ptr() as i32, req_bytes.len() as i32) };
        
        // Critical Fix: Use Vec built-in allocation handling correctly.
        // host::fetch returns the length of the response body.
        // We create a buffer with that capacity, let the host fill it, 
        // and then set the initialized length so Vec owns the memory properly.
        let mut response_buf = Vec::<u8>::with_capacity(len as usize);
        let ptr = response_buf.as_mut_ptr();
        
        unsafe { 
            host::fetch_read(ptr as i32);
            response_buf.set_len(len as usize);
        }

        let res: NetResponse = postcard::from_bytes(&response_buf)
            .map_err(|e| {
                 let msg = format!("NetResponse Decoding Error (len {}): {}", len, e);
                 host::print(&msg);
                 Error::Postcard(e)
            })?;
        
        Ok(res)
    }
}
