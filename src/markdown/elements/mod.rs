pub mod header;
pub mod list;
pub mod code;
pub mod link;

use std::io;
use super::style::StyleWriter;

// Common trait for all markdown elements
pub trait Element {
    fn render(&self, writer: &mut StyleWriter) -> io::Result<()>;
} 