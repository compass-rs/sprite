#![allow(dead_code)]

pub struct SpriteMap {
    width: u32,
    height: u32,
    names: Vec<String>,
    url: String
}

impl SpriteMap {
    // Returns the image and background position for use in a single shorthand property.
    // background: spriteMap.sprite("name").
    pub fn sprite(name:&str) -> String {
        name.to_string()
    }
}

#[test]
fn it_works() {
    assert_eq!(true,true)
}
