pub mod component {
    pub mod add {
        pub fn add(a: u32, b: u32) -> u32 {
            #[allow(unused_unsafe, clippy::all)]
            unsafe {
                #[cfg(target_arch = "wasm32")]
                #[link(wasm_import_module = "component:add/add")]
                extern "C" {
                    #[link_name = "add"]
                    fn wit_import(_: i32, _: i32) -> i32;
                }

                #[cfg(not(target_arch = "wasm32"))]
                fn wit_import(_: i32, _: i32) -> i32 {
                    unreachable!()
                }

                let ret = wit_import(a as i32, b as i32);
                ret as u32
            }
        }
    }

    pub mod sub {
        pub fn sub(a: u32, b: u32) -> u32 {
            #[allow(unused_unsafe, clippy::all)]
            unsafe {
                #[cfg(target_arch = "wasm32")]
                #[link(wasm_import_module = "component:sub/sub")]
                extern "C" {
                    #[link_name = "sub"]
                    fn wit_import(_: i32, _: i32) -> i32;
                }

                #[cfg(not(target_arch = "wasm32"))]
                fn wit_import(_: i32, _: i32) -> i32 {
                    unreachable!()
                }

                let ret = wit_import(a as i32, b as i32);
                ret as u32
            }
        }
    }
}