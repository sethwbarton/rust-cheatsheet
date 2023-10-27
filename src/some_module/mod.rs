/*
I guess this file has to be named mod.rs? Either that, or you need a file in the root directory
called some_module.rs. Kinda like header files.
 */

// Items in modules can be public or private
pub fn some_function() {
    println!("Hello from a separate module");
}

// You can also define modules inline like this.
pub mod some_sub_module {
    pub fn some_sub_fn() {

    }

    pub mod sub_sub {
        pub struct SomeType {}
        // Module members are private by default
        struct SomePrivateType {}
    }
}