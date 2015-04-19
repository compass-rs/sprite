//! This project provides an implementation of the Compass
//! sprite helpers
//! [reference](http://compass-style.org/reference/compass/helpers/sprites/)
//!
//! For image processing we use the image
//! [code](https://github.com/PistonDevelopers/image) library which
//! is part of Piston.

extern crate image as image_lib;


pub use sprite_map::{SpriteRegion,SpriteMap};

mod sprite_map;
mod css_formatter;
