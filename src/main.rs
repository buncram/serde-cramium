use serde_cramium::*;

use serde::{Deserialize, Serialize};
use std::error::Error;

fn print_demos(cmd: &DaricCmd, name: &str) -> Result<(), Box<dyn Error>> {
    let j = serde_json::to_string(cmd)?;
    println!("{}", j);
    let j = serde_json::to_string_pretty(cmd)?;
    println!("{}", j);
    // let y = serde_yaml::to_string(cmd)?;
    // println!("{}", y);
    let c_file = std::fs::File::create(name)?;
    ciborium::into_writer( &cmd, &c_file)?;
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let ex_hash_cmd = DaricCmd {
        dest: [1, 2, 3, 4, 5, 6, 7, 8].to_vec(),
        src1: [1, 2, 3, 4].to_vec(),
        src2: None,
        param: DaricParam::Hash(
            HashPrefix::Off,
            HashPreProcess::On,
            HashType::Keccak,
            HashWcMessage {msg: [].to_vec()},
        )
    };

    print_demos(&ex_hash_cmd, "hash_cbor.bin")?;

    let ex_getrand_cmd = DaricCmd {
        dest: "rng_outfile.bin".as_bytes().to_vec(),
        src1: [0, 0, 0, 1].to_vec(),
        src2: None,
        param: DaricParam::GetRand(
            GetRandType::Prime
        ),
    };

    print_demos(&ex_getrand_cmd, "rng_cbor.bin")?;

    let ex_modarith_cmd = DaricCmd {
        dest: [1, 1, 1, 1, 2, 2, 2, 2].to_vec(),
        src1: [3, 3, 3, 3].to_vec(),
        src2: Some([4, 4, 4, 4, 4, 4, 4, 4].to_vec()),
        param: DaricParam::ModArith(ModArithSize::Bits2048, ModArithType::Sub)
    };

    print_demos(&ex_modarith_cmd, "modarith_cbor.bin")?;

    Ok(())
}