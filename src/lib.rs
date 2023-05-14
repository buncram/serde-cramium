use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub enum GetRandType {
    Rand,
    Prime,
    Safe
}
#[derive(Serialize, Deserialize)]
pub enum HashPrefix {
    Off,
    On,
}
#[derive(Serialize, Deserialize)]
pub enum HashPreProcess {
    Off,
    On,
}
#[derive(Serialize, Deserialize)]
pub enum HashType {
    RipeMd = 0,
    Sha256 = 1,
    Sha512 = 2,
    Keccak = 3,
    Blake2b = 4,
    Blake2s = 5,
    Blake3 = 6,
}
#[derive(Serialize, Deserialize)]
pub enum ModArithSize {
    Bits256,
    Bits384,
    Bits512,
    Bits2048,
    Bits3072,
    Bits4096
}
#[derive(Serialize, Deserialize)]
pub enum ModArithType {
    Add,
    Multiply,
    Inverse,
    Exp,
    Sub,
}
#[derive(Serialize, Deserialize)]
pub struct HashWcMessage {
    pub msg: Vec<u8>
}

#[derive(Serialize, Deserialize)]
pub enum DaricParam {
    GetRand(GetRandType),
    Hash(HashPrefix, HashPreProcess, HashType, HashWcMessage),
    ModArith(ModArithSize, ModArithType),
}
#[derive(Serialize, Deserialize)]
pub struct DaricCmd {
    pub dest: Vec<u8>,
    pub src1: Vec<u8>,
    pub src2: Option<Vec<u8>>,
    pub param: DaricParam,
}
