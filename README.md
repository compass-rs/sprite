sprite
=====

Work in progress towards implementing Compass' sprite helpers: http://compass-style.org/reference/compass/helpers/sprites/

[Travis build status:] (https://travis-ci.org/compass-rs/sprite) ![Travis build status]
(https://travis-ci.org/compass-rs/sprite.svg?branch=master)

Documentation: http://compass-rs.github.io/sprite/

This is work in progress. To test that it works run the examples

```
cargo run --example build-sprite data/my-icons/

Running `target/debug/examples/build-sprite data/my-icons/`

Sprite Map [SpriteRegion { x: 0, y: 0, width: 32, height: 32, file_name: "data/my-icons/edit1.png", name: "edit1" }, SpriteRegion { x: 32, y: 0, width: 32, height: 32, file_name: "data/my-icons/edit2.png", name: "edit2" }, SpriteRegion { x: 64, y: 0, width: 32, height: 32, file_name: "data/my-icons/edit3.png", name: "edit3" }]
```
