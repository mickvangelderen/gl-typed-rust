use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let gl_registry = Registry::new(
        Api::Gl,
        (4, 5),
        Profile::Core,
        Fallbacks::All,
        ["GL_ARB_texture_filter_anisotropic"],
    );

    let out_dir = env::var("OUT_DIR").unwrap();

    // Ignore all file changes except build.rs.
    println!("cargo:rerun-if-changed=build.rs");

    {
        let mut bindings = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();
        gl_registry
            .write_bindings(StructGenerator, &mut bindings)
            .unwrap();
    }

    {
        let mut symbols = File::create(&Path::new(&out_dir).join("symbols.rs")).unwrap();

        write!(&mut symbols,
               r##"
pub trait Symbol<T> {{
    const VALUE: T;
}}
"##).unwrap();

        for e in gl_registry.enums.iter() {
            let bits = match e.ty.as_ref() {
                "GLboolean" => 8,
                "GLenum" => 32,
                "GLuint" => 32,
                "GLuint64" => 64,
                _ => panic!("Unexpected type"),
            };

            write_symbol_decl(&mut symbols, &e.ident);

            for &b in &[32, 64] {
                if bits <= b {
                    write_symbol_impl(&mut symbols, &e.ident, b, &e.value);
                }
            }
        }
    }
}

fn write_symbol_decl(w: &mut File, symbol: &str) {
    write!(
        w,
        r##"
#[derive(Debug, Copy, Clone)]
pub struct {symbol};
"##,
        symbol = symbol
    )
    .unwrap()
}

fn write_symbol_impl(w: &mut File, symbol: &str, bits: u32, value: &str) {
    write!(
        w,
        r##"
impl Symbol<u{bits}> for {symbol} {{
    const VALUE: u{bits} = {value} as u{bits};
}}

impl Symbol<i{bits}> for {symbol} {{
    const VALUE: i{bits} = {value} as u{bits} as i{bits};
}}
"##,
        symbol = symbol,
        bits = bits,
        value = value,
    )
    .unwrap();
}
