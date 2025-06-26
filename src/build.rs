use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use crate::{
    build_timing::DEFINE_BUILD_TIMING_RS,
    env::{BuildConstVal, BuildTimingConst, BUILD_OS},
    err::BtResult, BuildTiming,
};

use crate::date_time::DEFINE_SOURCE_DATE_EPOCH;
use is_debug::is_debug;

#[allow(clippy::all, clippy::pedantic, clippy::restriction, clippy::nursery)]
pub fn default_allow() -> BTreeSet<BuildTimingConst> {
    BTreeSet::from([BUILD_OS])
}

/// A builder pattern structure to construct a `BuildTiming` instance.
///
/// This struct allows for configuring various aspects of how build_timing will be built into your Rust project.
/// It provides methods to set up hooks, specify build patterns, define paths, and allow certain build constants.
///
/// # Fields
///
/// * `hook`: An optional hook that can be used during the build process. Hooks implement the `HookExt` trait.
/// * `build_pattern`: Determines the strategy for triggering package rebuilds (`Lazy`, `RealTime`, or `Custom`).
/// * `allow_const`: A set of build constant identifiers that should be allow in the build.
/// * `src_path`: The source path from which files are read for building.
/// * `out_path`: The output path where generated files will be placed.
///
pub struct BuildTimingBuilder {
    build_pattern: BuildPattern,
    allow_const: BTreeSet<BuildTimingConst>,
    out_path: Option<String>,
    pub(crate) hook_consts: Vec<Box<dyn BuildConstVal>>,
}

impl BuildTimingBuilder {
    /// Creates a new `BuildTimingBuilder` with default settings.
    ///
    /// Initializes the builder with the following defaults:
    /// - `hook`: None
    /// - `build_pattern`: `BuildPattern::Lazy`
    /// - `allow_const`: Uses the result from `default_allow()`
    /// - `src_path`: Attempts to get the manifest directory using `CARGO_MANIFEST_DIR` environment variable.
    /// - `out_path`: Attempts to get the output directory using `OUT_DIR` environment variable.
    ///
    /// # Returns
    ///
    /// A new instance of `BuildTimingBuilder`.
    pub fn builder() -> Self {
        let default_out_path = std::env::var("OUT_DIR").ok();
        Self {
            build_pattern: BuildPattern::default(),
            allow_const: default_allow(),
            out_path: default_out_path,
            hook_consts: Vec::new(),
        }
    }

    /// Gets the output path if it has been set.
    ///
    /// # Returns
    ///
    /// A `BtResult<&String>` containing the output path or an error if the path is missing.
    pub fn get_out_path(&self) -> BtResult<&String> {
        let out_path = self.out_path.as_ref().ok_or("missing `out_path`")?;
        Ok(out_path)
    }

    /// Gets the build pattern.
    ///
    /// # Returns
    ///
    /// A reference to the `BuildPattern` currently configured for this builder.
    pub fn get_build_pattern(&self) -> &BuildPattern {
        &self.build_pattern
    }

    /// Sets the granted constants for this builder.
    ///
    /// # Arguments
    ///
    /// * `allow_const` - A set of `BuildTimingConst` that should be allow from the build.
    ///
    /// # Returns
    ///
    /// A new `BuildTimingBuilder` instance with the specified granted constants.
    pub fn allow_const(mut self, allow_const: BTreeSet<BuildTimingConst>) -> Self {
        self.allow_const = allow_const;
        self
    }

    /// Gets the granted constants.
    ///
    /// # Returns
    ///
    /// A reference to the set of `BuildTimingConst` that are granted for this build.
    pub fn get_allow_const(&self) -> &BTreeSet<BuildTimingConst> {
        &self.allow_const
    }

    /// Builds a `BuildTiming` instance based on the current configuration.
    ///
    /// # Returns
    ///
    /// A `SdResult<BuildTiming>` that represents the outcome of the build operation.
    pub fn build(&mut self) -> BtResult<BuildTiming> {
        BuildTiming::build_inner(self)
    }

    pub fn add_const_hook(mut self, hook: Box<dyn BuildConstVal>) -> Self {
        self.hook_consts.push(hook);
        self
    }
}

