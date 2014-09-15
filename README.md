# Mockups Static Site Generator

Walk down a project directory and generate a static HTML site containing
resized images.

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

## Usage

    $ mockups -d "~/path/to/Project Name"

## Dependencies

* [Rust][rust-url]
* [Cargo][cargo-url]

[rust-url]:  https://github.com/rust-lang/rust
[cargo-url]: https://github.com/rust-lang/cargo

### Installation

    $ git clone ...
    $ cd ...
    $ cargo build
    $ cargo build --release # Optimized binary

## TODO

* Refactor site generator a bit?
