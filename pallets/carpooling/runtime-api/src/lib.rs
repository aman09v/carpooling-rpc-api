#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]
// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
//use parity_scale_codec;
sp_api::decl_runtime_apis! {
    pub trait CarpoolingApi
    {
        fn get_driver(d_id: u32) -> u32;
    }
}
