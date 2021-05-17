
use super::header::Header;

pub trait HasHeader {
    type Output: Header;
    fn get_header(&self) -> &Self::Output;
}

pub trait HasHeaderBoxed {
    fn get_header_boxed(&'_ self) -> &'_ dyn Header;
}
