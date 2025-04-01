use clap::Parser;
use not_tailwind::{
    args::NtwArgs, read_vue::build_ts_map, starter::start_all2,
};

pub fn main() {
    let args = NtwArgs::parse();
    if args.build_ts_map {
        build_ts_map();
    }
    if args.run.is_empty() {
        println!("Please specify file type");
    } else {
        start_all2(args);
    }
}
