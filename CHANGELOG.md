# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2019-03-08

This breaking update is pretty invasive because almost all calls now take
parameters by value instead of reference. The best part in this release are the
conversion traits `TryUnwrapAll` and friends.

Tracking ownership of Name objects is better left to a higher abstraction layer.
We can therefore make the types Copy and provide some zero-copy array
conversions.

We provide custom types for `Option<AttributeLocation>` and
`Option<UniformLocation>` so we can store them with zero memory overhead.
Unfortunately that means we have to convert them into options when we use them
which is not very ergonomic. The previous solutions was modify the values so
that the sentinel values become zero, and storing them in a NonZero* type.
Unfortunately that does not allow us to transmute between the super and the
subtype and thus prevent the zero-copy array conversions.

Usually you don't want to bind or unbind dynamically, you know at compile-time
which one you want so then it makes sense for them to be distinct methods.

### Added
 - Conversions between owned arrays and slices of `Option<T>` and `T` where they
   are of the same size.
 - Specialized types for locations wrapped in options: `OptionAttributeLocation`
   and `OptionUniformLocation`. We no longer utilize NonZero* to get the memory
   optimization. You can convert them into an actual option using `into_option`
   where they will actually be used.
 - Specialized unbind functions where it makes sense.

### Changed
 - Many types are now `Copy` and provide `into_*` function instead of `as_*`.
 - Unbinding is no longer done through the bind functions but through a
   specialized function.

### Removed
 - Type parameter on `UniformLocation`. The loss of ergonomics weighed heavier
   than the additional type safety.

## [0.1.2] - 2019-03-01

**WARNING**: The version was never updated in Cargo.toml.

### Added
 - `Gl::buffer_sub_data` along with `Gl::buffer_reserve` which allows you to
   call glBufferData with a non-zero length and a null pointer.
 - Uniform setters `Gl::uniform_1i` and `Gl::uniform_1f`.

### Changed
 - `Gl::active_texture` takes `Into<TextureUnit>`.

### Removed
 - Use of try_from feature since it is stable since rustc 1.34.0.

## [0.1.1] - 2019-02-26

### Added
 - `Capability` along with `Gl::enable` and `Gl::disable`.
 - `PolygonMode` along with `Gl::polygon_mode`.
 - `CullFace` along with `Gl::cull_face`.
 - `DrawElementsType` along with `Gl::draw_elements`.
 - `MajorAxis` along with the `Row` and `Column` symbols for use with uniform
   setters.
 - `Gl::uniform_matrix4f`
 - `Gl::uniform_matrix4fv`
 - Unbinding of buffers and vertex arrays.
 - More formats to `InternalFormat`, `Format` and `ComponentFormat` accoring to
   the OpenGL 4.6 specification.

### Removed
 - Accidentally left in debug message in `Gl::get_attrib_location`.

## [0.1.0] - 2019-02-21

Initial release.

[Unreleased]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/mickvangelderen/gl-typed-rust/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/mickvangelderen/gl-typed-rust/compare/0.1.0...0.1.1
