
//reperesnetation for zan-can id since it is a subset of can id

use embedded_can::Id;


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

impl TryFrom<Id> for ZanCanAddress {
    type Error = &'static str;
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        match value {
            Id::Extended(_) => Err("unable to convert from Id to ZanCanAddress due to extended ids not being supported"),
            Id::Standard(sid) => {
                Ok(ZanCanAddress { id: sid.as_raw() as u8 })
            }
        }
    }
}