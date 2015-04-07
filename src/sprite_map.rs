#![allow(dead_code)]

use std::fs;
use std::path::Path;
use image_lib;
use std::cmp;
use image_lib::GenericImage;

// Hold information about a built sprite map.
pub struct SpriteMap {
    pub width: u32,
    pub height: u32,
    pub names: Vec<String>,
    pub url: String
}

impl SpriteMap {
    // Generates a css sprite map from the files matching the glob pattern.
    // Only PNG files can be made into css sprites at this time.
    pub fn build(glob:&str) -> SpriteMap {
        let paths = fs::read_dir(&Path::new(glob)).unwrap();

        let mut names:Vec<String> = vec!();
        let mut width = 0;
        let mut height = 0;
        for entry in paths {
            let path = entry.unwrap().path();
            if path.extension().unwrap() == "png" {
                match image_lib::open(&path) {
                    Ok(img) => {
                        let dimensions = img.dimensions();
                        width = width + dimensions.0;
                        height = cmp::max(height, dimensions.1);
                        names.push(path.file_stem().unwrap().to_str().unwrap().to_string())
                    },
                    Err(_) => {

                    }
                }
            }
        }
        SpriteMap {
            width: width,
            height: height,
            names: names,
            url: glob.to_string()
        }
    }

    // Returns the image and background position for use in a single shorthand property.
    // background: spriteMap.sprite("name").
    pub fn sprite(&self, name:&str) -> String {
        name.to_string()
    }
}

#[test]
fn builds() {
    let sprite_map = SpriteMap::build("data/my-icons");
    let mut names = sprite_map.names;
    names.sort();
    assert_eq!(names,vec!("edit1","edit2","edit3"));
    assert_eq!(sprite_map.height, 32 as u32);
    assert_eq!(sprite_map.width, (names.len()*32) as u32);
}
