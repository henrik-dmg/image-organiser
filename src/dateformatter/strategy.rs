use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum DateGroupingStragegy {
    Year,
    Month,
    Week,
}
