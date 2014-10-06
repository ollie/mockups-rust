//! Generate HTML static site from the data structure.

use std::io::{
    File,
    IoResult,
};

use std::io::fs::PathExtensions;

use mustache;

use structure::{
    Category,
    Section,
    Image,
};

use utils::{
    create_dir,
    create_file,
};

#[deriving(Encodable)]
struct SiteData<'a> {
    app_name:    &'a str,
    icon_exists: &'a bool,
    categories:  &'a Vec<Category>,
}

#[deriving(Encodable)]
struct CategoryData<'a> {
    app_name:    &'a str,
    icon_exists: &'a bool,
    categories:  &'a Vec<Category>,
    category:    &'a Category,
    sections:    &'a Vec<Section>,
}

#[deriving(Encodable)]
struct SectionData<'a> {
    app_name:    &'a str,
    icon_exists: &'a bool,
    categories:  &'a Vec<Category>,
    category:    &'a Category,
    sections:    &'a Vec<Section>,
    section:     &'a Section,
    images:      &'a Vec<Image>,
}

/// Generate the HTML file and directory structure. External assets like
/// styles, images and JavaScripts need to be embedded in the binary
/// as they are not available at run-time.
pub fn generate(project_path: &Path, categories: &Vec<Category>) {
    let site_path   = project_path.join("site");
    let icon_path   = project_path.join("mockups").join("icon.png");
    let icon_exists = &icon_path.is_file();

    // App name is the name of the directory
    let app_name = project_path.filename_str().unwrap();

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
    copy_styles_css(site_path.join("css"));
    // copy_styles_less(site_path.join("css"));
    copy_mockups_js(site_path.join("js"));
    // copy_less_min_js(site_path.join("js"));
    copy_logo_img(site_path.join("img"));
    copy_icon_img(site_path.join("img"));
}

fn copy_styles_css(css_path: Path) {
    create_dir(&css_path);

    let target_path     = Path::new(css_path.join("styles.css"));
    let mut target_file = File::create(&target_path);
    let data            = include_str!("css/styles.css");
    let _               = target_file.write(data.as_bytes()).unwrap();
}

// For development purposes
// fn copy_styles_less(css_path: Path) {
//     create_dir(&css_path);
//
//     let target_path     = Path::new(css_path.join("styles.less"));
//     let mut target_file = File::create(&target_path);
//     let data            = include_str!("css/styles.less");
//     let _               = target_file.write(data.as_bytes()).unwrap();
// }

fn copy_mockups_js(js_path: Path) {
    create_dir(&js_path);

    let target_path     = Path::new(js_path.join("mockups.js"));
    let mut target_file = File::create(&target_path);
    let data            = include_str!("js/mockups.js");
    let _               = target_file.write(data.as_bytes()).unwrap();
}

// For development purposes
// fn copy_less_min_js(js_path: Path) {
//     create_dir(&js_path);
//
//     let target_path     = Path::new(js_path.join("less.min.js"));
//     let mut target_file = File::create(&target_path);
//     let data            = include_str!("js/less.min.js");
//     let _               = target_file.write(data.as_bytes()).unwrap();
// }

fn copy_logo_img(img_path: Path) {
    create_dir(&img_path);

    let target_path     = Path::new(img_path.join("logo.png"));
    let mut target_file = File::create(&target_path);
    let data            = include_bin!("img/logo.png");
    let _               = target_file.write(data).unwrap();
}

fn copy_icon_img(img_path: Path) {
    create_dir(&img_path);

    let target_path     = Path::new(img_path.join("icon.png"));
    let mut target_file = File::create(&target_path);
    let data            = include_bin!("img/icon.png");
    let _               = target_file.write(data).unwrap();
}

fn fill_in_site_index_file(
    file_result: IoResult<File>,
    app_name:    &str,
    icon_exists: &bool,
    categories:  &Vec<Category>
) {
    let mut file = file_result.unwrap();

    let data = SiteData {
        app_name:    app_name,
        icon_exists: icon_exists,
        categories:  categories,
    };

    let template = mustache::compile_str(include_str!("templates/site.mustache"));
    let _        = template.render(&mut file, &data);
}

fn fill_in_category_index_file(
    file_result: IoResult<File>,
    app_name:    &str,
    icon_exists: &bool,
    categories:  &Vec<Category>,
    category:    &Category
) {
    let mut file = file_result.unwrap();

    let data = CategoryData {
        app_name:    app_name,
        icon_exists: icon_exists,
        categories:  categories,
        category:    category,
        sections:    &category.sections,
    };

    let template = mustache::compile_str(include_str!("templates/category.mustache"));
    let _        = template.render(&mut file, &data);
}

fn fill_in_section_file(
    file_result: IoResult<File>,
    app_name:    &str,
    icon_exists: &bool,
    categories:  &Vec<Category>,
    category:    &Category,
    section:     &Section
) {
    let mut file = file_result.unwrap();

    let data = SectionData {
        app_name:    app_name,
        icon_exists: icon_exists,
        categories:  categories,
        category:    category,
        sections:    &category.sections,
        section:     section,
        images:      &section.images,
    };

    let template = mustache::compile_str(include_str!("templates/section.mustache"));
    let _        = template.render(&mut file, &data);
}
