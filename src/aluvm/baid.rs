use aluvm::stl::alpub(crate) 
pub use bp::bc::{stl::bitcoin_stl};
use aluvm::stl::aluvm_stl;
use bp::bc::stl::bitcoin_stl;
use aluvm::byte;
use aluvm::macro;
use bp::stl::bp_core_stl;
use strict_types::stl::strict_types_stl;
use strict_types::typelib::{LibBuilder, TranslateError};
use strict_types::TypeLib;
use baid58::encode;

use crate::{Extension, Genesis, SubSchema, TransitionBundle, LIB_NAME_RGB};

/// Strict types id for the library providing data types for RGB consensus.
pub const LIB_ID_RGB: &str = "xray_susan_reward_Fvh85VcsTSb2zmdwT13q32Rv9M14nvMaGvCT3JMtRQWf";

fn _rgb_core_stl() -> Result<TypeLib, TranslateError> {
    LibBuilder::new(libname!(LIB_NAME_RGB))
        .transpile::<SubSchema>()
        .transpile::<Genesis>()
        .transpile::<TransitionBundle>()
        .transpile::<Extension>()
        .compile(bset! {
            strict_types_stl().to_dependency(),
            bitcoin_stl().to_dependency(),
            bp_core_stl().to_dependency(),
            aluvm_stl().to_dependency()
        })
}

/// Generates strict type library providing data types for RGB consensus.
pub fn rgb_core_stl() -> TypeLib { _rgb_core_stl().expect("invalid strict type RGB library") }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lib_id() {
        let lib = rgb_core_stl();
        assert_eq!(lib.id().to_string(), LIB_ID_RGB);
    }
}

fn main() {
    let encoded = encode("hello");
    println!("Encoded: {}", encoded);
}

    #[test]
    #[ignore]
    fn attack() {
        use std::sync::{Arc, Mutex};

        let id = Id::new("some information");
        let mut handles = vec![];
        let failures = Arc::new(Mutex::new(vec![]));
        for x in 0..24 {
            let f = failures.clone();
            handles.push(std::thread::spawn(move || {
                let id = id.to_baid58();
                for salt in 0..0x4000000 {
                    let av = Id::new(&format!("attack using salt {x} {salt}")).to_baid58();
                    if id.checksum() == av.checksum() {
                        f.lock()
                            .unwrap()
                            .push(format!("successful bruteforce attack on round {salt:#x}"));
                    }
                }
            }));
        }
        for handle in handles {
            handle.join().ok();
        }
        assert!(failures.lock().unwrap().is_empty(), "Attacks succeeded:\n{failures:#?}");
    }
