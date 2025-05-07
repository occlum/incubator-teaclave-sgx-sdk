#![cfg_attr(not(target_vendor = "teaclave"), no_std)]
#![cfg_attr(target_vendor = "teaclave", feature(rustc_private))]
#[cfg(not(target_vendor = "teaclave"))]
#[macro_use]
extern crate sgx_tstd as std;
pub use std::vec::Vec;
pub use sgx_types::sgx_status_t;
extern crate sgx_serialize;
pub use sgx_serialize::{SerializeHelper, DeSerializeHelper};
#[macro_use]
extern crate sgx_serialize_derive;
mod ocall;
mod test;
use test::sgx_t_test;
use test::sgx_t_test1;
use test::sgx_t_test2;
#[repr(C)]
pub struct DynEntryTable {
	pub nr_ocall: usize,
	pub entry_table: [[u8; 5]; 71],
}
#[no_mangle]
static g_dyn_entry_table: DynEntryTable = DynEntryTable {
	nr_ocall: 71,
	entry_table: [[0; 5]; 71],
};
extern "C" {	
fn sgx_ecall(eid: u64, index: i32, ocall_table:*const(), ms:&Vec<u8>) -> sgx_status_t;	
fn sgx_t_global_init_ecall(pms:&Vec<u8>) ->sgx_status_t;	
fn sgx_t_global_exit_ecall(pms:&Vec<u8>) ->sgx_status_t;
}
pub struct EcallTable {
	pub nr_ecall: usize,
	pub ecall_table: [(unsafe extern "C" fn(&Vec<u8>) -> sgx_status_t,u8,u8); 5],
}
#[no_mangle]
static g_ecall_table : EcallTable=EcallTable{
	nr_ecall: 5,
	ecall_table : [
		(sgx_t_test,0,0),
		(sgx_t_test1,0,0),
		(sgx_t_test2,0,0),
		(sgx_t_global_init_ecall,0,0),
		(sgx_t_global_exit_ecall,0,0),
],
};
