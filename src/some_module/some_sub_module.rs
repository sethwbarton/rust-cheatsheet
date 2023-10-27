/*
I guess this file has to be named some_sub_module? Either that, or you need a file in the root directory
called some_module.rs. Kinda like header files.
 */

// You can also define modules inline like this.
pub fn some_sub_fn() {}

pub mod sub_sub {
    pub struct SomeType {}
    // Module members are private by default
    struct SomePrivateType {}
}
