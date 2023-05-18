use anyhow::Error;
use reqwest::Client;

use crate::types::{
    http::{
        RpcCall,
        GetObjectsOptions,
        RpcParameter,
    },
};


pub struct RpcClient {
    client: Client,
    rpc: String,
}

impl RpcClient {
    pub fn new(rpc: String) -> Self {
        Self {
            client: Client::new(),
            rpc
        }
    }

    pub async fn multi_get_objects<'a>(&self, objects: &[&str]) -> Result<String, Error> {
        let body = RpcCall {
            jsonrpc: "2.0",
            id: "1",
            method: "sui_multiGetObjects",
            params: &[
                RpcParameter::Objects(objects),
                RpcParameter::Options(GetObjectsOptions {
                    show_owner: true
                })
            ]
        };
        let response = self.make_request(serde_json::to_string(&body)?).await?;
        Ok(response)
    } 

    pub async fn get_reference_gas_price(&self) -> Result<String, Error> {
        let body = RpcCall {
            jsonrpc: "2.0",
            id: "1",
            method: "suix_getReferenceGasPrice",
            params: &[]
        };
        let response = self.make_request(serde_json::to_string(&body)?).await?;
        Ok(response)
    }

    pub async fn execute_transaction_block(&self,
        tx_bytes: String, 
        signatures: &[String]
    ) -> Result<String, Error> {
        let body = RpcCall {
            jsonrpc: "2.0",
            id: "1",
            method: "sui_executeTransactionBlock",
            params: &[
                RpcParameter::String(tx_bytes),
                RpcParameter::SliceString(signatures)
            ]
        };
        let response = self.make_request(serde_json::to_string(&body)?).await?;
        Ok(response)
    }

    pub async fn make_request(&self, body: String) -> Result<String, Error> {
        Ok(self.client.post(self.rpc.as_str())
            .body(body)
            .header("content-type", "application/json")
            .send().await?
            .text().await?)
    }
}