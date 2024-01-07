use std::env::ArgsOs;

use dct;
fn main() {
    let args: ArgsOs = std::env::args_os();
    dct::dct_main(args);
}
