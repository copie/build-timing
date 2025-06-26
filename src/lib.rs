mod build;
mod build_timing;
mod date_time;
mod env;
mod err;

pub const CARGO_CLIPPY_ALLOW_ALL: &str =
    "#[allow(clippy::all, clippy::pedantic, clippy::restriction, clippy::nursery)]";

#[cfg(feature = "build")]
mod pub_export {
    pub use crate::build::{BuildPattern, BuildTimingBuilder, ConstVal, ConstType};
    pub use crate::env::{BuildConstVal, BuildTimingConst};
    pub use crate::date_time::DateTime;
    pub use crate::err::{BtResult, BuildTimingError};
    pub use crate::build_timing::BuildTiming;

    pub trait Format {
        fn human_format(&self) -> String;
    }
}

#[cfg(feature = "build")]
pub use pub_export::*;


#[macro_export]
macro_rules! build_timing {
    ($build_mod:ident) => {
        #[doc = r#"build_timing mod"#]
        pub mod $build_mod {
            include!(concat!(env!("OUT_DIR"), "/build_timing.rs"));
        }
    };
}
