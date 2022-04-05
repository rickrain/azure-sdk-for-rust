#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-preview-2021-05")]
pub mod package_preview_2021_05;
#[cfg(all(feature = "package-preview-2021-05", not(feature = "no-default-tag")))]
pub use package_preview_2021_05::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-03-01")]
pub mod package_2021_03_01;
#[cfg(all(feature = "package-2021-03-01", not(feature = "no-default-tag")))]
pub use package_2021_03_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-06-02")]
pub mod package_2020_06_02;
#[cfg(all(feature = "package-2020-06-02", not(feature = "no-default-tag")))]
pub use package_2020_06_02::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-07-12")]
pub mod package_2018_07_12;
#[cfg(all(feature = "package-2018-07-12", not(feature = "no-default-tag")))]
pub use package_2018_07_12::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2017-12-01")]
pub mod package_2017_12_01;
#[cfg(all(feature = "package-2017-12-01", not(feature = "no-default-tag")))]
pub use package_2017_12_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
