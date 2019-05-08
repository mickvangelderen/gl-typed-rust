# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
[Unreleased]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.5.0...HEAD

## [0.5.0] - 2019-05-08
[0.5.0]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.4.8...0.5.0

### Removed
 - **breaking** `TextureTargetGE2D` and `TextureTargetGE3D`. The benefits are not worth the added complexity. 

## [0.4.8] - 2019-05-08
[0.4.8]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.4.7...0.4.8

### Added

 - `glClipControl`
 - `GL_DEPTH_ATTACHMENT` and `GL_STENCIL_ATTACHMENT` to `FramebufferAttachment`
 
## [0.4.7] - 2019-05-03
[0.4.7]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.4.6...0.4.7

### Added
 - `get_programiv_move`
 - `get_program_info_log_move`

## [0.4.6] - 2019-04-26
[0.4.6]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.4.5...0.4.6

### Added
 - Extention ARB_texture_filter_anisotropic.
 - Sampler objects.

## [0.4.5] - 2019-04-25
[0.4.5]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.4.4...0.4.5

### Changed
 - Fixed build issues, version 0.4.4 should never be used.

## [0.4.4] - 2019-04-25 (broken)
[0.4.4]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.4.3...0.4.4

### Added
 - glClearDepth
 - Renderbuffer functionality
 - glColorMask
 - glDepthMask
 - glDepthFunc
 - glDepthRange
 - glStencilMask
 
### Changed
 - Removed unused type parameter from `delete_buffers`.
 - Made locations' new_unchecked const functions.

## [0.4.3] - 2019-04-10
[0.4.3]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.4.2...0.4.3

### Added

 - Uniform buffer object functionality.

## [0.4.2] - 2019-04-10
[0.4.2]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.4.1...0.4.2

### Added

 - `unuse_program`
 - `get_context_flags`
 - `draw_buffers`

## [0.4.1] - 2019-04-05
[0.4.1]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.4.0...0.4.1

### Added
 - `get_error`
 - Multiple uniform setters `uniform_{1|2|3|4}{i|ui|f}`.

## [0.4.0] - 2019-04-03
[0.4.0]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.3.0...0.4.0

### Added
 - `is_some` and `is_none` to `Option{Attribute|Uniform}Location`.
 - `From<Option{X}Location> for `Option<{X}Location>` where `X = Attribute|Uniform`.
 - 'OptionAttributeLocation::NONE' and `OptionUniformLocation::NONE`.

### Changed
 - `Gl::get_attrib_location` now returns `OptionAttributeLocation` instead of
   `Option<AttributeLocation>`.
 - Copied `std::convert::Try{From|Into}` into this crate until they're stable. I
   thought they were but it turns out I cant actually use them with the current
   stable rustc.

### Removed
 - Warnings about aliases being ignored in build.

## [0.3.0] - 2019-03-12
[0.3.0]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.2.0...0.3.0

The conversion traits were moved into a stand-alone crate `convute`.

### Changed
 - The wrap and unwrap traits have been replaced with the more solid traits from
   `convute`. The traits are re-exported in the `convert` and `marker` modules.

## [0.2.0] - 2019-03-08
[0.2.0]: https://github.com/mickvangelderen/gl-typed-rust/compare/v0.1.2...0.2.0

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
[0.1.2]: https://github.com/mickvangelderen/gl-typed-rust/compare/0.1.1...0.1.2

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
[0.1.1]: https://github.com/mickvangelderen/gl-typed-rust/compare/0.1.0...0.1.1

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

## 0.1.0 - 2019-02-21

Initial release.

