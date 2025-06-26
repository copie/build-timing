use crate::build::{ConstType, ConstVal};
use lazy_static::lazy_static;
use std::{collections::BTreeMap, env as std_env};

lazy_static! {
    pub(crate) static ref STD_ENV_MAP: BTreeMap<String, String> = {
        let mut env_map = BTreeMap::new();
        for (k, v) in std_env::vars() {
            env_map.insert(k, v);
        }
        env_map
    };
}

/// `build_timing` build constant identifiers.
pub type BuildTimingConst = &'static str;

const BUILD_OS_DOC: &str = r#"
Operating system and architecture on which the project was build.
The format of this variable is always `os-arch`,
where `os` is the operating system name as returned by [`std::env::consts::OS`],
and `arch` is the computer architecture as returned by [`std::env::consts::ARCH`]."#;
pub const BUILD_OS: BuildTimingConst = "BUILD_OS";

pub trait BuildConstVal {
    fn build_val(&self) -> ConstVal;
}

impl BuildConstVal for BuildTimingConst {
    fn build_val(&self) -> ConstVal {
        match self {
            &BUILD_OS => ConstVal {
                desc: BUILD_OS_DOC.to_string(),
                v: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
                t: ConstType::Str,
            },
            _ => panic!("Unknown build constant: {self}"),
        }
    }
}
