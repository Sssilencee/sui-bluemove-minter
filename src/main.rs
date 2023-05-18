mod types;
mod utils;

use anyhow::Error;
use base64::{
    engine::general_purpose, 
    Engine as _
};
use bcs::to_bytes;
use ed25519_dalek::Keypair;
use std::env;
use utils::{
    rpc_client::RpcClient,
    sui_keypair::SuiKeypair
};
use types::{
    sui::{
        TransactionData,
        TransactionDataV1,
        TransactionKind,
        GasData,
        ProgrammableTransaction,
        CallArg,
        Command,
        ProgrammableMoveCall,
        ObjectArg,
        Argument,
        SUI_CLOCK_OBJECT_ID,
        SUI_CLOCK_OBJECT_SHARED_VERSION,
        SUI_MAINNET_RPC,
    },
    http::{
        RpcResponse,
        Object,
    },
};


fn from_hex(s: &String) -> [u8; 32] { 
    hex::decode(s)
        .unwrap()[..]
        .try_into()
        .unwrap() 
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    let keypair = Keypair::from_hex(env::var("secret_key")?);
    let address = keypair.get_address();

    let count = env::var("count")?;
    let sale_type = env::var("sale_type")?;
    let cap_address = env::var("cap_address")?;
    let gas_address = env::var("gas_address")?;
    
    let client = RpcClient::new(SUI_MAINNET_RPC.to_string());

    let prefix_cap = "0x".to_owned() + &cap_address;
    let prefix_gas = "0x".to_owned() + &gas_address;
    let objects_response = client.multi_get_objects(&[ 
        prefix_cap.as_str(), prefix_gas.as_str() 
    ]).await?;
    let objects: RpcResponse<Vec<Object>> = serde_json::from_str(objects_response.as_str())?;
    let gas_data = objects.result[1].data;

    let gas_response = client.get_reference_gas_price().await?;
    let gas_price: RpcResponse<&str> = serde_json::from_str(gas_response.as_str())?;

    let mut inputs = vec![
        CallArg::Pure(env::var("price")?
            .parse::<u64>()?
            .to_ne_bytes()
            .into()
        ),
        CallArg::Pure(sale_type
            .parse::<u64>()?
            .to_ne_bytes()
            .into()
        ),
        CallArg::Object(ObjectArg::SharedObject {
            id: from_hex(&cap_address),
            initial_shared_version: objects.result[0].data.owner
                .shared()
                .unwrap()
                .initial_shared_version,
            mutable: true
        }),
        CallArg::Pure(count
            .parse::<u64>()?
            .to_ne_bytes()
            .into()
        ),
        CallArg::Object(ObjectArg::SharedObject {
            id: from_hex(&SUI_CLOCK_OBJECT_ID.to_string()),
            initial_shared_version: SUI_CLOCK_OBJECT_SHARED_VERSION,
            mutable: false
        })
    ];

    let tx = TransactionData::V1(TransactionDataV1 {
        kind: TransactionKind::ProgrammableTransaction(ProgrammableTransaction {
            inputs: if count == sale_type {
                inputs.remove(3); inputs
            } else { inputs },
            commands: vec![
                Command::SplitCoins(
                    Argument::GasCoin, 
                    vec![Argument::Input(0)]
                ),
                Command::MoveCall(
                    Box::new(ProgrammableMoveCall {
                        package: from_hex(&env::var("mint_address")?),
                        module: String::from("bluemove_launchpad"),
                        function: String::from("mint_with_quantity"),
                        type_arguments: vec![],
                        arguments: vec![
                            Argument::NestedResult(0, 0), 
                            Argument::Input(1),
                            Argument::Input(2),
                            Argument::Input(
                                if count == sale_type {1} else {3}
                            ),
                            Argument::Input(4)
                        ]
                    })
                )
            ]
        }),
        sender: address,
        gas_data: GasData {
            payment: vec![
                (
                    from_hex(&gas_address),
                    gas_data.version
                        .parse()
                        .unwrap(),
                    bs58::decode(gas_data.digest).into_vec()?
                )
            ],
            owner: address,
            price: gas_price.result
                .parse()
                .unwrap(),
            budget: env::var("gas_budget")?
                .parse()
                .unwrap(),
        },
        expiration: 0
    });

    let signature = keypair.sign_bs64(&to_bytes(&tx).unwrap()[..]);

    let response = client.execute_transaction_block(
        general_purpose::STANDARD.encode(to_bytes(&tx).unwrap()),
        &[signature]
    ).await?;

    println!("{}", response);

    Ok(())

}