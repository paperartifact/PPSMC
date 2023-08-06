#![feature(get_mut_unchecked)]

mod automata;
mod command;
mod ltl;
mod property_driven;
mod traditional;
mod util;

use clap::Parser;
use command::Algorithm;
use smv::Smv;

type BddManager = sylvan::Sylvan;
type Bdd = sylvan::Bdd;

fn main() {
    let input_file =
    // "abp8-p0.smv";
    "prod-cons-p0.smv";
    // "production-cell.smv";
    // "bc57-sensors-p1.smv";

    // "viscoherence-p1.smv";
    // "cunim1ro.smv";
    // "cuhanoi7ro.smv";
    // "cuhanoi10ro.smv";
    // "cuabq2mfro.smv";
    // "phils-p1.smv";
    // "msi_wtrans.smv";
    // "syncarb.smv";
    // "elevator.smv";
    // "ftp3-flat.smv"; //???

    let mut input_file = format!("./benchmark/{}", input_file);
    let args = command::Args::parse();
    if !args.file.is_empty() {
        input_file = args.file.to_string();
    }
    let smv = Smv::from_file(input_file).unwrap();
    let manager = BddManager::init(args.parallel);
    let algorithm = match args.algorithm {
        Algorithm::PropertyDriven => property_driven::check,
        Algorithm::Traditional => traditional::check,
    };
    let (res, time) = algorithm(manager, smv, args);
    println!("res: {}, time: {:?}", res, time);
}
