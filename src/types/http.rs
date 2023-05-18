use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize)]
pub struct RpcCall<'a> {
    pub jsonrpc: &'a str,
    pub id: &'a str,
    pub method: &'a str,
    pub params: &'a [RpcParameter<'a>]
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum RpcParameter<'a> {
    Objects(&'a [&'a str]),
    Options(GetObjectsOptions),
    String(String),
    SliceString(&'a [String])
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectsOptions { 
    pub show_owner: bool 
}

#[derive(Debug, Deserialize)]
pub struct RpcResponse<'a, T> {
    pub jsonrpc: &'a str,
    pub id: &'a str,
    pub result: T
}

#[derive(Debug, Deserialize)]
pub struct Object<'a> { 
    #[serde(borrow)]
    pub data: ObjectData<'a> 
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectData<'a> {
    pub object_id: &'a str,
    pub version: &'a str,
    pub digest: &'a str,
    pub owner: Owner<'a>
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum Owner<'a> {
    Shared(SharedOwner),
    AddressOwner(&'a str)
}

impl<'a> Owner<'a> {
    pub fn shared(self) -> Option<SharedOwner> {
        match self {
            Owner::Shared(x) => Some(x),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct SharedOwner { 
    pub initial_shared_version: u64
}
