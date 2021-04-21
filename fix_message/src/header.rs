
use fix_common::ApplVerID;

pub trait Header {
    fn get_sender_comp_id(&self) -> &str;
    fn get_target_comp_id(&self) -> &str;
    fn get_msg_seq_num(&self) -> u32;
    fn reply_boxed(&mut self, other: Box<&dyn Header>);
}

pub trait HeaderExt: Header {
    fn get_appl_ver_id<const V: u8>(&self) -> ApplVerID<V>;
    fn reply<H: HeaderExt>(&mut self, other: &H);
}
