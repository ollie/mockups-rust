//! Copy images and resize them by half.

use std::io::File;
use std::sync::TaskPool;
use std::io::stdio::flush;
use std::os::num_cpus;

use image;

use image::{
    GenericImage,
    imageops,
};

use structure::Category;
use utils::create_dir;

/// Generate smaller versions of mockup images.
/// `iphone-portrait/XY-[section-a]-0.png -> site/thumbs/iphone-portrait/XY-[section-a]-0.png`
pub fn generate_thumbs(project_path: &Path, categories: &Vec<Category>) {
    let thumbs_path = project_path.join("site").join("thumbs");

    // Create thumbs directory
    create_dir(&thumbs_path);

    // Task pool so we don't overwhelm the system with hundreds of threads.
    // Use as many threads as there are CPU cores + the main thread.
    let f: || -> proc(uint):Send -> uint = || { proc(i) { i } };
    let mut task_pool = TaskPool::new(num_cpus(), f);

    // We need channels so we can wait until the tasks are done.
    let (tx, rx): (Sender<()>, Receiver<()>) = channel();
    let mut total = 0u8;

    // Loop over the categories/sections/images and spawn a new task
    // for thumb generation.
    for category in categories.iter() {
        let category_path = thumbs_path.join(category.file.clone());

        // The site/thumbs/iphone-portrait directory
        create_dir(&category_path);

        for section in category.sections.iter() {
            for image in section.images.iter() {
                let source_image_path = project_path
                    .join(image.category.clone())
                    .join(image.file.clone());

                let target_image_path = thumbs_path
                    .join(image.category.clone())
                    .join(image.file.clone());

                total += 1;
                let tx = tx.clone();

                task_pool.execute(proc(_) {
                    resize_image(source_image_path, target_image_path);
                    tx.send(());
                });
            }
        }
    }

    for _ in range(0u8, total) {
        rx.recv();
    }

    print!("\n");
}

fn resize_image(source_image_path: Path, target_image_path: Path) {
    let img             = image::open(&source_image_path).unwrap();
    let (width, height) = img.dimensions();
    let nwidth          = width / 2;
    let ratio           = nwidth as f64 / width as f64;
    let nheight         = (height as f64 * ratio).round() as u32;
    let resized_img     = img.resize(nwidth, nheight, imageops::Nearest);
    let fout            = File::create(&target_image_path);
    let _               = resized_img.save(fout, image::PNG);

    print!(".");
    flush();
    // println!("{} -> {}", source_image_path.display(), target_image_path.display());
}
