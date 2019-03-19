use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};
use heck::CamelCase;
use heck::ShoutySnakeCase;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let gl_registry = Registry::new(Api::Gl, (4, 6), Profile::Core, Fallbacks::All, []);
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
        let mut constants = File::create(&Path::new(&out_dir).join("constants.rs")).unwrap();

        for e in gl_registry.enums.iter() {
            if let Some(ref alias) = e.alias {
                println!(
                    "cargo:warning=Ignoring {} which is an alias for {}.",
                    alias, &e.ident
                );
            }

            let s = e
                .ident
                .to_camel_case()
                .replace("1d", "1D")
                .replace("2d", "2D")
                .replace("3d", "3D");
            let c = e.ident.to_shouty_snake_case();

            write_symbol(&mut symbols, &s);
            write_constant(&mut constants, &s, &c);
        }

        for ident in [
            "Uncompiled",
            "Compiled",
            "Unlinked",
            "Linked",
            "Row",
            "Column",
        ]
        .iter()
        {
            let s = ident
                .to_camel_case()
                .replace("1d", "1D")
                .replace("2d", "2D")
                .replace("3d", "3D");
            let c = ident.to_shouty_snake_case();

            write_symbol(&mut symbols, &s);
            write_constant(&mut constants, &s, &c);
        }
    }
}

fn write_symbol(w: &mut File, s: &str) {
    write!(
        w,
        r##"
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct {s};
"##,
        s = s,
    )
    .unwrap();
}

fn write_constant(w: &mut File, s: &str, c: &str) {
    write!(
        w,
        r##"pub const {c}: symbols::{s} = symbols::{s};
"##,
        c = c,
        s = s,
    )
    .unwrap();
}
