#![allow(dead_code)]

use std::fs;
use std::path::Path;
use image_lib;
use std::cmp;
use image_lib::GenericImage;
use image_lib::imageops::overlay;

/// Information about one of the embedded images.
#[derive(Debug)]
pub struct SpriteRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub file_name: String,
    pub name: String
}

// Hold information about a generated sprite map.
pub struct SpriteMap {
    pub width: u32,
    pub height: u32,
    pub regions: Vec<SpriteRegion>,
    pub url: String
}

impl SpriteMap {

    // Read all the image sizes and initialize the regions
    // with the proper layout information.
    // Returns (width, height, regions)
    fn layout(glob:&str) -> (u32,u32,Vec<SpriteRegion>) {
        let paths = fs::read_dir(&Path::new(glob)).unwrap();

        let mut regions:Vec<SpriteRegion> = vec!();
        let mut width = 0;
        let mut height = 0;
        for entry in paths {
            let path = entry.unwrap().path();
            if path.extension().unwrap() == "png" {
                match image_lib::open(&path) {
                    Ok(img) => {
                        let (image_width,image_height) = img.dimensions();
                        let region = SpriteRegion {
                            x: width,
                            y:0,
                            width: image_width,
                            height: image_height,
                            file_name: path.to_str().unwrap().to_string(),
                            name: path.file_stem().unwrap().to_str().unwrap().to_string()
                        };
                        regions.push(region);
                        width = width + image_width;
                        height = cmp::max(height, image_height);
                    },
                    Err(_) => {

                    }
                }
            }
        }
        (width, height, regions)

    }

    // Render a previously laid out SpriteMap.
    fn render(layout:(u32,u32,Vec<SpriteRegion>),output:&Path) -> SpriteMap {
        let mut generated = image_lib::ImageBuffer::new(layout.0, layout.1);
        for region in layout.2.iter() {
            let image_path = Path::new(&region.file_name);
            match image_lib::open(&image_path) {
                Ok(image) => {
                    let buffer = image.to_rgba();
                    overlay( &mut generated, &buffer, region.x, region.y)
                },
                Err(_) => println!("Cannot load image {}", &region.file_name)
            }

        }
        let ref mut fout = fs::File::create(output).unwrap();
        let generated_image = image_lib::ImageRgba8(generated);
        let _ = generated_image.save(fout, image_lib::PNG);
        SpriteMap {
            width: layout.0,
            height: layout.1,
            regions: layout.2,
            url: output.to_str().unwrap().to_string()
        }
    }

    // Generates a css sprite map from the files matching the glob pattern.
    // Only PNG files can be made into css sprites at this time.
    pub fn build(glob:&str,output:&Path) -> SpriteMap {
        let layout = SpriteMap::layout(glob);
        let sprite_map = SpriteMap::render(layout, output);
        sprite_map
    }

    // Returns the image and background position for use in a single shorthand property.
    // background: spriteMap.sprite("name").
    pub fn sprite(&self, name:&str) -> String {
        name.to_string()
    }
}

#[test]
fn builds() {
    let sprite_map = SpriteMap::build("data/my-icons",&Path::new("data/out.png"));
    let mut names:Vec<&str> = sprite_map.regions.iter().map(|r| &*r.name).collect();
    names.sort();
    assert_eq!(names,vec!("edit1","edit2","edit3"));
    assert_eq!(sprite_map.height, 32 as u32);
    assert_eq!(sprite_map.width, (names.len()*32) as u32);
}
