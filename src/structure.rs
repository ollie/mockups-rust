//! Read file structure into data-structure.
//! Currently translates a 2-level structure into a 3-level structure.

use std::collections::HashMap;
use std::io::fs;
use std::io::fs::PathExtensions;
use std::char;

use url::percent_encoding::{
    FORM_URLENCODED_ENCODE_SET,
    utf8_percent_encode,
};

/// Eg `iPhone Portrait`, contains sections (which contain images).
#[deriving(Encodable)]
pub struct Category {
    pub file:     String,
    pub name:     String,
    pub sections: Vec<Section>,
}

/// Eg `Dashboard`, contains images.
#[deriving(Encodable)]
pub struct Section {
    pub file:   String,
    pub name:   String,
    pub class:  String,
    pub images: Vec<Image>,
}

/// Eg `XY-[dashboard]-1.png`.
#[deriving(Encodable)]
pub struct Image {
    pub category: String,
    pub file:     String,
    pub file_url: String,
    pub number:   u8,
}

impl Category {
    fn new(file: &str, name: &str) -> Category {
        Category {
            file:     file.to_string(),
            name:     name.to_string(),
            sections: Vec::new(),
        }
    }

    /// This is a poor man's version of HashMap's find_or_insert.
    /// I need additional fields on the datastructure so I went with a struct
    /// instead of a HashMap.
    fn add_section_image(&mut self, section_file: String, filename: &str, number: u8) {
        let mut section_file_w_ext = section_file.clone();
        section_file_w_ext.push_str(".html");

        for section in self.sections.iter_mut() {
            if section.file == section_file_w_ext {
                section.images.push(Image::new(self.file.clone(), filename, number));
                return;
            }
        }

        let section_name = self.name_from_file(section_file.clone());
        let mut section  = Section::new(section_file_w_ext, section_name, section_file);
        section.images.push(Image::new(self.file.clone(), filename, number));
        self.sections.push(section);
    }

    /// Take a filename (without extension), eg "new-post", split it by
    /// hyphen ["new", "post"], capitalize words ["New", "Post"], join them
    /// by space "New Post".
    fn name_from_file(&self, file: String) -> String {
        let mut words: Vec<&str> = file.as_slice().split('-').collect();
        let mut cap_words        = Vec::new();

        for word in words.iter_mut() {
            let mut cap_word = char::to_uppercase(word.char_at(0)).to_string();
            let rest         = word.slice_chars(1, word.len());

            cap_word.push_str(rest);
            cap_words.push(cap_word);
        }

        cap_words.connect(" ")
    }
}

impl Section {
    fn new(file: String, name: String, class: String) -> Section {
        Section {
            file:   file,
            name:   name,
            class:  class,
            images: Vec::new(),
        }
    }
}

impl Image {
    fn new(category: String, file: &str, number: u8) -> Image {
        Image {
            category: category,
            file:     String::from_str(file),
            file_url: utf8_percent_encode(file.as_slice(), FORM_URLENCODED_ENCODE_SET),
            number:   number
        }
    }
}

/// Those categories are given and some of them may not be present.
pub fn read_directories(project_path: &Path, categories: &mut Vec<Category>) {
    let mut possible_categories = HashMap::new();

    possible_categories.insert("iphone-portrait",  "iPhone Portrait");
    possible_categories.insert("iphone-landscape", "iPhone Landscape");
    possible_categories.insert("ipad-portrait",    "iPad Portrait");
    possible_categories.insert("ipad-landscape",   "iPad Landscape");

    for (category_file, category_name) in possible_categories.iter() {
        let category_path = project_path.join(*category_file);

        if !category_path.is_dir() {
            continue;
        }

        let mut category = Category::new(*category_file, *category_name);

        read_images(category_path, &mut category);
        categories.push(category);
    }

    categories.sort_by(|a, b| a.name.cmp(&b.name) );
}

/// Extract the section name from the image and insert it into the category.
fn read_images(category_path: Path, category: &mut Category) {
    let mut paths = match fs::walk_dir(&category_path) {
        Ok(paths) => paths,
        Err(_)    => return
    };

    let image_regex = regex!(r"\A[A-Z]{2}-\[(?P<section>[\w-]+)\]-(?P<number>\d+)\.png\z");

    for path in paths {
        if !path.is_file() {
            continue;
        }

        let filename = path.filename_str().unwrap();

        match image_regex.captures(filename) {
            Some(caps) => {
                let section_file = caps.name("section").to_string();
                let number: u8   = from_str(caps.name("number")).unwrap();

                category.add_section_image(section_file, filename, number);
            },
            None => ()
        }
    }

    category.sections.sort_by(|a, b| a.name.cmp(&b.name) );

    for section in category.sections.iter_mut() {
        section.images.sort_by(|a, b| a.number.cmp(&b.number) );
    }
}
