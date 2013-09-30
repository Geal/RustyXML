#[link(name = "xml", vers = "0.1", author = "Florob")];

#[ crate_type = "lib" ];
#[forbid(non_camel_case_types)];
#[warn(missing_doc)];

pub use base::*;
pub use Parser::Parser;
pub use ElementBuilder::ElementBuilder;
use std::from_str::FromStr;
pub mod base;
pub mod Parser;
pub mod ElementBuilder;

impl FromStr for Element {
    #[inline]
    fn from_str(data: &str) -> Option<Element> {
        let mut p = Parser::Parser::new();
        let mut e = ElementBuilder::ElementBuilder::new();
        let mut result = None;

        do p.parse_str(data) |event| {
            match event {
                Ok(event) => match e.push_event(event) {
                    Ok(Some(elem)) => result = Some(elem),
                    _ => ()
                },
                _ => ()
            }
        }
        result
    }
}