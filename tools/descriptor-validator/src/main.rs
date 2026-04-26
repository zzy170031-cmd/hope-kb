use std::env;
use std::process;

use descriptor_validator::model::ValidationStatus;
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
    let failed = report.status == ValidationStatus::Failed;
    println!("{}", report.to_json_pretty());

    if failed {
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

Offline validator for Hope KB descriptor fixtures.
This tool validates JSON descriptor fixtures with artifact-class allow-listing,
recursive denied-field scanning, leakage guards, purge checks, rollback checks,
refresh telemetry checks, and activation/pointer static checks. It does not
read runtime artifacts, snapshot SQLite files, raw KB rows, or full
source_register content."
    );
}
