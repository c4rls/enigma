use libenigma::vm;

use std::env;
use std::process;

fn run() -> i32 {
    // use simplelog::*;
    // use std::fs::File;
    // CombinedLogger::init(vec![WriteLogger::new(
    //     LevelFilter::Info,
    //     Config::default(),
    //     File::create("enigma.log").unwrap(),
    // )])
    // .unwrap();

    let _args: Vec<String> = env::args().collect();

    // std::panic::set_hook(Box::new(|panic_info| {
    //     let backtrace = backtrace::Backtrace::new();
    //     println!("{:?}", panic_info);
    //     println!("{:?}", backtrace);
    // }));

    let vm = vm::Machine::new();

    // erlexec defaults:
    let args: Vec<String> = vec![
        // "/Users/speed/src/rust/enigma/target/debug/enigma",
        "--",
        "-root",
        //"/usr/local/Cellar/erlang/21.3.2/lib/erlang",
        // "otp",
        "/Users/speed/src/rust/enigma/otp",
        "-progname",
        "enigma",
        "--",
        "-home",
        dirs::home_dir()
            .expect("No home directory")
            .to_str()
            .unwrap(),
        //"-init_debug",
        "--",
        "-kernel",
        "start_distribution",
        "false",
        // "-kernel",
        // "logger_level",
        // "debug",
        // "-kernel",
        // "logger_log_progress",
        // "true",
        "-pa",
        // "/Users/speed/src/elixir/lib/*/ebin",
        "/Users/speed/src/elixir/lib/elixir/ebin",
        "/Users/speed/src/elixir/lib/ex_unit/ebin",
        "/Users/speed/src/elixir/lib/iex/ebin",
        "/Users/speed/src/elixir/lib/logger/ebin",
        "/Users/speed/src/elixir/lib/mix/ebin",
        "-elixir",
        "ansi_enabled",
        "true",
        "-noshell",
        "-user",
        "Elixir.IEx.CLI",
        "-extra",
        "--no-halt",
        "--erl",
        "-noshell",
        // "-user",
        // "Elixir.IEx.CLI",
        "+iex",
        // "--",
        // "-kernel shell_history enabled",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    vm.preload_modules();

    vm.start(args);

    0
}

fn main() {
    process::exit(run());
}
