//! Copy images and resize them by half.

use std::fs::File;
use std::path::Path;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::io::Write;
use std::io;
use sys_info;

use image;

use image::{
    GenericImage,
    imageops,
};

use structure::Category;
use utils;

/// Generate smaller versions of mockup images.
/// `iphone-portrait/XY-[section-a]-0.png -> site/thumbs/iphone-portrait/XY-[section-a]-0.png`
pub fn generate_thumbs(project_path: &Path, categories: &Vec<Category>) {
    let thumbs_path = project_path.join("site").join("thumbs");

    // Create thumbs directory
    utils::create_dir(&thumbs_path);

    // Task pool so we don't overwhelm the system with hundreds of threads.
    // Use as many threads as there are CPU cores + the main thread.
    let num_cpus = sys_info::cpu_num().ok().expect("Cannot deternimne number of cores") as usize;
    let pool     = ThreadPool::new(num_cpus);

    // We need channels so we can wait until the tasks are done.
    let (tx, rx) = channel();
    let mut total = 0u8;

    // Loop over the categories/sections/images and spawn a new task
    // for thumb generation.
    for category in categories.iter() {
        let category_path = thumbs_path.join(category.file.clone());

        // The site/thumbs/iphone-portrait directory
        utils::create_dir(&category_path);

        for section in category.sections.iter() {
            for image in section.images.iter() {
                let source_image_path = project_path
                    .join(image.category.clone())
                    .join(image.file.clone());

                if !utils::is_file(&source_image_path) {
                    println!("{:?} does not exist!", source_image_path);
                    continue;
                }

                let target_image_path = thumbs_path
                    .join(image.category.clone())
                    .join(image.file.clone());

                total += 1;
                let tx = tx.clone();

                pool.execute(move || {
                    resize_image(&source_image_path, &target_image_path);
                    let _ = tx.send(());
                });
            }
        }
    }

    for _ in 0u8..total {
        let _ = rx.recv();
    }

    print!("\n");
}

fn resize_image(source_image_path: &Path, target_image_path: &Path) {
    let img = image::open(source_image_path)
        .ok()
        .expect(&format!("Image {:?} does not exist!", source_image_path));

    let (width, height) = img.dimensions();
    let nwidth          = width / 2;
    let ratio           = nwidth as f64 / width as f64;
    let nheight         = (height as f64 * ratio).round() as u32;
    let resized_img     = img.resize(nwidth, nheight, imageops::Nearest);

    let ref mut fout = File::create(target_image_path)
        .ok()
        .expect(&format!("Cannot create file {:?}", target_image_path));

    let _  = resized_img.save(fout, image::PNG)
        .ok()
        .expect(&format!("Cannot save image to {:?}", target_image_path));

    print!(".");
    // println!("{:?} -> {:?}", source_image_path, target_image_path);
    let _ = io::stdout().flush();
}
