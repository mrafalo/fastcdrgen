use serde::Deserialize;
pub const CONFIG_FILE:&str = "config.toml";

#[derive(Clone, Debug, Deserialize, Copy)]
pub enum DistributionType {
    NORMAL,
    POISON,
    WEIBULL,
    UNIFORM,
    CAUCHY
}