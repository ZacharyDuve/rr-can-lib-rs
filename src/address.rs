
//reperesnetation for zan-can id since it is a subset of can id


pub const ADDRESS_BIT_LENGTH: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZanCanAddress {
    id: u8
}

impl From<u8> for ZanCanAddress {
    fn from(v: u8) -> Self {
        Self { id: v }
    }
}

impl From<ZanCanAddress> for u8 {
    fn from(addr: ZanCanAddress) -> Self {
        addr.id
    }
}
