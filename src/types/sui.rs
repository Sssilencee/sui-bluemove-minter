use serde::Serialize;


pub const SUI_CLOCK_OBJECT_ID: &str = "0000000000000000000000000000000000000000000000000000000000000006";
pub const SUI_CLOCK_OBJECT_SHARED_VERSION: u64 = 1;
pub const SUI_MAINNET_RPC: &str = "https://fullnode.mainnet.sui.io/";

#[derive(Debug, Serialize)]
pub enum TransactionData {
    V1(TransactionDataV1),
}

#[derive(Debug, Serialize)]
pub struct TransactionDataV1 {
    pub kind: TransactionKind,
    pub sender: [u8; 32],
    pub gas_data: GasData,
    pub expiration: u8,
}

#[derive(Debug, Serialize)]
pub enum TransactionKind {
    ProgrammableTransaction(ProgrammableTransaction),
}

#[derive(Debug, Serialize)]
pub struct GasData {
    pub payment: Vec<ObjectRef>,
    pub owner: [u8; 32],
    pub price: u64,
    pub budget: u64,
}

#[derive(Debug, Serialize)]
pub struct ProgrammableTransaction {
    pub inputs: Vec<CallArg>,
    pub commands: Vec<Command>,
}

#[derive(Debug, Serialize)]
pub enum CallArg {
    Pure(Vec<u8>),
    Object(ObjectArg),
}

#[derive(Debug, Serialize)]
pub enum Command {
    MoveCall(Box<ProgrammableMoveCall>),
    _TransferObjects(Vec<Argument>, Argument),
    SplitCoins(Argument, Vec<Argument>),
}

#[derive(Debug, Serialize)]
pub struct ProgrammableMoveCall {
    pub package: [u8; 32],
    pub module: String,
    pub function: String,
    pub type_arguments: Vec<u8>,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Serialize)]
pub enum ObjectArg {
    _ImmOrOwnedObject(ObjectRef),
    SharedObject {
        id: [u8; 32],
        initial_shared_version: u64,
        mutable: bool,
    },
}

#[derive(Debug, Serialize)]
pub enum Argument {
    GasCoin,
    Input(u16),
    _Result(u16),
    NestedResult(u16, u16),
}

pub type ObjectRef = ([u8; 32], u64, Vec<u8>);