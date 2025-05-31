pub mod code;
pub mod emphasis;
pub mod header;
pub mod link;
pub mod list;

use super::style::StyleWriter;
use std::io;

pub trait Element {
    fn render(&self, writer: &mut StyleWriter) -> io::Result<()>;
}
