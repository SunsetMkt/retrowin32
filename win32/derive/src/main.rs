//! Code generator for winapi functions.
//! Generates functions that pop arguments off the x86 stack.

mod gen;
mod parse;

use proc_macro2::TokenStream;
use quote::quote;
use std::{io::Write, path::Path};

fn generate_dll(
    out_dir: &Path,
    module_name: &str,
    dllexports: &parse::DllExports,
) -> anyhow::Result<()> {
    // Note: it is critical that the DLL's exported fns match up to the shims array
    // generated in the shims module.  This means we must first export all the fns
    // found in dllexports.fns, before exporting any other fns (like the .is_none entries
    // in the vtable).

    let mut f = std::fs::File::create(out_dir.join(format!("{}.s", module_name)))?;
    writeln!(f, "# generated by win32/derive")?;
    writeln!(f, ".intel_syntax noprefix")?;
    for dllexport in &dllexports.fns {
        writeln!(f, ".globl _{}", dllexport.sym_name)?;
        writeln!(f, "_{}:", dllexport.sym_name)?;
        writeln!(f, "  call [__imp__retrowin32_syscall]")?;
        let stack = dllexport.stack_consumed();
        if stack > 0 {
            writeln!(f, "  ret {}", stack)?;
        } else {
            writeln!(f, "  ret")?;
        }
    }

    for vtable in &dllexports.vtables {
        for (name, imp) in &vtable.fns {
            if imp.is_none() {
                writeln!(f, ".globl _{}_{}", vtable.name, name)?;
                writeln!(f, "_{}_{}:", vtable.name, name)?;
                writeln!(f, "  int3")?;
            }
        }
        writeln!(f, ".globl _{}", vtable.name)?;
        writeln!(f, "_{}:", vtable.name)?;
        for (name, imp) in &vtable.fns {
            let name = match imp {
                Some(imp) => imp.clone(),
                None => format!("{}_{}", vtable.name, name),
            };
            writeln!(f, "  .long _{}", name)?;
        }
    }

    let mut f = std::fs::File::create(out_dir.join(format!("{}.def", module_name)))?;
    writeln!(f, "; generated by win32/derive")?;
    writeln!(f, "LIBRARY {}", module_name)?;
    writeln!(f, "EXPORTS")?;
    for dllexport in &dllexports.fns {
        let ordinal = dllexport.meta.ordinal.unwrap();
        writeln!(f, "  {} @{}", dllexport.sym_name, ordinal)?;
    }
    for vtable in &dllexports.vtables {
        writeln!(f, "  {} DATA", vtable.name)?;
    }

    Ok(())
}

