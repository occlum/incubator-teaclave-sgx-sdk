use std::fmt::format;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use syn::{parse_file, Ident, Item, ItemFn, Meta, MetaList, NestedMeta, Type, parse_str};
use quote::{ToTokens, quote};

fn is_enclave_ecall(attr: &Meta) -> bool {
    if let Meta::List(meta_list) = attr {
        for meta in &meta_list.nested {
            if let NestedMeta::Meta(sub_meta) = meta {
                if let Meta::Path(path_value) = sub_meta {
                    if path_value.is_ident("ecall") {
                        return true;
                    }
                }
            }
        }
    }
    false
}


fn is_enclave_ocall(attr: &Meta) -> bool {
    if let Meta::List(meta_list) = attr {
        for meta in &meta_list.nested {
            if let NestedMeta::Meta(sub_meta) = meta {
                if let Meta::Path(path_value) = sub_meta {
                    if path_value.is_ident("ocall") {
                        return true;
                    }
                }
            }
        }
    }
    false
}



fn do_file(path: &Path, idents: &mut Vec<Ident>, params: &mut Vec<Vec<(Ident, Type)>>) -> Result<(), std::io::Error> {
    let content = fs::read_to_string(path)?;
    let parsed_file = parse_file(&content).expect("Failed to parse Rust file");


    for item in parsed_file.items {
        if let Item::Fn(func) = item {
            for attr in &func.attrs {
                if let Ok(meta) = attr.parse_meta() {
                    if attr.path.is_ident("enclave") && is_enclave_ecall(&meta) {
                        idents.push(func.sig.ident.clone());

                        // 获取函数参数
                        let mut func_params = Vec::new();
                        for input in &func.sig.inputs {
                            if let syn::FnArg::Typed(pat_type) = input {
                                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                                    func_params.push((pat_ident.ident.clone(), *pat_type.ty.clone()));
                                }
                            }
                        }
                        params.push(func_params);
                    }
                }
            }
        }
    }
    Ok(())
}

fn do_ptr(file:&mut File,idents: &mut Vec<Ident>){
    writeln!(file, "#[repr(C)]");
    writeln!(file, "pub struct DynEntryTable {{\n\tpub nr_ocall: usize,\n\tpub entry_table: [[u8; {}]; 71],\n}}", idents.len()+2);
    writeln!(file, "#[no_mangle]");
    writeln!(file, "static g_dyn_entry_table: DynEntryTable = DynEntryTable {{\n\tnr_ocall: 71,\n\tentry_table: [[0; {}]; 71],\n}};", idents.len()+2);
    writeln!(file, "extern \"C\" {{\t");
    writeln!(file, "fn sgx_ecall(eid: u64, index: i32, ocall_table:*const(), ms:&Vec<u8>) -> sgx_status_t;\t");
    writeln!(file, "fn sgx_t_global_init_ecall(pms:&Vec<u8>) ->sgx_status_t;\t");
    writeln!(file, "fn sgx_t_global_exit_ecall(pms:&Vec<u8>) ->sgx_status_t;");
    writeln!(file, "}}");
    writeln!(file, "pub struct EcallTable {{");
    writeln!(file, "\tpub nr_ecall: usize,");
    writeln!(file, "\tpub ecall_table: [(unsafe extern \"C\" fn(&Vec<u8>) -> sgx_status_t,u8,u8); {}],",idents.len()+2);
    writeln!(file, "}}");
    writeln!(file, "#[no_mangle]\nstatic g_ecall_table : EcallTable=EcallTable{{");
    writeln!(file, "\tnr_ecall: {},",idents.len()+2);
    writeln!(file, "\tecall_table : [");
    for func in idents.iter(){
        writeln!(file, "\t\t(sgx_t_{},0,0),", func);
    }
    writeln!(file, "\t\t(sgx_t_global_init_ecall,0,0),");
    writeln!(file, "\t\t(sgx_t_global_exit_ecall,0,0),");
    writeln!(file, "],");
    writeln!(file, "}};");
}


