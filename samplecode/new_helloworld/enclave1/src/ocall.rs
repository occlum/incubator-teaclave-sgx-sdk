use sgx_serialize::{SerializeHelper, DeSerializeHelper, DeSerializable, Serializable};
pub use sgx_types::sgx_status_t;
use std::vec::Vec;
extern "C" {
	fn sgx_ocall(index:i32, ms:&Vec<u8>) -> sgx_status_t;
}
pub extern "C" fn ocall_test(a:i32, b:i32){
#[derive(Serializable)]
struct ms_ocall_test { a : i32 , b : i32 } ; let ms = ms_ocall_test { a , b , } ;
	let helper = SerializeHelper::new();
	let data = helper.encode(ms).unwrap();
	unsafe{
		println!("start sgx_ocall");
		let re = sgx_ocall(66, &data);
		println!("{}",re.as_str());
	}
}

