//! Read file structure into datastructure.

use std::collections::HashMap;
use std::io::fs;
use std::char;
use url::percent_encoding::{
    FORM_URLENCODED_ENCODE_SET,
    utf8_percent_encode,
};

#[deriving(Encodable)]
pub struct Category {
    pub file:     String,
    pub name:     String,
    pub sections: Vec<Section>,
}

#[deriving(Encodable)]
pub struct Section {
    pub file:   String,
    pub name:   String,
    pub class:  String,
    pub images: Vec<Image>,
}

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

    fn add_section_image(&mut self, section_file: String, filename: &str, number: u8) {
        let section_file_w_ext = section_file.clone().append(".html");

        for section in self.sections.mut_iter() {
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

    fn name_from_file(&self, file: String) -> String {
        let mut words: Vec<&str> = file.as_slice().split('-').collect();
        let mut cap_words        = Vec::new();

        for word in words.mut_iter() {
            let first = char::to_uppercase(word.char_at(0)).to_string();
            let rest  = word.slice_chars(1, word.len());

            let cap_word = first.append(rest);
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

pub fn read_directories(project_path: &Path, categories: &mut Vec<Category>) {
    let mut possible_categories = HashMap::new();

    possible_categories.insert("iphone-portrait", "iPhone Portrait");
    possible_categories.insert("iphone-landscape", "iPhone Landscape");
    possible_categories.insert("ipad-portrait", "iPad Portrait");
    possible_categories.insert("ipad-landscape", "iPad Landscape");

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

    for section in category.sections.mut_iter() {
        section.images.sort_by(|a, b| a.number.cmp(&b.number) );
    }
}
