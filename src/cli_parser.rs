use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {

    // The path to an file that consists of raw bits with or without gnuradio metadata.
    input_file: Option<PathBuf>,

}
