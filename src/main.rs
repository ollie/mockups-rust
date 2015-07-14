//! Walk down a project directory and generate a static HTML site containing
//! resized images.
//!
//! ## File structure before site is generated
//!
//! ```
//! Project Name
//!   |- mockups
//!   |    +- icon.png
//!   |- iphone-portrait
//!   |    |- XY-[section-a]-0.png
//!   |    +- XY-[section-a]-1.png
//!   |- iphone-landscape
//!   |    |- XY-[section-a]-0.png
//!   |    +- XY-[section-b]-0.png
//!   |- ipad-portrait
//!   |    +- XY-[section-b]-0.png
//!   +- ipad-landscape
//!        |- XY-[section-a]-0.png
//!        +- XY-[section-b]-0.png
//! ```
//!
//! ## File structure after site is generated
//!
//! ```
//! Project Name
//!   |- mockups
//!   |    +- icon.png
//!   |- iphone-portrait
//!   |    |- XY-[section-a]-0.png
//!   |    +- XY-[section-a]-1.png
//!   |- iphone-landscape
//!   |    |- XY-[section-a]-0.png
//!   |    +- XY-[section-b]-0.png
//!   |- ipad-portrait
//!   |    +- XY-[section-b]-0.png
//!   |- ipad-landscape
//!   |    |- XY-[section-a]-0.png
//!   |    +- XY-[section-b]-0.png
//!   +- site
//!        |- thumbs
//!        |    |- iphone-portrait
//!        |    |    |- XY-[section-a]-0.png
//!        |    |    +- XY-[section-a]-1.png
//!        |    |- iphone-landscape
//!        |    |    |- XY-[section-a]-0.png
//!        |    |    +- XY-[section-b]-0.png
//!        |    |- ipad-portrait
//!        |    |    +- XY-[section-b]-0.png
//!        |    +- ipad-landscape
//!        |         |- XY-[section-a]-0.png
//!        |         +- XY-[section-b]-0.png
//!        |- index.html
//!        |- css
//!        |    +- styles.css
//!        |- img
//!        |    |- icon.png
//!        |    +- logo.png
//!        |- js
//!        |    +- mockups.js
//!        |- iphone-portrait
//!        |    |- index.html
//!        |    +- section-a.html
//!        |- iphone-landscape
//!        |    |- index.html
//!        |    |- section-a.html
//!        |    +- section-b.html
//!        |- ipad-portrait
//!        |    |- index.html
//!        |    +- section-b.html
//!        +- ipad-landscape
//!             |- index.html
//!             |- section-a.html
//!             +- section-b.html
//! ```
//!
//! ## Usage
//!
//! ```
//! mockups -d "~/path/to/Project Name"
//! ```

#![feature(fs_walk)]

extern crate regex;
extern crate rustc_serialize;
extern crate docopt;
extern crate url;
extern crate image;
extern crate threadpool;
extern crate mustache;
extern crate sys_info;

use std::path::Path;
use docopt::Docopt;

mod structure;
mod site;
mod images;
mod utils;

static USAGE: &'static str = "
Usage: mockups -d <directory>
       mockups -h | --help
";

#[derive(RustcDecodable)]
struct Args {
    arg_directory: String
}

fn main() {
  let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(std::env::args().into_iter()).decode())
        .unwrap_or_else(|e| e.exit());

    let project_path = Path::new(&args.arg_directory);

    if !utils::is_dir(project_path) {
        println!("{:?} is not a directory", project_path);
        return
    }

    let mut categories = Vec::new();

    structure::read_directories(&project_path, &mut categories);
    site::generate(&project_path, &categories);
    images::generate_thumbs(&project_path, &categories);
}
