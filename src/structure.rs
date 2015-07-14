//! Read file structure into data-structure.
//! Currently translates a 2-level structure into a 3-level structure.

use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::str::FromStr;
use std::string::ToString;
use regex::Regex;

use url::percent_encoding::{
    FORM_URLENCODED_ENCODE_SET,
    utf8_percent_encode,
};

use utils;

/// Eg `iPhone Portrait`, contains sections (which contain images).
pub struct Category {
    pub file:     String,
    pub name:     String,
    pub sections: Vec<Section>,
}

/// Eg `Dashboard`, contains images.
pub struct Section {
    pub file:   String,
    pub name:   String,
    pub class:  String,
    pub images: Vec<Image>,
}

/// Eg `XY-[dashboard]-1.png`.
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
        file
            .split('-')
            .map(|word| {
                let mut capitalized = String::new();

                for (index, ch) in word.char_indices() {
                    if index == 0 {
                        capitalized.push_str(&ch.to_uppercase().collect::<String>());
                    } else {
                        capitalized.push(ch);
                    };
                }

                capitalized
            })
            .collect::<Vec<String>>()
            .connect(" ")
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
            file:     file.to_string(),
            file_url: utf8_percent_encode(file, FORM_URLENCODED_ENCODE_SET),
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

        if !utils::is_dir(&category_path) {
            println!("{:?} not found", category_path);
            continue;
        }

        let mut category = Category::new(*category_file, *category_name);

        read_images(&category_path, &mut category);
        categories.push(category);
    }

    categories.sort_by(|a, b| a.name.cmp(&b.name) );
}

/// Extract the section name from the image and insert it into the category.
fn read_images(category_path: &Path, category: &mut Category) {
    let paths = match fs::walk_dir(&category_path) {
        Ok(paths) => paths.map(|path| path.unwrap()),
        Err(_)    => return
    };

    let image_regex = Regex::new(r"\A[A-Z]{2}-\[(?P<section>[\w-]+)\]-(?P<number>\d+)\.png\z").unwrap();

    for path in paths {
        if !utils::is_file(&path.path()) {
            continue;
        }

        let filename = &path.file_name().into_string().unwrap();

        match image_regex.captures(filename) {
            Some(caps) => {
                let section_file = ToString::to_string(caps.name("section").unwrap());
                let number: u8   = FromStr::from_str(caps.name("number").unwrap()).unwrap();

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