fn pre_init(file : &mut File){
    writeln!(file, "#![cfg_attr(not(target_vendor = \"teaclave\"), no_std)]");
    writeln!(file, "#![cfg_attr(target_vendor = \"teaclave\", feature(rustc_private))]");

    writeln!(file, "#[cfg(not(target_vendor = \"teaclave\"))]");
    writeln!(file, "#[macro_use]");
    writeln!(file, "extern crate sgx_tstd as std;");

    writeln!(file,"pub use std::vec::Vec;");
    writeln!(file,"pub use sgx_types::sgx_status_t;");
    writeln!(file, "extern crate sgx_serialize;\npub use sgx_serialize::{{SerializeHelper, DeSerializeHelper}};");
    writeln!(file, "#[macro_use]\nextern crate sgx_serialize_derive;");
    writeln!(file, "mod ocall;");
}
fn do_ocall_file(path: &Path, idents: &mut Vec<Ident>, params: &mut Vec<Vec<(Ident, Type)>>) ->Result<(), std::io::Error>{
    let content = fs::read_to_string(path)?;
    let parsed_file = parse_file(&content).expect("Failed to parse Rust file");


    for item in parsed_file.items {
        if let Item::Fn(func) = item {
            for attr in &func.attrs {
                if let Ok(meta) = attr.parse_meta() {
                    if attr.path.is_ident("enclave") && is_enclave_ocall(&meta) {
                        idents.push(func.sig.ident.clone());

                        // 获取函数参数
                        let mut func_params = Vec::new();
                        for input in &func.sig.inputs {
                            if let syn::FnArg::Typed(pat_type) = input {
                                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                                    func_params.push((pat_ident.ident.clone(), *pat_type.ty.clone()));
                                }
                            }
                        }
                        params.push(func_params);
                    }
                }
            }
        }
    }
    Ok(())
}
fn do_ocall_proxy(file:&mut File, idents: Vec<Ident>, params: Vec<Vec<(Ident, Type)>>){
    writeln!(file, "use sgx_serialize::{{SerializeHelper, DeSerializeHelper, DeSerializable, Serializable}};");
    writeln!(file, "pub use sgx_types::sgx_status_t;");
    writeln!(file, "use std::vec::Vec;");
    writeln!(file, "extern \"C\" {{");
    writeln!(file, "\tfn sgx_ocall(index:i32, ms:&Vec<u8>) -> sgx_status_t;");
    writeln!(file, "}}");

    let mut id = 0;
    for (func, param) in idents.iter().zip(params.iter()){
        
        write!(file, "pub extern \"C\" fn {}(", func);
        let mut param_name = Vec::new();
        let mut param_type = Vec::new();
        for pa in param{
            param_name.push(pa.0.clone());
            param_type.push(pa.1.clone());
        }
        let struct_name = syn::Ident::new(&format!("ms_{}", func), func.span());
        for idx in 0..param.len(){
            let paramtype = param[idx].1.clone();
            if idx == param.len()-1{
                write!(file, "{}:{}", param[idx].0, quote!(#paramtype));
            }
            else{
                write!(file, "{}:{}, ", param[idx].0, quote!(#paramtype));
            }
        }
        let para = quote!{
            struct #struct_name{
                #(#param_name:#param_type),*
            };
            let ms = #struct_name{
                #(
                    #param_name,
                )*
            };
        };
        writeln!(file, "){{");
        writeln!(file, "#[derive(Serializable)]");
        writeln!(file, "{}", para);
        writeln!(file, "\tlet helper = SerializeHelper::new();\n\tlet data = helper.encode(ms).unwrap();");
        writeln!(file, "\tunsafe{{");
        writeln!(file, "\t\tprintln!(\"start sgx_ocall\");");
        writeln!(file, "\t\tlet re = sgx_ocall({}, &data);", id+66);
        writeln!(file, "\t\tprintln!(\"{{}}\",re.as_str());");
        writeln!(file, "\t}}");
        id+=1;

        writeln!(file, "}}\n");
    }
}
fn do_ocall()-> Result<(), std::io::Error> {
    let new_module_name = "ocall";
    let output_path = Path::new("src/").join(format!("{}.rs", new_module_name));
    let mut file = File::create(output_path)?;

    let mut ocall_idents = Vec::new();
    let mut ocall_params = Vec::new();
    let ecall_path = Path::new("../app/src/");
    if ecall_path.is_dir() {
        for entry in fs::read_dir(ecall_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                // 获取文件名
                if let Some(file_name) = path.file_name() {
                    // 将文件名转换为字符串并打印
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.ends_with(".rs") {
                            // 去除 .rs 后缀
                            let trimmed_name = &file_name_str[0..file_name_str.len() - 3];
                            do_ocall_file(&path, &mut ocall_idents, &mut ocall_params)?;                            
                        }
                    }
                }
            }
        }
    }
    do_ocall_proxy(&mut file, ocall_idents, ocall_params);
    Ok(())
}
fn main() -> Result<(), std::io::Error> {

    let new_module_name = "lib";
    let output_path = Path::new("src/").join(format!("{}.rs", new_module_name));
    let mut file = File::create(output_path)?;
    pre_init(&mut file);
    
    let mut total_idents = Vec::new();
    let mut total_params = Vec::new();
    let dir_path = Path::new("./src");
    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                // 获取文件名
                if let Some(file_name) = path.file_name() {
                    // 将文件名转换为字符串并打印
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.eq("lib.rs")  || file_name_str.eq("ocall.rs"){
                            continue;
                        } 
                        if file_name_str.ends_with(".rs") {
                            // 去除 .rs 后缀
                            let trimmed_name = &file_name_str[0..file_name_str.len() - 3];
                            let mut idents = Vec::new();
                            do_file(&path, &mut idents, &mut total_params)?;

                            if idents.len() == 0 {
                                continue;
                            }else if  idents.len() == 1 {
                                writeln!(file, "mod {};", trimmed_name);
                                writeln!(file, "use {}::sgx_t_{};", trimmed_name, idents[0]);
                                total_idents.push(idents[0].clone());
                                
                            }
                            else {
                                writeln!(file, "mod {};", trimmed_name);
                                for ident in idents.iter() {
                                    writeln!(file, "use {}::sgx_t_{};", trimmed_name, ident);
                                    total_idents.push(ident.clone());
                                }
                            }
                            
                        }
                    }
                }
            }
        }
    }
    do_ocall();
    do_ptr(&mut file, &mut total_idents);
    Ok(())
}