/// Serialized values for build constants.
#[derive(Debug, Clone)]
pub struct ConstVal {
    /// User-facing documentation for the build constant.
    pub desc: String,
    /// Serialized value of the build constant.
    pub v: String,
    /// Type of the build constant.
    pub t: ConstType,
}

impl ConstVal {
    pub fn new<S: Into<String>>(desc: S) -> ConstVal {
        // Creates a new `ConstVal` with an empty string as its value and `Str` as its type.
        ConstVal {
            desc: desc.into(),
            v: "".to_string(),
            t: ConstType::Str,
        }
    }

    pub fn new_bool<S: Into<String>>(desc: S) -> ConstVal {
        // Creates a new `ConstVal` with "true" as its value and `Bool` as its type.
        ConstVal {
            desc: desc.into(),
            v: "true".to_string(),
            t: ConstType::Bool,
        }
    }

    pub fn new_slice<S: Into<String>>(desc: S) -> ConstVal {
        // Creates a new `ConstVal` with an empty string as its value and `Slice` as its type.
        ConstVal {
            desc: desc.into(),
            v: "".to_string(),
            t: ConstType::Slice,
        }
    }
}

/// Supported types of build constants.
#[derive(Debug, Clone)]
pub enum ConstType {
    /// [`&str`](`str`).
    Str,
    /// [`bool`].
    Bool,
    /// [`&[u8]`].
    Slice,
    /// [`usize`].
    Usize,
}

impl Display for ConstType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstType::Str => write!(f, "&str"),
            ConstType::Bool => write!(f, "bool"),
            ConstType::Slice => write!(f, "&[u8]"),
            ConstType::Usize => write!(f, "usize"),
        }
    }
}

/// The BuildPattern enum defines strategies for triggering package rebuilding.
///
/// Default mode is `Lazy`.
///
/// * `Lazy`: The lazy mode. In this mode, if the current Rust environment is set to `debug`,
///   the rebuild package will not run every time the build script is triggered.
///   If the environment is set to `release`, it behaves the same as the `RealTime` mode.
/// * `RealTime`: The real-time mode. It will always trigger rebuilding a package upon any change,
///   regardless of whether the Rust environment is set to `debug` or `release`.
/// * `Custom`: The custom build mode, an enhanced version of `RealTime` mode, allowing for user-defined conditions
///   to trigger rebuilding a package.
///
#[derive(Debug, Default, Clone)]
pub enum BuildPattern {
    #[default]
    Lazy,
    RealTime,
    Custom {
        /// A list of paths that, if changed, will trigger a rebuild.
        /// See <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed>
        if_path_changed: Vec<String>,
        /// A list of environment variables that, if changed, will trigger a rebuild.
        /// See <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed>
        if_env_changed: Vec<String>,
    },
}

impl BuildPattern {
    /// Determines when Cargo should rerun the build script based on the configured pattern.
    ///
    /// # Arguments
    ///
    /// * `other_keys` - An iterator over additional keys that should trigger a rebuild if they change.
    /// * `out_dir` - The output directory where generated files are placed.
    pub(crate) fn rerun_if<'a>(
        &self,
        other_keys: impl Iterator<Item = &'a BuildTimingConst>,
        out_dir: &str,
    ) {
        match self {
            BuildPattern::Lazy => {
                if is_debug() {
                    return;
                }
            }
            BuildPattern::RealTime => {}
            BuildPattern::Custom {
                if_path_changed,
                if_env_changed,
            } => {
                if_env_changed
                    .iter()
                    .for_each(|key| println!("cargo:rerun-if-env-changed={key}"));
                if_path_changed
                    .iter()
                    .for_each(|p| println!("cargo:rerun-if-changed={p}"));
            }
        }

        other_keys.for_each(|key| println!("cargo:rerun-if-env-changed={}", key.to_string()));
        println!("cargo:rerun-if-env-changed={DEFINE_SOURCE_DATE_EPOCH}");
        println!("cargo:rerun-if-changed={out_dir}/{DEFINE_BUILD_TIMING_RS}");
    }
}
