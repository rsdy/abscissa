//! Templating engine for generating new Abscissa applications

#![deny(warnings, unsafe_code, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa_generator/0.0.0"
)]

pub mod properties;
pub mod template;

pub use self::properties::Properties;
pub use self::template::AppTemplate;
pub use heck::CamelCase;