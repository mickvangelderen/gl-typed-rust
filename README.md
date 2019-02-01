If you're looking for a safe OpenGL abstraction, check out
[glium](https://github.com/glium/glium). This project is for people who want
total control over what happens and easily see what OpenGL functions are being
called.

## Goals

1. Implement OpenGL API with more type safety.
2. Everything *must* be zero-cost.
3. Rename OpenGL identifiers to comply with Rust's naming conventions.

Safety is not on the list because I don't trust myself enough to make that
judgement. It may be possible to make the API safe without violating the
zero-cost constraint.

I think it is impossible to prevent all OpenGL errors from occurring (like glium
promises) under the zero-cost constraint.

## Nuances

We don't try to prove that OpenGL is in the right state. Doing so for any
non-trivial program will incur some overhead, violating the zero-cost
constraint. Instead we make it harder to call OpenGL functions with the wrong
types.

The distinction between state and types is not very clear, since state can be
captured in the type. For example we could expose a type `UncompiledShader`

I'm having trouble writing the `compile_shader` definition, as it takes in a
`Shader<K, S>`, for which we do not care about the kind K nor the current
compile status S. On the OpenGL side the status may have changed and we need to
query it in order to know what it is. We could return a `Shader<K, Unknown>` and
add a function that takes that shader, does a glGetShaderiv call to query the
compile status and return a `Shader<K, CompileStatus>`. This keeps everything
type safe and zero-cost but it comes at a price: it adds a function to the API
that doesn't exist in OpenGL.

Another strategy is to simply query the status in `compile_shader`. Doing so
violates the zero-cost constraint because it does more than a regular
`glCompileShader` call. However, when would you ever want to
