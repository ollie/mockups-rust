# Mockups Static Site Generator

Walk down a project directory and generate a static HTML site containing
resized images.

## Usage

Go to the [releases][releases] section and download the latest binary to your
favorite location, say "Downloads", then open Terminal and enter:

    $ cd Downloads
    $ ./mockups -d "path/to/Your Project"

Note: You may want to simply type `./mockups -d ` and then drag and drop
your project directory from Finder into the Terminal and it will fill in
the path to it.

## File structure before site is generated

    Project Name
      |- icon.png
      |- iphone-portrait
      |    |- XY-[section-a]-0.png
      |    |- XY-[section-a]-1.png
      |- iphone-landscape
      |    |- XY-[section-a]-0.png
      |    +- XY-[section-b]-0.png
      |- ipad-portrait
      |    +- XY-[section-b]-0.png
      +- ipad-landscape
           |- XY-[section-a]-0.png
           +- XY-[section-b]-0.png


## File structure after site is generated

    Project Name
      |- icon.png
      |- iphone-portrait
      |    |- XY-[section-a]-0.png
      |    |- XY-[section-a]-1.png
      |- iphone-landscape
      |    |- XY-[section-a]-0.png
      |    +- XY-[section-b]-0.png
      |- ipad-portrait
      |    +- XY-[section-b]-0.png
      |- ipad-landscape
      |    |- XY-[section-a]-0.png
      |    +- XY-[section-b]-0.png
      +- site
           |- thumbs
           |    |- iphone-portrait
           |    |    |- XY-[section-a]-0.png
           |    |    |- XY-[section-a]-1.png
           |    |- iphone-landscape
           |    |    |- XY-[section-a]-0.png
           |    |    +- XY-[section-b]-0.png
           |    |- ipad-portrait
           |    |    +- XY-[section-b]-0.png
           |    +- ipad-landscape
           |         |- XY-[section-a]-0.png
           |         +- XY-[section-b]-0.png
           |- index.html
           |- iphone-portrait
           |    |- index.html
           |    +- section-a.html
           |- iphone-landscape
           |    |- index.html
           |    |- section-a.html
           |    +- section-b.html
           |- ipad-portrait
           |    |- index.html
           |    +- section-b.html
           +- ipad-landscape
                |- section-a.html
                +- section-b.html

## Installation

Install dependencies:

* [Rust][rust-url]
* [Cargo][cargo-url]

And then build the crate:

    $ git clone https://github.com/ollie/mockups-rust mockups
    $ cd mockups
    $ cargo build           # Not optimized binary in target/mockups
    $ cargo build --release # Optimized binary in target/mockups/release

## TODO

* Refactor site generator a bit?

[releases]:  https://github.com/ollie/mockups-rust/releases
[rust-url]:  https://github.com/rust-lang/rust
[cargo-url]: https://github.com/rust-lang/cargo