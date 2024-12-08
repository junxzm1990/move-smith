// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

#![no_main]

use arbitrary::Unstructured;
use libfuzzer_sys::fuzz_target;
use move_smith::{
    cli::{
        compile::simple_compile,
	raw2move::raw2move,
    },
    config::Config,
    execution::{
        transactional::{
            CommonRunConfig, TransactionalExecutor, TransactionalInputBuilder, TransactionalResult,
        },
        ExecutionManager,
    },
    CodeGenerator, MoveSmith,
};
use once_cell::sync::Lazy;
use std::{env, fs::OpenOptions, fs::File, io::Write, path::PathBuf, sync::Mutex, time::Instant};

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_path =
        env::var("MOVE_SMITH_CONFIG").unwrap_or_else(|_| "MoveSmith.toml".to_string());
    let config_path = PathBuf::from(config_path);
    Config::from_toml_file_or_default(&config_path)
});

fuzz_target!(|data: &[u8]| {

/*
    let u = &mut Unstructured::new(data);
    let mut smith = MoveSmith::new(&CONFIG.generation);
    
    match smith.generate(u) {
        Ok(()) => (),
        Err(_) => return,
    };
    let code = smith.get_compile_unit().emit_code();

    simple_compile(&code);
*/
    
    let (_, _, code) = raw2move(&CONFIG.generation, data);

    simple_compile(&code);

});

