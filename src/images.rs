// use std::io::File;

// use glob::{
//     glob_with,
//     Paths,
//     MatchOptions,
// };

// use image;

// use image::{
//     GenericImage,
//     imageops,
// };

// pub fn resize_list(directory_path: Path) {
//     let mut images_list = list_images(directory_path);

//     let mut sections = Vec::new();
//     let mut section = Section::new("iphone-portrait", "iPhone Portrait");

//     section.add_image(Image::new("XY-[section-a]-0.png", 0));
//     section.add_image(Image::new("XY-[section-a]-1.png", 1));
//     section.add_image(Image::new("XY-[section-a]-2.png", 2));

//     sections.push(section);
//     //println!("{}", sections);

//     for image_path in images_list {
//         spawn(proc() {
//             resize_image(image_path);
//         });
//     }
// }

// fn list_images(directory_path: Path) -> Paths {
//     let pattern           = directory_path.join("*[0-9].png");
//     let mut match_options = MatchOptions::new();
//     match_options.case_sensitive = false;

//     glob_with(pattern.as_str().unwrap(), match_options)
// }

// fn resize_image(image_path: Path) {
//     let file_name         = image_path.filestem_str().unwrap().as_string();
//     let file_ext          = image_path.extension_str().unwrap();
//     let new_file_name_ext = file_name.append("-resized").append(".").append(file_ext);

//     let mut cloned_image_path = image_path.clone();
//     cloned_image_path.pop();
//     let new_image_path = cloned_image_path.join(new_file_name_ext.as_slice());

//     print!("{} ->", image_path.display());

//     let img             = image::open(&Path::new(image_path.clone())).unwrap();
//     let (width, height) = img.dimensions();
//     let nwidth          = 320;
//     let ratio           = nwidth as f64 / width as f64;
//     let nheight         = (height as f64 * ratio).round() as u32;
//     let resized_img     = img.resize(nwidth, nheight, imageops::Nearest);
//     let fout            = File::create(&new_image_path.clone()).unwrap();
//     let _               = resized_img.save(fout, image::PNG);

//     print!(" {}\n", new_image_path.display());
// }
