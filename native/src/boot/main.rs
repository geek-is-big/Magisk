use magiskboot::base::CmdArgs;
use std::env;
use std::process;

fn main() {
    let args = env::args()
        .map(|arg| Box::leak(arg.into_boxed_str()) as &'static str)
        .collect();
    process::exit(magiskboot::run(CmdArgs(args)));
}
