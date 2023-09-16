# raytracer

Very basic raytracer in Rust, following Peter Shirley's minibook series. SIMD-optimized to improve rendering times

There's a few hardcoded scenarios. To toggle between them, go to src/image_gen/switch.rs and change the number in the match statement.

To run, make sure you have [portable-simd](https://github.com/rust-lang/portable-simd/tree/master) support (when I made this you needed to run using the +nighly flag), then run the project *from the src directory* (path to texture files ended up hardcoded in a bad way).

So in the src folder run `cargo +nightly run`
