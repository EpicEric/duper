#[allow(unused_imports)]
use uniffi_runtime_javascript::{self as js, uniffi as u, IntoJs, IntoRust};
use wasm_bindgen::prelude::wasm_bindgen;
extern "C" {
    fn uniffi_duper_uniffi_fn_func_parse(
        input: u::RustBuffer,
        parse_any: i8,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_duper_uniffi_fn_func_serialize(
        input: u::RustBuffer,
        options: u::RustBuffer,
        status_: &mut u::RustCallStatus,
    ) -> u::RustBuffer;
    fn uniffi_duper_uniffi_checksum_func_parse() -> u16;
    fn uniffi_duper_uniffi_checksum_func_serialize() -> u16;
    fn ffi_duper_uniffi_uniffi_contract_version() -> u32;
}
#[wasm_bindgen]
pub fn ubrn_uniffi_duper_uniffi_fn_func_parse(
    input: js::ForeignBytes,
    parse_any: js::Int8,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_duper_uniffi_fn_func_parse(
            u::RustBuffer::into_rust(input),
            i8::into_rust(parse_any),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub fn ubrn_uniffi_duper_uniffi_fn_func_serialize(
    input: js::ForeignBytes,
    options: js::ForeignBytes,
    f_status_: &mut js::RustCallStatus,
) -> js::ForeignBytes {
    let mut u_status_ = u::RustCallStatus::default();
    let value_ = unsafe {
        uniffi_duper_uniffi_fn_func_serialize(
            u::RustBuffer::into_rust(input),
            u::RustBuffer::into_rust(options),
            &mut u_status_,
        )
    };
    f_status_.copy_from(u_status_);
    value_.into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_duper_uniffi_checksum_func_parse() -> js::UInt16 {
    uniffi_duper_uniffi_checksum_func_parse().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_uniffi_duper_uniffi_checksum_func_serialize() -> js::UInt16 {
    uniffi_duper_uniffi_checksum_func_serialize().into_js()
}
#[wasm_bindgen]
pub unsafe fn ubrn_ffi_duper_uniffi_uniffi_contract_version() -> js::UInt32 {
    ffi_duper_uniffi_uniffi_contract_version().into_js()
}
