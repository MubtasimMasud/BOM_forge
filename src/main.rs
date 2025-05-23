// Author:
//     ___         __              __       __  ___                     __
//    /   | ____ _/ /______ ______/ /_     /  |/  /___ ________  ______/ /
//   / /| |/ __ `/ //_/ __ `/ ___/ __ \   / /|_/ / __ `/ ___/ / / / __  /
//  / ___ / /_/ / ,< / /_/ (__  ) / / /  / /  / / /_/ (__  ) /_/ / /_/ /
// /_/  |_\__,_/_/|_|\__,_/____/_/ /_/  /_/  /_/\__,_/____/\__,_/\__,_/
use anyhow::Result;

use parse::{CplData, PreprocessedBOM};

pub mod components;
pub mod parse;

fn main() -> Result<()> {
    let bom_path = "/home/aakash_masud/Projects/bom_forge/test_BOMs/BOM_CAN_Ver.csv".to_string();
    let cpl_path =
        "/home/aakash_masud/Downloads/CANbus_verification_2025/Pick Place for PCB1.csv".to_string();
    let mut bom = PreprocessedBOM::from_csv(&bom_path)?;
    let mut cpl = CplData::from_csv(&cpl_path)?;

    bom.fix_ambigious_rows(&cpl)?;

    for entry in bom.bom {
        println!("{:?}", entry);
    }

    Ok(())
}
