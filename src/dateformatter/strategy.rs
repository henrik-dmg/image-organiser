use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum DateGroupingStrategy {
    Year,
    Month,
    Week,
}
