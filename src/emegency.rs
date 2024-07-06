

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EmegencyStatus {
    Stop,
    Clear
}

impl From<EmegencyStatus> for u8 {
    fn from(s: EmegencyStatus) -> u8 {
        match s {
            EmegencyStatus::Stop => 0x00,
            EmegencyStatus::Clear => 0x80
        }
    }
}

const INVALID_EMERGENCY_STATUS: &str = "Invalid EmergencyStatus, valid values are 0x00 and 0x80";

impl TryFrom<u8> for EmegencyStatus {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(EmegencyStatus::Stop),
            0x80 => Ok(EmegencyStatus::Clear),
            _ => Err(&INVALID_EMERGENCY_STATUS)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EmergencyReason {
    pub reason: u16
}

const IVALID_REASON_TOO_LARGE: &str = "Reason is too large for 15 bits";

impl TryFrom<u16> for EmergencyReason {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value >= 0x8000u16 {
            Err(IVALID_REASON_TOO_LARGE)
        } else {
            Ok(EmergencyReason { reason: value })
        }
    }
}

impl From<EmergencyReason> for u16 {
    fn from(value: EmergencyReason) -> Self {
        value.reason
    }
}