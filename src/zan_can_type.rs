
use embedded_can::Id;

use super::address::ADDRESS_BIT_LENGTH;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZanCanFrameType {
    //Emergency is a stop the world event. Meant for safety. Should have highest priority
    Emergency,
    Error,
    Time,
    SentData,
    RequestData,
    SetData
}

impl From<ZanCanFrameType> for u8 {
    fn from(t: ZanCanFrameType) -> u8 {
        match t {
            ZanCanFrameType::Emergency => 0x00,
            ZanCanFrameType::Error => 0x02,
            ZanCanFrameType::Time => 0x03,
            ZanCanFrameType::SentData => 0x04,
            ZanCanFrameType::RequestData => 0x05,
            ZanCanFrameType::SetData => 0x06
        }
    }
}

impl From<u8> for ZanCanFrameType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Emergency,
            0x02 => Self::Error,
            0x03 => Self::Time,
            0x04 => Self::SentData,
            0x05 => Self::RequestData,
            0x06 => Self::SetData,
            _ => panic!("Got an invalid value for from u8 to ZanCanFrameType")
        }
    }
}

impl From<u16> for ZanCanFrameType {
    fn from(value: u16) -> Self {
        ZanCanFrameType::from(value as u8)
    }
}

impl From<u32> for ZanCanFrameType {
    fn from(value: u32) -> Self {
        ZanCanFrameType::from(value as u8)
    }
}

impl From<Id> for ZanCanFrameType{
    fn from(id: Id) -> Self {
        match id {
            Id::Standard(sid) => {
                //Strip the address off by bitshifting
                ZanCanFrameType::from(sid.as_raw() >> ADDRESS_BIT_LENGTH)
            },
            Id::Extended(eid) => {
                ZanCanFrameType::from(eid.as_raw() >> ADDRESS_BIT_LENGTH)
            }
        }
    }
}