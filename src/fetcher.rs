use crate::error::O2SError;
use oapi::{self, OApi};

pub fn load_oapi(link: &str) -> Result<OApi, O2SError> {
    let content = if link.starts_with("http") {
        load_http(link)?
    } else {
        load_file(link)?
    };
    parse_oapi(content.as_str())
}

fn load_http(link: &str) -> Result<String, O2SError> {
    todo!()
}

fn load_file(path: &str) -> Result<String, O2SError> {
    todo!()
}

fn parse_oapi(content: &str) -> Result<OApi, O2SError> {
    // OApi::new(SparseRoot::new_from_value(rval, path, others))
    todo!()
}
