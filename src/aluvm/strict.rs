pub use aluvm::stl::aluvm_stl;
pub use bp::bc::stl::bitcoin_stl;
pub use bp::stl::bp_core_stl;
pub use strict_types::stl::{std_stl, strict_types_stl;
pub use strict_types::typelib::{LibBuilder, TranslateError};
pub use strict_types::TypeLib;

use crate::{Extension, Genesis, SubSchema, TransitionBundle, LIB_NAME_RGB};

/// Strict types id for the library providing data types for RGB consensus.
pub const LIB_ID_RGB: &str = "sultan_banana_henry_DDKh5Jk4DCqxiWZNyHnkCbq68nV8fsfWuA9cPhUAcvgz";

fn _rgb_core_stl() -> Result<TypeLib, TranslateError> {
    LibBuilder::new(libname!(LIB_NAME_RGB))
        .transpile::<SubSchema>()
        .transpile::<Genesis>()
        .transpile::<TransitionBundle>()
        .transpile::<Extension>()
        .compile(bset! {
            td_stl().to_dependency(),
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

