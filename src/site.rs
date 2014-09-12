//! Generate HTML static site from the data structure.

use std::io::{
    File,
    IoResult,
};

use structure::{
    Category,
    Section,
    Image,
};

use utils::{
    create_dir,
    create_file,
};

use mustache;

#[deriving(Encodable)]
struct SiteData<'a> {
    categories: &'a Vec<Category>,
}

#[deriving(Encodable)]
struct CategoryData<'a> {
    categories: &'a Vec<Category>,
    category:   &'a Category,
    sections:   &'a Vec<Section>,
}

#[deriving(Encodable)]
struct SectionData<'a> {
    categories: &'a Vec<Category>,
    category:   &'a Category,
    sections:   &'a Vec<Section>,
    section:    &'a Section,
    images:     &'a Vec<Image>,
}

pub fn generate(project_path: &Path, categories: &Vec<Category>) {
    let site_path = project_path.join("site");

    // The site directory
    create_dir(&site_path);

    // The site/index.html file
    let site_index_path        = site_path.join("index.html");
    let site_index_file_result = create_file(&site_index_path);
    fill_in_site_index_file(site_index_file_result, categories);

    for category in categories.iter() {
        let category_path = site_path.join(category.file.clone());

        // The site/iphone-portrait directory
        create_dir(&category_path);

        // The site/iphone-portrait/index.html file
        let category_index_path        = category_path.join("index.html");
        let category_index_file_result = create_file(&category_index_path);
        fill_in_category_index_file(category_index_file_result, categories, category);

        for section in category.sections.iter() {
            let section_path = category_path.join(section.file.clone());

            // The site/iphone-portrait/dashboard.html file
            let section_file_result = create_file(&section_path);
            fill_in_section_file(section_file_result, categories, category, section);
        }
    }
}

fn fill_in_site_index_file(
        file_result: IoResult<File>,
        categories:  &Vec<Category>
    ) {
    let mut file = file_result.unwrap();

    let data = SiteData {
        categories: categories,
    };

    let template = mustache::compile_str("<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <title>App Name</title>
    <style>
      aside {
        float: left;
        width: 15%;
      }

      main {
        float: right;
        width: 80%;
      }

      .section-item {
        overflow: hidden;
      }

      .detail-item {
        float: left;
        margin: 0 20px 20px 0;
        text-align: center;
      }
    </style>
  </head>
  <body>

    <header>
      <img src=\"../../icon.png\" alt=\"\">
      <p>App Name</p>
      <p>Tapmates</p>
    </header>

    <aside>
      <ul>
      {{#categories}}
        <li>
          <a href=\"{{file}}/index.html\">{{name}}</a>
        </li>
      {{/categories}}
      </ul>
    </aside>

    <main>
      <h1>Hello :)</h1>
    </main>

  </body>
</html>
");

    let _ = template.render(&mut file, &data);
}

fn fill_in_category_index_file(
        file_result: IoResult<File>,
        categories:  &Vec<Category>,
        category:    &Category
    ) {
    let mut file = file_result.unwrap();

    let data = CategoryData {
        categories: categories,
        category:   category,
        sections:   &category.sections,
    };

    let template = mustache::compile_str("<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <title>App Name</title>
    <style>
      aside {
        float: left;
        width: 15%;
      }

      main {
        float: right;
        width: 80%;
      }

      .section-item {
        overflow: hidden;
      }

      .detail-item {
        float: left;
        margin: 0 20px 20px 0;
        text-align: center;
      }
    </style>
  </head>
  <body>

    <header>
      <img src=\"../../icon.png\" alt=\"\">
      <p>App Name</p>
      <p>Tapmates</p>
    </header>

    <aside>
      <ul>
      {{#categories}}
        <li>
          <a href=\"../{{file}}/index.html\">{{name}}</a>
        </li>
      {{/categories}}
      </ul>
      <ul>
      {{#sections}}
        <li>
          <a href=\"{{file}}\">{{name}}</a>
        </li>
      {{/sections}}
      </ul>
    </aside>

    <main>
    {{#category}}
      <h1>{{name}}</h1>
    {{/category}}
    {{#sections}}
      <div class=\"section-item\">
        <h2>
          <a href=\"{{file}}\">{{name}}</a>
        </h2>
      {{#images}}
        <div class=\"detail-item\">
          <p>
            <a href=\"../../{{category}}/{{file_url}}\">
              <img width=\"200\" src=\"../../{{category}}/{{file_url}}\" alt=\"\">
            </a>
          </p>
          <p>{{number}}</p>
        </div>
      {{/images}}
      </div>
    {{/sections}}
    </main>

  </body>
</html>
");

    let _ = template.render(&mut file, &data);
}

fn fill_in_section_file(
        file_result: IoResult<File>,
        categories:  &Vec<Category>,
        category:    &Category,
        section:     &Section
    ) {
    let mut file = file_result.unwrap();

    let data = SectionData {
        categories: categories,
        category:   category,
        sections:   &category.sections,
        section:    section,
        images:     &section.images,
    };

    let template = mustache::compile_str("<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <title>App Name</title>
    <style>
      aside {
        float: left;
        width: 15%;
      }

      main {
        float: right;
        width: 80%;
      }

      .section-item {
        overflow: hidden;
      }

      .detail-item {
        float: left;
        margin: 0 20px 20px 0;
        text-align: center;
      }
    </style>
  </head>
  <body>

    <header>
      <img src=\"../../icon.png\" alt=\"\">
      <p>App Name</p>
      <p>Tapmates</p>
    </header>

    <aside>
      <ul>
      {{#categories}}
        <li><a href=\"../{{file}}/index.html\">{{name}}</a></li>
      {{/categories}}
      </ul>
      <ul>
      {{#sections}}
        <li><a href=\"{{file}}\">{{name}}</a></li>
      {{/sections}}
      </ul>
    </aside>

    <main>
      <h1>
      {{#category}}
        <a href=\"../{{file}}/index.html\">{{name}}</a>
      {{/category}}
        &gt;
      {{#section}}
        {{name}}
      {{/section}}
      </h1>
    {{#images}}
      <div class=\"detail-item\">
        <p>
          <a href=\"../../{{category}}/{{file_url}}\">
            <img width=\"200\" src=\"../../{{category}}/{{file_url}}\" alt=\"\">
          </a>
        </p>
        <p>{{number}}</p>
      </div>
    {{/images}}
    </main>

  </body>
</html>
");

    let _ = template.render(&mut file, &data);
}
