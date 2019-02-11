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

## Design

Designing a good API is hard. You have to balance ergonomics, flexibility and
performance. You're trying to guess what would be useful to potential consumers
of your library and abstract over potentially multiple, possibly yet unwritten
implementations.

### Tracking state

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

### Object relations

Do you see the problem with this example?

```rust
// Assume create_context creates an OpenGL context.
let gl1 = create_context();
let gl2 = create_context();
let n: ShaderName = gl1.create_shader(VERTEX_SHADER);
let mut s = Default::default();
gl2.get_shaderiv(&n, COMPILE_STATUS, &mut s);
```

We cannot prove an object name is valid without somehow storing the context it
was created from. This can be done at compile-time by creating a different type
for every context. 

```rust
let gl1 = create_context!(Gl1);
let gl2 = create_context!(Gl2);
let n: ShaderName<Gl1> = gl1.create_shader(VERTEX_SHADER);
let mut s = Default::default();
gl2.get_shaderiv(&n, COMPILE_STATUS, &mut s); // Compile error: Expected &ShaderName<Gl2>, got &ShaderName<Gl1>.
```

This doesn't work when we don't have an upper limit on the number of contexts we
might need. To ensure we dont' use an object name from a different context, we
must store something that uniquely identifies the context (like a reference),
and use it directly or check for equality with the used context.

```rust
struct ShaderName {
    gl: Rc<Gl>,
    name: NonZeroU32,
}
```

Obviously, this comes with a cost. This doesn't really matter much since it is
likely you'll only store a few shader names, if any. However, the same
relationship holds for all object names, like buffers, programs. The relation
also exists between programs and uniform locations.

It seems pretty silly right? What can we do? Well, we can try to limit the
possibility of making such errors by keeping related objects close together.

```rust
struct Program {
    name: ProgramName,
    // ...
    time_loc: UniformLocation<f32>,
    ambient_loc: UniformLocation<f32>,
}
```

With this setup it is still possible to do stupid things.

```rust
// Assume p1 and p2 are programs.
gl.uniform_1f(&p1, &p2.some_loc_not_from_p1, 0.5f);
```

In the end, if you write stupid code, it is going to do stupid things. We just
want to make it hard to do stupid things. Making it impossible to do stupid
things comes at a cost, which can be argued is small, but these small things add
up in a large application. I would rather have users of a library decide how
much protection they want. They should make the trade-off between flexibility,
correctness, speed, maintainability, etc.

### Abstracting over static and dynamic arrays

Should use the `array-generics` and `arrayvec` crates but it is hard to use them
well if you don't know what you are doing. Therefore I wrote [traits and
implementations](https://github.com/mickvangelderen/gl-typed-rust/blob/1e440ec4f643abca5e5ecd14d8af1665871a04f6/src/array.rs)
to abstract over statically and dynamically sized arrays. It works well but
might not be the best solution. As a bonus I took a look at the assembly to see
if it is worth doing at all.

```rust
impl GlTyped {
    // ...

    pub unsafe fn asm_shader_source<'a>(&self, name: &mut ShaderName, sources: &[&'a [u8]; 4]) {
        self.shader_source(name, sources);
    }

    pub unsafe fn asm_shader_source_slice<'a>(&self, name: &mut ShaderName, sources: &[&'a [u8]]) {
        self.shader_source(name, sources);
    }
}
```

Then I installed some utilities to make it easy to view the asm and count the lines of code.

```sh
cargo install cargo-asm
cargo install tokei
```

```sh
cargo asm gl_typed::GlTyped::asm_shader_source > array.s
cargo asm gl_typed::GlTyped::asm_shader_source_slice > slice.s
tokei -f *.s
--------------------------------------------------------------------------------
 Language             Files        Lines         Code     Comments       Blanks
--------------------------------------------------------------------------------
 Assembly                 2          278          278            0            0
--------------------------------------------------------------------------------
 array.s                              30           30            0            0
 slice.s                             248          248            0            0
--------------------------------------------------------------------------------
 Total                    2          278          278            0            0
--------------------------------------------------------------------------------
```

Here is the assembly for the `[&[u8]; 4]` version:

```asm
gl_typed::GlTyped::asm_shader_source:
 push    rbp
 mov     rbp, rsp
 and     rsp, -32
 sub     rsp, 96
 mov     r8, rdi
 mov     edi, dword, ptr, [rsi]
 movsd   xmm0, qword, ptr, [rdx, +, 16]
 movsd   xmm1, qword, ptr, [rdx]
 movlhps xmm1, xmm0
 movsd   xmm0, qword, ptr, [rdx, +, 48]
 movsd   xmm2, qword, ptr, [rdx, +, 32]
 movlhps xmm2, xmm0
 movaps  xmmword, ptr, [rsp, +, 48], xmm2
 movaps  xmmword, ptr, [rsp, +, 32], xmm1
 mov     ecx, dword, ptr, [rdx, +, 8]
 mov     esi, dword, ptr, [rdx, +, 24]
 mov     eax, dword, ptr, [rdx, +, 40]
 mov     edx, dword, ptr, [rdx, +, 56]
 mov     dword, ptr, [rsp, +, 16], ecx
 mov     dword, ptr, [rsp, +, 20], esi
 mov     dword, ptr, [rsp, +, 24], eax
 mov     dword, ptr, [rsp, +, 28], edx
 lea     rdx, [rsp, +, 32]
 lea     rcx, [rsp, +, 16]
 mov     esi, 4
 call    qword, ptr, [r8, +, 7712]
 mov     rsp, rbp
 pop     rbp
 ret
```

Even with all the code to deal with types that might drop, functions that might
panic, and assertions all over the place the code can compile down to a
relatively small number of instructions. I'm not proficient in asm but it looks
like the byte slices are effectively split pointers and lengths. The loop is
unrolled, there seems to be some simd instructions and there are no branches at
all.
