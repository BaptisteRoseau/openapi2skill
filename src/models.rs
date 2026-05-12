use std::collections::{HashMap, HashSet};

enum Method {
    POST,
    PUT,
    HEAD,
    GET,
    CONNECT,
    OPTIONS,
    TRACE,
    DELETE,
    PATCH,
}

struct Endpoint {
    method: Method,
    path: String,
    description: String,
    inputs: Vec<EndpointInput>,
    outputs: Vec<EndpointOutputs>,
}

struct Schema {
    fields: HashMap<String, String>
}

struct EndpointInput {
    status_codes: HashSet<u8>,
    schema: HashMap<String, String>,
}

struct EndpointOutputs {
    status_codes: HashSet<u8>,
    schema: HashMap<String, String>,
}

struct Category {
    name: String,
    description: String,
    enpoints: Vec<Endpoint>,
}

struct Skill {
    name: String,
    description: String,
    authentication: Option<String>,
    categories: Vec<Category>,
}
