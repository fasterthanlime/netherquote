//! Utilities module! Every crate's gonna have one of them

use serde::{Deserialize, Serialize};

#[derive(
    parse_display::Display,
    parse_display::FromStr,
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    knuffel::DecodeScalar,
)]
pub enum Environment {
    #[display("development")]
    Development,
    #[display("production")]
    Production,
}
