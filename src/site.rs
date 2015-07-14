//! Generate HTML static site from the data structure.

use std::fs::File;
use std::path::Path;
use std::io;
use std::io::Write;

use mustache;
use mustache::{
    MapBuilder,
    VecBuilder,
};

use structure::{
    Category,
    Section,
};

use utils::{
    is_file,
    create_dir,
    create_file,
};

/// Generate the HTML file and directory structure. External assets like
/// styles, images and JavaScripts need to be embedded in the binary
/// as they are not available at run-time.
pub fn generate(project_path: &Path, categories: &Vec<Category>) {
    let site_path   = project_path.join("site");
    let icon_path   = project_path.join("mockups").join("icon.png");
    let icon_exists = &is_file(&icon_path);

    // App name is the name of the directory
    let app_name = project_path.file_name().unwrap().to_str().unwrap();

    // The site directory
    create_dir(&site_path);

    // CSS files and images
    copy_assets(&site_path);

    // The site/index.html file
    let site_index_path        = site_path.join("index.html");
    let site_index_file_result = create_file(&site_index_path);
    fill_in_site_index_file(site_index_file_result, app_name, icon_exists, categories);

    for category in categories.iter() {
        let category_path = site_path.join(category.file.clone());

        // The site/iphone-portrait directory
        create_dir(&category_path);

        // The site/iphone-portrait/index.html file
        let category_index_path        = category_path.join("index.html");
        let category_index_file_result = create_file(&category_index_path);
        fill_in_category_index_file(category_index_file_result, app_name, icon_exists, categories, category);

        for section in category.sections.iter() {
            let section_path = category_path.join(section.file.clone());

            // The site/iphone-portrait/dashboard.html file
            let section_file_result = create_file(&section_path);
            fill_in_section_file(section_file_result, app_name, icon_exists, categories, category, section);
        }
    }
}

fn copy_assets(site_path: &Path) {
    copy_styles_css(&site_path.join("css"));
    // copy_styles_less(&site_path.join("css"));
    // copy_less_min_js(&site_path.join("js"));
    copy_logo_img(&site_path.join("img"));
    copy_icon_img(&site_path.join("img"));
}

fn copy_styles_css(css_path: &Path) {
    create_dir(css_path);

    let target_path     = css_path.join("styles.css");
    let mut target_file = File::create(&target_path).unwrap();
    let data            = include_str!("css/styles.css");
    let _               = target_file.write_all(data.as_bytes()).unwrap();
}

// For development purposes
// fn copy_styles_less(css_path: &Path) {
//     create_dir(css_path);
//
//     let target_path     = css_path.join("styles.less");
//     let mut target_file = File::create(&target_path).unwrap();
//     let data            = include_str!("css/styles.less");
//     let _               = target_file.write_all(data.as_bytes()).unwrap();
// }

// For development purposes
// fn copy_less_min_js(js_path: &Path) {
//     create_dir(js_path);
//
//     let target_path     = js_path.join("less.min.js");
//     let mut target_file = File::create(&target_path).unwrap();
//     let data            = include_str!("js/less.min.js");
//     let _               = target_file.write_all(data.as_bytes()).unwrap();
// }

fn copy_logo_img(img_path: &Path) {
    create_dir(img_path);

    let target_path     = img_path.join("logo.png");
    let mut target_file = File::create(&target_path).unwrap();
    let data            = include_bytes!("img/logo.png");
    let _               = target_file.write_all(data).unwrap();
}

fn copy_icon_img(img_path: &Path) {
    create_dir(img_path);

    let target_path     = img_path.join("icon.png");
    let mut target_file = File::create(&target_path).unwrap();
    let data            = include_bytes!("img/icon.png");
    let _               = target_file.write_all(data).unwrap();
}

fn aside_categories(categories: &Vec<Category>, selected: Option<String>) -> VecBuilder {
    let selected_category = selected.unwrap_or(String::new());

    let mut builder = VecBuilder::new();

    for category in categories.iter() {
        builder = builder.push_map(|builder| {
            builder
                .insert_str("file",      category.file.clone())
                .insert_str("name",      category.name.clone())
                .insert_bool("selected", selected_category == category.name.clone())
        });
    }

    builder
}

