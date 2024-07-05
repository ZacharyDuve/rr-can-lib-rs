# ZAN CAN

Version: v0.1.0

## What it is:
    A trying to be simple protocol for different devices on a can bus to communicate with each other. I needed something I could turn into a library for myself for a model railroad and
    other prexisiting libraries that I looked at had complexities. I did steal some ideas though from canOpen.

## Message Types

Address 0x00 is broadcast

Need some way of creating can packets
256 (2^8) nodes max to a bus
Ids are message identifiers 11 bits long
11 - 8 = 3bits for commands
    - EStop: high priority stop the world
        - 0b000_AAAA_AAAA , data: first byte 0b0XXX_XXXX for stop 0b1XXX_XXXX for cleared estop, remaining bits for estop reason code other bytes reserved. 
    - Device error: used to transmit error for given device
        - 0b001_AAAA_AAAA, first two bytes are for error code that device encountered
    - Timestamp broadcast: used to update nodes with universal time
        - 0b010_AAAA_AAAA, all eight bytes are time in millis since epoch
    - Device Sending data: Used by a device to send data in broadcast to network
        - 0b011_AAAA_AAAA, 
            follows full data field organization
    - Request Device Send data: used by one device to request another to send data
        - 0b100_AAAA_AAAA
            follows data identifier
    - Set Device Data: Set specific data for device if possible
        - 0b101_AAAA_AAAA
            follows full data field organization


Data field organization:
    Data identifier:
        - Think of addressing for data on device. Doesn't actually have to line up with addressing though
        - up to first 4 bytes are data identifier though minimum can be sent and missing bytes will be presumed to be 0 value. 
            Say if data id is < 256 then only one byte needs to be sent.
        - First 2 bits of data identifier are data length value + 1 is bytes so 00 = 1 byte, 01 = 2 bytes, 10 = 3 bytes, and 11 = 4 bytes
    Data:
        up to last 4 bytes are data bytes as described by data length in the data identifier

Required supported device data identified data
    - 0x00: Device Manufacturer | ro | u32
    - 0x01: Device Model | ro | u32
    - 0x02: Device Version | ro | u32
    - 0x03: Device Serial number | ro | u32
    - 0x04: Device state | rw | u8 | 0x00 Ready, 0x01 Active, 0x02 Errored

