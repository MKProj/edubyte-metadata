pub const MAIN_URL: &str = "https://edubyte.mkproj.com/";

pub mod statistics {
    use std::ops::Range;

    // Price tiers
    pub const FREE: f64 = 0.0;
    pub const TIER1: Range<f64> = 19.99..35.99;
    pub const TIER2: Range<f64> = 35.99..40.99;
    pub const TIER3: Range<f64> = 40.99..49.99;
    pub const TIER4: Range<f64> = 49.99..69.99;
    pub const TIER5: Range<f64> = 69.99..99.99;
}
