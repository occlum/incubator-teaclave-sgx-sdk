use std::vec::Vec;
#[macro_use]
use sgx_new_edl::enclave;
use sgx_serialize::{SerializeHelper, DeSerializeHelper, DeSerializable, Serializable};
use sgx_types::sgx_status_t;
use crate::ocall::ocall_test;

#[enclave(ecall)]
fn test(a: i32, b: i32) -> i32 {
    println!("Hello, world1!");
            a + b
}
#[enclave(ecall)]
fn test1(a: i32, b: i32) -> i32 {
    println!("Hello, world2!");
            a + b
}
#[enclave(ecall)]
fn test2(a: i32, b:i32) -> i32 {
    ocall_test(a,b);
    println!("Hello, world3!");
        a + b
}
