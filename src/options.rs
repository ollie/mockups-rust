//! Handles CLI options parsing and errors.

use getopts;
use std::os;

/// Parse CLI options and return them.
pub fn get_options() -> getopts::Matches {
    let args          = os::args();
    let first_arg     = args[0].clone();
    let option_args   = args.tail();
    let filename_path = Path::new(first_arg);
    let program       = filename_path.filename_str().unwrap();

    process_options(program, option_args)
}

/// Define individual options, fail if incorrect.
pub fn process_options(program: &str, option_args: &[String]) -> getopts::Matches {
    let opts = [
        getopts::reqopt("d",  "dir",  "set path to images directory", "PATH"),
        getopts::optflag("h", "help", "print this help")
    ];

    let matches = match getopts::getopts(option_args, opts) {
        Ok(m)    => m,
        Err(why) => {
            println!("{}", why);
            println!("{}", getopts::usage(program, opts));
            fail!();
        }
    };

    matches
}
