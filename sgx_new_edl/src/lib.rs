

use syn::{parse_macro_input, ItemFn,FnArg, PatType, Pat, PatIdent, Ident, Type};
use proc_macro::TokenStream;
use quote::quote;




fn enclave_ecall(input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as ItemFn);

    // 获取函数名和参数
    let name = &item_fn.sig.ident;

    // 获取参数名
    let param_names = item_fn.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(PatType { pat, .. }) = arg {
            // 检查模式是否为 Pat::Ident 类型
            if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                Some(ident.clone())
            } else {
                None
            }
        } else {
            None
        }
    }).collect::<Vec<Ident>>();


    // 获取参数类型
    let param_types = item_fn.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            Some(*ty.clone())
        } else {
            None
        }
    }).collect::<Vec<Type>>();
    
    // 创建一个新的函数名
    let new_name = syn::Ident::new(&format!("sgx_t_{}", name), name.span());
    let param_name = syn::Ident::new(&format!("ms_{}", name), name.span());


    // 使用quote!宏构建输出代码
    let expanded = quote! {
        #item_fn
        // only one serialize arg 
        #[derive(Serializable, DeSerializable)]
        #[no_mangle]
        pub struct #param_name{
            #(#param_names:#param_types),*
        }
        pub extern "C" fn #new_name (mm:&Vec<u8>) -> sgx_status_t
        {
            // println!("sgx_ecall1");
            let helper = DeSerializeHelper::<#param_name>::new(mm.to_vec());
            // println!("sgx_ecall2");
            let ms : #param_name =helper.decode().unwrap();
            #(
                let #param_names = ms.#param_names;
            )*
            #name(#(#param_names),*);
            // println!("status = sgx_ecall(eid, func_id, &ocall_table_EnclaveResponder, &ms);");
            sgx_status_t::SGX_SUCCESS
            // 这里可以添加更多的逻辑

        }
        
    };
    let token_stream = TokenStream::from(expanded);

    token_stream
}

fn enclave_ocall(input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as ItemFn);

    // 获取函数名和参数
    let name = &item_fn.sig.ident;

    // 获取参数名
    let param_names = item_fn.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(PatType { pat, .. }) = arg {
            // 检查模式是否为 Pat::Ident 类型
            if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                Some(ident.clone())
            } else {
                None
            }
        } else {
            None
        }
    }).collect::<Vec<Ident>>();


    // 获取参数类型
    let param_types = item_fn.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            Some(*ty.clone())
        } else {
            None
        }
    }).collect::<Vec<Type>>();
    
    // 创建一个新的函数名
    let new_name = syn::Ident::new(&format!("sgx_t_{}_ocall", name), name.span());
    let param_name = syn::Ident::new(&format!("ms_{}", name), name.span());


    // 使用quote!宏构建输出代码
    let expanded = quote! {
        #item_fn
        // only one serialize arg 
        #[derive(Serializable, DeSerializable)]
        #[no_mangle]
        pub struct #param_name{
            #(#param_names:#param_types),*
        }
        #[no_mangle]
        pub extern "C" fn #new_name (mm:&Vec<u8>) -> sgx_status_t
        {
            let helper = DeSerializeHelper::<#param_name>::new(mm.to_vec());
            // println!("sgx_ecall2");
            let ms : #param_name =helper.decode().unwrap();
            #(
                let #param_names = ms.#param_names;
            )*
            #name(#(#param_names),*);
        
            sgx_status_t::SGX_SUCCESS
        }
    };
    let token_stream = TokenStream::from(expanded);
    token_stream
}


#[proc_macro_attribute]
pub fn enclave(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_str = args.to_string();
    let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();

    // 检查第一个参数是否为 "ecall" 或 "ocall"
    if args.is_empty() {
        panic!("Expected at least one argument for #[enclave]");
    }
    let enclave_type = args[0];
    if enclave_type == "ecall" {
        enclave_ecall(input)
    }else if enclave_type == "ocall" {
        enclave_ocall(input)
    }else {
        panic!("Only support sgx enclave o/ecall");
    }
}

