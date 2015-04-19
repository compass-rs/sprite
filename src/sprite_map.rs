#![allow(dead_code)]

use std::fs;
use std::path::{Path,PathBuf};
use image_lib;
use std::cmp;
use image_lib::GenericImage;
use image_lib::imageops::overlay;
use std::collections::{HashMap,hash_map};
use css_formatter::CssFormatter;


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

impl SpriteRegion {

    /// Output the position for this region, using the offsets.
    pub fn css_position(&self, offset_x: i32, offset_y: i32) -> String {
        CssFormatter::position(self.x as i32 + offset_x, self.y as i32 + offset_y)
    }

}

/// The sprite map holding all the images for a given invocation.
pub struct SpriteMap {
    pub width: u32,
    pub height: u32,
    pub regions: HashMap<String, SpriteRegion>,
    pub url: String
}

impl SpriteMap {

    /// Layout all the image matching the given glob.
    /// The code reads the images, gets their dimensions and initialize the regions
    /// with the proper layout information. Right now all the images
    /// are assembled in an horizontal strip.
    /// Returns (width, height, regions)
    fn layout(glob:&str) -> (u32,u32,HashMap<String,SpriteRegion>) {
        let paths = fs::read_dir(&Path::new(glob)).unwrap();

        let mut regions = HashMap::new();
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
                        regions.insert( region.name.clone(), region);
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

    /// Render a previously laid out region.
    /// Returns the path to the generated file.
    fn render(width:u32, height:u32, regions:hash_map::Values<String,SpriteRegion>,output:&Path) -> PathBuf {
        let mut generated = image_lib::ImageBuffer::new(width, height);
        for region in regions {
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
        output.to_path_buf()
    }

    /// Generates a css sprite map from the files matching the glob pattern.
    /// Notes:
    ///   - the glob parameter is a folder for the time being
    ///   - only PNG files can be made into css sprites at this time.
    pub fn build(glob:&str,output:&Path, base_url:&str) -> SpriteMap {
        let layout = SpriteMap::layout(glob);
        let output = SpriteMap::render(layout.0, layout.1, layout.2.values(), output);
        let mut url = base_url.to_string();
        url.push_str(output.to_str().unwrap());
        SpriteMap {
            width: layout.0,
            height: layout.1,
            regions: layout.2,
            url: url
        }
    }


    /// Return the region with the given name (compass API).
    pub fn region(&self,name: &str) -> Option<&SpriteRegion> {
        self.regions.get(name)
    }

    /// Return the names of all the regions (compass API).
    pub fn names(&self) -> Vec<&str> {
        let mut names:Vec<&str> = vec!();
        for k in self.regions.keys() {
           names.push(&*k);
        }
        names
    }

    /// Returns the image and background position for use in a single shorthand property.
    /// For example:
    ///    background: spriteMap.sprite("name")
    /// could generate:
    ///    url('/images/icons.png') 0 -24px no-repeat;
    pub fn css_background(&self, region: &SpriteRegion, offset_x: i32, offset_y: i32) -> String {
        let position = CssFormatter::position( -(region.x as i32) + offset_x,  -(region.y as i32) + offset_y );
        format!("url('{}') {} no-repeat", self.url, position)
    }


}

#[test]
fn region() {
    let region = SpriteRegion {
        x: 10, y: 11,
        width: 32, height: 33,
        file_name: "a/foo.png".to_string(),
        name: "foo".to_string()
    };
    assert_eq!(region.css_position(0,0), "10px 11px");
    assert_eq!(region.css_position(-10,10), "0 21px");

}

fn builds() {
    let sprite_map = SpriteMap::build("data/my-icons", &Path::new("data/out.png"), "/images" );
    let mut names = sprite_map.names();
    names.sort();
    assert_eq!(names,vec!("edit1","edit2","edit3"));
    assert_eq!(sprite_map.height, 32 as u32);
    assert_eq!(sprite_map.width, (names.len()*32) as u32);
}