/// Generate one module's shim functions.
fn generate_shims_module(module_name: &str, dllexports: parse::DllExports) -> TokenStream {
    let module = quote::format_ident!("{}", module_name);
    let dll_name = format!("{}.dll", module_name);

    let mut impls = Vec::new();
    let mut shims = Vec::new();
    let mut shims_list = Vec::new();
    for dllexport in &dllexports.fns {
        let (wrapper, shim) = gen::fn_wrapper(quote! { winapi::#module }, dllexport);
        impls.push(wrapper);
        shims.push(shim);

        let sym_name = &dllexport.sym_name;
        shims_list.push(quote!(shims::#sym_name));
    }

    let shims_count = shims_list.len();
    let raw_dll_path = format!("../../dll/{}", dll_name);
    quote! {
        pub mod #module {
            use super::*;

            mod impls {
                use memory::Extensions;
                use crate::{machine::Machine, winapi::{self, stack_args::*, types::*}};
                use winapi::#module::*;
                #(#impls)*
            }

            mod shims {
                use super::impls;
                use super::Shim;
                #(#shims)*
            }

            const SHIMS: [Shim; #shims_count] = [
                #(#shims_list),*
            ];

            pub const DLL: BuiltinDLL = BuiltinDLL {
                file_name: #dll_name,
                shims: &SHIMS,
                raw: std::include_bytes!(#raw_dll_path),
            };
        }
    }
}

/// Parse a single .rs file or a directory's collection of files.
fn parse_files(path: &Path) -> anyhow::Result<Vec<syn::File>> {
    // path may be a .rs file or a directory (module).
    let mut paths: Vec<std::path::PathBuf> = if path.extension().is_none() {
        std::fs::read_dir(path)?
            .map(|e| e.unwrap().path())
            .collect()
    } else {
        vec![path.to_path_buf()]
    };
    paths.sort();

    let mut files = Vec::new();
    for path in paths {
        let buf = std::fs::read_to_string(&path)?;
        let file = syn::parse_file(&buf)?;
        files.push(file);
    }
    Ok(files)
}

fn generate_builtins_module(mods: Vec<TokenStream>) -> anyhow::Result<Vec<u8>> {
    let out = quote! {
        /// Generated code, do not edit.

        use crate::shims::Shim;

        pub struct BuiltinDLL {
            pub file_name: &'static str,
            /// The xth function in the DLL represents a call to shims[x].
            pub shims: &'static [Shim],
            /// Raw bytes of generated .dll.
            pub raw: &'static [u8],
        }

        #(#mods)*
    };

    // Verify output parses correctly.
    if let Err(err) = syn::parse2::<syn::File>(out.clone()) {
        anyhow::bail!("parsing macro-generated code: {}", err);
    };

    let mut buf = Vec::new();
    // parse2 seems to fail if it sees these, so dump them here.
    write!(&mut buf, "#![allow(non_snake_case)]\n").unwrap();
    write!(&mut buf, "#![allow(non_upper_case_globals)]\n").unwrap();
    write!(&mut buf, "#![allow(unused_imports)]\n").unwrap();
    write!(&mut buf, "#![allow(unused_variables)]\n").unwrap();
    let text = rustfmt(&out.to_string())?;
    buf.extend_from_slice(text.as_bytes());

    Ok(buf)
}

fn rustfmt(tokens: &str) -> anyhow::Result<String> {
    // Stolen from https://github.com/microsoft/windows-rs/blob/master/crates/tools/lib/src/lib.rs
    let mut child = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2018")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    let mut stdin = child.stdin.take().ok_or(anyhow::anyhow!("no stdin"))?;
    stdin.write_all(tokens.as_bytes())?;
    drop(stdin);
    let output = child.wait_with_output()?;

    if !output.status.success() {
        anyhow::bail!("rustfmt failed: {}", std::str::from_utf8(&output.stderr)?);
    }
    Ok(String::from_utf8(output.stdout)?)
}

/// Assign ordinals to all fns that don't have them already.
fn assign_ordinals(fns: &mut [parse::DllExport]) -> anyhow::Result<()> {
    let mut used_ordinals = std::collections::HashSet::new();
    for dllexport in fns.iter_mut() {
        if let Some(ordinal) = dllexport.meta.ordinal {
            if !used_ordinals.insert(ordinal) {
                return Err(syn::Error::new_spanned(dllexport.func, "duplicate ordinal").into());
            }
        }
    }

    let mut ordinal = 1;
    for dllexport in fns {
        if dllexport.meta.ordinal.is_none() {
            while used_ordinals.contains(&ordinal) {
                ordinal += 1;
            }
            dllexport.meta.ordinal = Some(ordinal);
            ordinal += 1;
        }
    }
    Ok(())
}

fn process_builtin_dll(path: &Path, dll_dir: &Path) -> anyhow::Result<TokenStream> {
    let module_name = path.file_stem().unwrap().to_string_lossy();
    eprintln!("{}.dll", module_name);

    let files = parse_files(path)?;
    let mut dllexports = parse::DllExports::default();
    for file in &files {
        parse::gather_dllexports(&file.items, &mut dllexports)?;
    }

    // Sort by name, then assign ordinals satisfying the ordinals that were specified,
    // then sort by ordinal to ensure the output is deterministic.
    dllexports.fns.sort_by(|a, b| a.sym_name.cmp(&b.sym_name));
    assign_ordinals(&mut dllexports.fns).unwrap();
    dllexports.fns.sort_by_key(|e| e.meta.ordinal.unwrap());

    generate_dll(dll_dir, &module_name, &dllexports)?;

    Ok(generate_shims_module(&module_name, dllexports))
}

#[derive(argh::FromArgs)]
/// dllexport generator
struct Args {
    /// output path
    #[argh(option)]
    builtins: String,

    /// dir to write asm files
    #[argh(option)]
    dll_dir: String,

    /// files to process
    #[argh(positional)]
    inputs: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    let mut mods = Vec::new();
    for path in &args.inputs {
        let gen = match process_builtin_dll(path.as_ref(), args.dll_dir.as_ref()) {
            Ok(gen) => gen,
            Err(err) => anyhow::bail!("processing module: {}", err),
        };
        mods.push(gen);
    }

    let builtins_module = generate_builtins_module(mods)?;
    std::fs::write(&args.builtins, &builtins_module)?;

    Ok(())
}