fn aside_sections(sections: &Vec<Section>, selected: Option<String>) -> VecBuilder {
    let selected_section = selected.unwrap_or(String::new());

    let mut builder = VecBuilder::new();

    for section in sections.iter() {
        builder = builder.push_map(|builder| {
            builder
                .insert_str("file",      section.file.clone())
                .insert_str("name",      section.name.clone())
                .insert_str("class",     section.class.clone())
                .insert_bool("selected", selected_section == section.name.clone())
        });
    }

    builder
}

fn fill_in_site_index_file(
    file_result: io::Result<File>,
    app_name:    &str,
    icon_exists: &bool,
    categories:  &Vec<Category>
) {
    let mut file = file_result.unwrap();

    let data = MapBuilder::new()
        .insert_str("app_name",         app_name.clone())
        .insert_bool("icon_exists",     icon_exists.clone())
        .insert_vec("aside_categories", |_| {
            aside_categories(categories, None)
        })
        .build();

    let template = mustache::compile_str(include_str!("templates/site.mustache"));
    let _        = template.render_data(&mut file, &data);
}

fn fill_in_category_index_file(
    file_result: io::Result<File>,
    app_name:    &str,
    icon_exists: &bool,
    categories:  &Vec<Category>,
    category:    &Category
) {
    let mut file = file_result.unwrap();

    let data = MapBuilder::new()
        .insert_str("app_name",         app_name.clone())
        .insert_bool("icon_exists",     icon_exists.clone())
        .insert_str("category_name",    category.name.clone())
        .insert_vec("aside_categories", |_| {
            aside_categories(categories, Some(category.name.clone()))
        })
        .insert_vec("aside_sections", |_| {
            aside_sections(&category.sections, None)
        })
        .insert_vec("sections", |mut builder| {
            for section in category.sections.iter() {
                builder = builder.push_map(|builder| {
                    builder
                        .insert_str("file",  section.file.clone())
                        .insert_str("name",  section.name.clone())
                        .insert_str("class", section.class.clone())
                        .insert_vec("images", |mut builder| {
                            for image in section.images.iter() {
                                builder = builder.push_map(|builder| {
                                    builder
                                        .insert_str("category", image.category.clone())
                                        .insert_str("file",     image.file.clone())
                                        .insert_str("file_url", image.file_url.clone())
                                        .insert("number",       &image.number).unwrap()
                                });
                            }

                            builder
                        })
                });
            }

            builder
        })
        .build();

    let template = mustache::compile_str(include_str!("templates/category.mustache"));
    let _        = template.render_data(&mut file, &data);
}

fn fill_in_section_file(
    file_result: io::Result<File>,
    app_name:    &str,
    icon_exists: &bool,
    categories:  &Vec<Category>,
    category:    &Category,
    section:     &Section
) {
    let mut file = file_result.unwrap();

    let data = MapBuilder::new()
        .insert_str("app_name",      app_name.clone())
        .insert_bool("icon_exists",  icon_exists.clone())
        .insert_str("category_name", category.name.clone())
        .insert_str("section_name",  section.name.clone())
        .insert_vec("aside_categories", |_| {
            aside_categories(categories, Some(category.name.clone()))
        })
        .insert_vec("aside_sections", |_| {
            aside_sections(&category.sections, Some(section.name.clone()))
        })
        .insert_vec("images", |mut builder| {
            for image in section.images.iter() {
                builder = builder.push_map(|builder| {
                    builder
                        .insert_str("category", image.category.clone())
                        .insert_str("file",     image.file.clone())
                        .insert_str("file_url", image.file_url.clone())
                        .insert("number",       &image.number).unwrap()
                });
            }

            builder
        })
        .build();

    let template = mustache::compile_str(include_str!("templates/section.mustache"));
    let _        = template.render_data(&mut file, &data);
}
