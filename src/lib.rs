#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
pub mod mysql;

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub mod postgres;

#[cfg(feature = "sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
pub mod sqlite;

#[cfg(feature = "migration")]
#[cfg_attr(docsrs, doc(cfg(feature = "migration")))]
pub mod migration;

pub use sea_query;

pub(crate) mod parser;
pub(crate) mod sqlx_types;
pub(crate) mod util;

pub mod name;
pub use name::*;
