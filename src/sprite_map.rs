#![allow(dead_code)]

use std::fs;
use std::path::Path;

pub struct SpriteMap {
    width: u32,
    height: u32,
    names: Vec<String>,
    url: String
}

impl SpriteMap {
    // Generates a css sprite map from the files matching the glob pattern.
    // Only PNG files can be made into css sprites at this time.
    pub fn build(glob:&str) -> SpriteMap {
        let paths = fs::read_dir(&Path::new(glob)).unwrap();

        let mut names:Vec<String> = vec!();
        for entry in paths {
            let path = entry.unwrap().path();
            if path.extension().unwrap() == "png" {
                names.push(path.file_stem().unwrap().to_str().unwrap().to_string())
            }
        }
        SpriteMap {
            width: 0,
            height: 0,
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
    assert_eq!(names,vec!("edit1","edit2","edit3"))
}
