//! Walk down a project directory and generate a static HTML site containing
//! resized images.
//!
//! ## File structure before site is generated
//!
//! ```
//! Project Name
//!   |- icon.png
//!   |- iphone-portrait
//!   |    |- XY-[section-a]-0.png
//!   |    |- XY-[section-a]-1.png
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
//!   |- icon.png
//!   |- iphone-portrait
//!   |    |- XY-[section-a]-0.png
//!   |    |- XY-[section-a]-1.png
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
//!        |    |    |- XY-[section-a]-1.png
//!        |    |- iphone-landscape
//!        |    |    |- XY-[section-a]-0.png
//!        |    |    +- XY-[section-b]-0.png
//!        |    |- ipad-portrait
//!        |    |    +- XY-[section-b]-0.png
//!        |    +- ipad-landscape
//!        |         |- XY-[section-a]-0.png
//!        |         +- XY-[section-b]-0.png
//!        |- index.html
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
//!             |- section-a.html
//!             +- section-b.html
//! ```
//!
//! ## Usage
//!
//! ```
//! mockups -d "~/path/to/Project Name"
//! ```

#![feature(phase)]

// Shipped with Rust
extern crate getopts;
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate serialize;

// External libraries
extern crate url;
extern crate image;
extern crate mustache;

mod options;
mod structure;
mod site;
mod images;
mod utils;

fn main() {
    let options        = options::get_options();
    let project_path   = Path::new(options.opt_str("d").unwrap());
    let mut categories = Vec::new();

    structure::read_directories(&project_path, &mut categories);
    site::generate(&project_path, &categories);
    images::generate_thumbs(&project_path, &categories);
}