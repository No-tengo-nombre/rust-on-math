# Rust on Math
*Rust on Math* (shortened to *rom*) is a package for Rust, which implements
different numerical methods used throughout science and engineering, as well as
important algorithms such as the FFT.

It is inspired by packages like *NumPy* and *SciPy*, which are available for the
Python programming language.

# Implemented packages and features
## core
Within `rom_rs::core` you can find all the basic functionalities, such as the generation of linear spaces, basic mathematical operations and constants, as well as basic types and traits that are used within other subpackages. All of these can be imported directly from `rom_rs` (i.e you can just call `rom_rs::linspace(...)` instead of having to call `rom_rs::core::axis::linspace(...)`, in order to save some time), since they are part of the basic functionality.

## fft
This subpackage contains the code to calculate the Fourier transform of some data. For this, there is a basic DFT algorithm which only really exists for testing purposes since it is really slow; instead, you should use the FFT algorithm, which is currently implemented through the use of another crate called *RustFFT* [1].

## linalg
Here are different numerical methods useful for linear algebra, such as inverting a matrix, calculating an LU decomposition, etc.

## num_th
This package contains algorithms and numerical methods that are useful within a number theory context, as well as some basic operations that might be useful elsewhere, such as the gcd (greatest common divisor) of a series of numbers, lcm (lowest common multiple), as well as some methods useful for criptography.

## stats
In here, you can find numerical methods for statistics, as well as some operations that come from this context (such as calculating the mean or median).

# License
This package is distributed under the MIT license.

# References
[1] [RustFFT Crate](https://docs.rs/rustfft/latest/rustfft/) and [Github repo](https://github.com/ejmahler/RustFFT).
