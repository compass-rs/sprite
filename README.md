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

.edit3 {
  background-url: url('/imagesbuild-sprite-out/sprite.png') -64px 0 no-repeat;
 }

.edit1 {
  background-url: url('/imagesbuild-sprite-out/sprite.png') 0 0 no-repeat;
 }

.edit2 {
  background-url: url('/imagesbuild-sprite-out/sprite.png') -32px 0 no-repeat;
 }
```
