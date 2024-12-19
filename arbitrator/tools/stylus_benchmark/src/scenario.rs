// Copyright 2021-2024, Offchain Labs, Inc.
// For license information, see https://github.com/OffchainLabs/nitro/blob/master/LICENSE

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use strum_macros::{EnumString, Display};

#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumString, Display)]
pub enum Scenario {
    #[strum(serialize = "add_i32")]
    AddI32,
}

fn generate_add_i32_wat() -> Vec<u8> {
    let number_of_ops = 20_000;

    let mut wat = Vec::new();

    wat.write_all(b"(module\n").unwrap();
    wat.write_all(b"    (import \"debug\" \"toggle_benchmark\" (func $toggle_benchmark))\n").unwrap();
    wat.write_all(b"    (memory (export \"memory\") 0 0)\n").unwrap();
    wat.write_all(b"    (func (export \"user_entrypoint\") (param i32) (result i32)\n").unwrap();

    wat.write_all(b"        call $toggle_benchmark\n").unwrap();

    wat.write_all(b"        i32.const 1\n").unwrap();
    for _ in 0..number_of_ops {
        wat.write_all(b"        i32.const 1\n").unwrap();
        wat.write_all(b"        i32.add\n").unwrap();
    }

    wat.write_all(b"        call $toggle_benchmark\n").unwrap();

    wat.write_all(b"        drop\n").unwrap();
    wat.write_all(b"        i32.const 0)\n").unwrap();
    wat.write_all(b")").unwrap();

    wat
}

pub fn generate_wat(scenario: Scenario, output_wat_dir_path: Option<PathBuf>) -> Vec<u8> {
    let wat = match scenario {
        Scenario::AddI32 => generate_add_i32_wat(),
    };

    if let Some(output_wat_dir_path) = output_wat_dir_path {
        let mut output_wat_path = output_wat_dir_path;
        output_wat_path.push(format!("{}.wat", scenario));
        let mut file = File::create(output_wat_path).unwrap();
        file.write_all(&wat).unwrap();
    }

    wat
}
