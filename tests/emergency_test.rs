use zan_can::{address::ZanCanAddress, emegency::{EmegencyStatus, EmergencyReason}, ZanCanFrame};
use embedded_can::Frame;


#[test]
fn creates_emergency_frame() {
    let addr = ZanCanAddress::from(0x44);
    let status = EmegencyStatus::Stop;
    let reason = EmergencyReason::try_from(0x5123).expect("error generating reason");

    let frame = ZanCanFrame::new_emergency(addr, status, reason);

    assert_eq!(frame.dlc(), 2);
}