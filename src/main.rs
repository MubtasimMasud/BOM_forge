// Author:
//     ___         __              __       __  ___                     __
//    /   | ____ _/ /______ ______/ /_     /  |/  /___ ________  ______/ /
//   / /| |/ __ `/ //_/ __ `/ ___/ __ \   / /|_/ / __ `/ ___/ / / / __  /
//  / ___ / /_/ / ,< / /_/ (__  ) / / /  / /  / / /_/ (__  ) /_/ / /_/ /
// /_/  |_\__,_/_/|_|\__,_/____/_/ /_/  /_/  /_/\__,_/____/\__,_/\__,_/
use anyhow::Result;
use std::io;

use parse::{find_ambigious_names, parse_cpl, read_preprocessed_bom};

pub mod components;
pub mod parse;

fn main() -> Result<()> {
    let data =
        read_preprocessed_bom("/home/aakash_masud/Projects/bom_forge/test_BOMs/BOM_CAN_Ver.csv")?;
    let cpl_path =
        "/home/aakash_masud/Downloads/CANbus_verification_2025/Pick Place for PCB1.csv".to_string();

    let correct_designators = parse_cpl(&cpl_path)?;

    for pair in correct_designators {
        println!("{:?}", pair);
    }

    Ok(())
}
