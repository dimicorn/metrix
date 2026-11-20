pub mod metrics {
    pub mod f1_score;
    pub mod parse;
    pub mod precision;
    pub mod recall;
    pub(crate) mod utils;
}

pub use metrics::f1_score::f1_score;
pub use metrics::parse::parse_data;
pub use metrics::precision::precision;
pub use metrics::recall::recall;
pub(crate) use metrics::utils::confusion_matrix;