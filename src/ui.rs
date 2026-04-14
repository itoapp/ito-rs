use crate::host;
use crate::models::HomeComponent;

pub fn push_home_component(comp: &HomeComponent) -> crate::Result<()> {
    let bytes = postcard::to_allocvec(comp).map_err(crate::Error::Postcard)?;
    unsafe {
        host::push_home_component(bytes.as_ptr() as i32, bytes.len() as i32);
    }
    Ok(())
}
