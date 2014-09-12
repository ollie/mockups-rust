//! Generate HTML static site from the data structure.

use std::io::{
    fs,
    File,
    UserDir,
    IoResult,
};

use structure::{
    Category,
    Section,
};

pub fn generate(project_path: &Path, categories: &Vec<Category>) {
    let site_path = project_path.join("site");

    // The site directory
    create_dir(&site_path);

    // The site/index.html file
    let site_index_path        = site_path.join("index.html");
    let site_index_file_result = create_file(&site_index_path);

    for category in categories.iter() {
        let category_path = site_path.join(category.file.clone());

        // The site/iphone-portrait directory
        create_dir(&category_path);

        // The site/iphone-portrait/index.html file
        let category_index_path        = category_path.join("index.html");
        let category_index_file_result = create_file(&category_index_path);

        for section in category.sections.iter() {
            let section_path = category_path.join(section.file.clone());

            // The site/iphone-portrait/dashboard.html file
            let section_file_result = create_file(&section_path);
            fill_in_section_file(section_file_result, categories, category, section);
        }
    }
}

fn create_dir(path: &Path) {
    if path.is_dir() {
        return;
    }

    let _ = fs::mkdir(path, UserDir);
}

fn create_file(path: &Path) -> IoResult<File> {
    File::create(path)
}

fn styles() -> String {
    String::from_str("
        sidebar {
          float: left;
          width: 15%;
        }

        main {
          float: right;
          width: 80%;
        }

        .detail-item {
          float: left;
          margin: 0 20px 20px 0;
          text-align: center;
        }
    ")
}

fn fill_in_section_file(
        file_result: IoResult<File>,
        categories:  &Vec<Category>,
        category:    &Category,
        section:     &Section
    ) {
    let mut file = file_result.unwrap();
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("  <head>\n");
    html.push_str("    <title>App Name</title>\n");
    html.push_str("    <style>\n");
    html.push_str(styles().as_slice());
    html.push_str("    </style>\n");
    html.push_str("  </head>\n");
    html.push_str("  <body>\n");
    html.push_str("    <header>\n");
    html.push_str("      <img src=\"../../icon.png\" alt=\"\">\n");
    html.push_str("      <p>App Name</p>\n");
    html.push_str("      <p>Tapmates</p>\n");
    html.push_str("    </header>\n");
    html.push_str("    <sidebar>\n");
    html.push_str("      <ul>\n");

    for subcategory in categories.iter() {
        html.push_str(        "        <li>\n");
        html.push_str(format!("          <a href=\"../{}/index.html\">\n", subcategory.file).as_slice());
        html.push_str(format!("            {}\n", subcategory.name).as_slice());
        html.push_str(        "          </a>\n");
        html.push_str(        "        </li>\n");
    }

    html.push_str("      </ul>\n");
    html.push_str("      <ul>\n");

    for subsection in category.sections.iter() {
        html.push_str(        "        <li>\n");
        html.push_str(format!("          <a href=\"{}\">\n", subsection.file).as_slice());
        html.push_str(format!("            {}\n", subsection.name).as_slice());
        html.push_str(        "          </a>\n");
        html.push_str(        "        </li>\n");
    }

    html.push_str("      </ul>\n");
    html.push_str("    </sidebar>\n");
    html.push_str("    <main>\n");

    for image in section.images.iter() {
        html.push_str(        "      <div class=\"detail-item\">\n");
        html.push_str(        "        <p>\n");
        html.push_str(format!("          <a href=\"../../{}/{}\">\n", category.file, image.file).as_slice());
        html.push_str(format!("            <img width=\"200\" src=\"../../{}/{}\" alt=\"\">\n", category.file, image.file).as_slice());
        html.push_str(        "          </a>\n");
        html.push_str(        "        </p>\n");
        html.push_str(format!("        <p>{}</p>\n", image.number).as_slice());
        html.push_str(        "      </div>\n");
    }

    html.push_str("    </main>\n");
    html.push_str("  </body>\n");
    html.push_str("</html>\n");

    let _ = file.write_line(html.as_slice());
}
