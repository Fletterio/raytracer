# raytracer
Wanted to teach myself Rust and at the same time I wanted to learn CG basics

This is an implementation of a basic raytracer in Rust, following Peter Shirley's minibook series. 

Update: I found out SIMD exists. I haven't timed it but it should speed up rendering by 2x-2.5x since most operations can be vectorized (vector arithmetic, dot and cross products and scalar multiplications). Since one of my devices can't run rustup (I'm using portable SIMD that runs on nightly which requires rustup, perhaps I'll switch to packed SIMD later) I decided to do this on a simd branch and keep the original SISD version on main.
