#![doc(html_logo_url = "https://duper.dev.br/logos/duper-100-100.png")]
//!
#![doc = include_str!("../../duper_website/docs/duperq.md")]
//!

mod accessor;
mod filter;
mod formatter;
mod processor;
mod query;
mod types;

pub use query::query;
