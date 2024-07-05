
/*
This file is meant as an import of other files in library
 */

use embedded_can::{Frame, Id, StandardId};

pub mod address;
pub mod zan_can_type;
mod emegency;
pub mod sent_data;

use zan_can_type::ZanCanFrameType;
use address::ZanCanAddress;
use emegency::{EmegencyStatus, EmergencyReason};

pub struct ZanCanFrame {
    id: Id,
    data_len: usize,
    data: [u8; 8],
    f_type: ZanCanFrameType
}

impl Frame for ZanCanFrame {
    fn new(_: impl Into<Id>, _: &[u8]) -> Option<Self> {
        panic!("new is intentionally not implemented for ZanCanFrame since it doesn't make sense")
    }

    fn new_remote(_: impl Into<Id>, _: usize) -> Option<Self> {
        panic!("new_remote is intentionally not implemented for ZanCanFrame since it doesn't make sense")
    }

    fn is_extended(&self) -> bool {
        false
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        self.id
    }

    fn dlc(&self) -> usize {
        self.data_len
    }

    fn data(&self) -> &[u8] {
        &self.data[..self.data_len]
    }
}


impl ZanCanFrame {

    pub fn from_frame<F: Frame>(f: F) -> Self {
        let f_type = ZanCanFrameType::from(f.id());
        let mut data = [0u8; 8];
        let mut i: usize = 0;

        while i < f.data().len() {
            data[i] = f.data()[i];
            i += 1;
        }
        
        Self { id: f.id(), data_len: f.dlc(), data, f_type }
    }

    pub fn frame_type(&self) -> ZanCanFrameType {
        self.f_type
    }

    pub fn new_emergency(addr: ZanCanAddress, status: EmegencyStatus, reason: EmergencyReason) -> ZanCanFrame {
        let reason_u16 = u16::from(reason);
        let mut data = [0u8; 8];
        //First bit of reason should always be 0 due to checking in creation of reason. Therefor logic or the status into first bit with the reason
        data[0] = u8::from(status) | ( reason_u16 >> 8) as u8;
        //Rest of reason goes into the second byte
        data[1] = reason_u16 as u8;

        ZanCanFrame{id: id_from_type_and_address(ZanCanFrameType::Emergency, addr), data_len: 2, data, f_type: ZanCanFrameType::Emergency}
    }

    pub fn decode_emergency(&self) -> Result<(EmegencyStatus, EmergencyReason), &str> {
        if self.frame_type() != ZanCanFrameType::Emergency {
            Err("Cannot decode emergency frame if not of emergency type")
        } else {
            let status = EmegencyStatus::try_from(self.data[0] & 0x80)?;
            let mut reason_u16: u16 = (self.data[0] & 0x7F) as u16;
            reason_u16 = reason_u16 << 8;
            reason_u16 |= self.data[1] as u16;
            let reason = EmergencyReason::try_from(reason_u16)?;
            Ok((status, reason))
        }
    }
}

fn id_from_type_and_address(t: ZanCanFrameType, addr: ZanCanAddress) -> Id {
    let mut id_u16: u16 = 0x0000;
    id_u16 |= u8::from(t) as u16;
    id_u16 = id_u16 << address::ADDRESS_BIT_LENGTH;
    id_u16 |= u8::from(addr) as u16;
    Id::Standard(StandardId::new(id_u16).expect("something went horribly wrong creating id from type and address"))
}
