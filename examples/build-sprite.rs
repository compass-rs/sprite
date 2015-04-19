extern crate sprite;
use std::fs::{create_dir};
use std::path::Path;
use sprite::*;


pub fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let input_folder = args.next().expect("Please pass in an input folder");
    let output_path = Path::new("build-sprite-out");
    let _ = create_dir(&output_path);
    let output_file = output_path.join("sprite.png");
    let sprite_map = SpriteMap::build(&*input_folder,&output_file,"/images");
    for (name, region) in sprite_map.regions.iter() {
        let background_url = sprite_map.css_background(&region, 0,0);
        println!(".{} {{\n  background-url: {};\n }}\n", name, background_url );
    }
}
