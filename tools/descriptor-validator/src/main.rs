use std::env;
use std::process;

use descriptor_validator::validate_fixture_dir;

fn main() {
    let mut args = env::args().skip(1);
    let first = args.next();

    if matches!(first.as_deref(), Some("-h" | "--help")) {
        print_help();
        return;
    }

    let fixture_dir = match first {
        Some(value) => value,
        None => {
            eprintln!("error: missing fixture directory argument\n");
            print_help();
            process::exit(2);
        }
    };

    if args.next().is_some() {
        eprintln!("error: expected exactly one fixture directory argument\n");
        print_help();
        process::exit(2);
    }

    let report = validate_fixture_dir(fixture_dir);
    println!("{}", report.to_json_pretty());

    if !report.errors.is_empty() {
        process::exit(1);
    }
}

fn print_help() {
    println!(
        "\
descriptor-validator

Usage:
  descriptor-validator <fixture-dir>
  descriptor-validator --help

First-wave offline scaffold for Hope KB descriptor validation.
This tool only validates the fixture directory boundary today. It does not read
runtime artifacts, snapshot SQLite files, raw KB rows, or full source_register
content."
    );
}
