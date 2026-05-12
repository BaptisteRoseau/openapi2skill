use oapi::{self, OApi};
use sppparse::SparseRoot;

enum Error {}

pub fn load_oapi(link: &str) -> Result<OApi, Error> {
    let content = if link.starts_with("http") {
        load_http(link)?
    } else {
        load_file(link)?
    };
    Ok(parse_oapi(content.as_str())?)
}

fn load_http(link: &str) -> Result<String, Error> {
    todo!()
}

fn load_file(path: &str) -> Result<String, Error> {
    todo!()
}

fn parse_oapi(content: &str) -> Result<OApi, Error> {
    OApi::new(SparseRoot::new_from_value(rval, path, others))
    todo!()
}
