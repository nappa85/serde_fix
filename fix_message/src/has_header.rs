
use super::header::Header;

pub trait HasHeader {
    type Output: Header;
    fn get_header(&self) -> &Self::Output;
}
