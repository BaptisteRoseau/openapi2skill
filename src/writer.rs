use crate::error::O2SError;
use oapi::{OApi, OApiDocument, OApiSchemaObject};
use std::{io, path::Path};

pub fn openapi2skill(openapi: &OApi, output_directory: Option<&Path>) -> Result<(), O2SError> {
    // let skill_name: OApiDocument = openapi.check().unwrap().;
    todo!()
}

pub fn write_schema(writer: &dyn io::Write, schema: OApiSchemaObject) {
    todo!()
}
