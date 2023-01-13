# raytracer
Wanted to teach myself Rust and at the same time I wanted to learn CG basics

This is an implementation of a basic raytracer in Rust, following Peter Shirley's minibook series. 

Update: I found out SIMD exists. I haven't timed it but it should speed up rendering by 2x-2.5x since most operations can be vectorized (vector arithmetic, dot and cross products and scalar multiplications) 
