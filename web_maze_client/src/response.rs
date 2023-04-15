use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Submit {
    #[serde(rename = "runId")]
    pub run_id: String,
}

#[derive(Deserialize, Debug)]
pub struct List {
    pub limit: usize,
    pub start: usize,

    #[serde(rename = "runIds")]
    pub run_ids: Vec<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Queries {
    #[serde(rename = "runId")]
    pub run_id: String,
    pub limit: usize,
    pub start: usize,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub queries: Vec<Query>,
}

#[derive(Deserialize, Debug)]
pub struct Query {
    pub connection_source: String,
    pub connection_port: usize,
    pub query_target: usize,
}
