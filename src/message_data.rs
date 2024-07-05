
const MAX_IDENTIFIER_LENGTH: usize = 4;
const MAX_DATA_LENGTH: usize = 4;

#[derive(Debug, Clone, Copy)]
pub struct DataIdentifier {
    identifier: u32
}

impl From<DataIdentifier> for u32 {
    fn from(d: DataIdentifier) -> Self {
        d.identifier
    }
}

impl TryFrom<u32> for DataIdentifier {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value >= 0x4000_0000 {
            Err("Unable to convert u32 to DataIdentifier due to data in u32 being larger than or equal to 0x4000_0000")
        } else {
            Ok(DataIdentifier{identifier: value})
        }
    }
}

impl TryFrom<&[u8]> for DataIdentifier {
    type Error = &'static str;

    fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
        if buff.len() == 0 {
            Err("unable to convert &[u8] to DataIdentifier due to slice being size 0")
        } else if buff.len() > MAX_IDENTIFIER_LENGTH {
            Err("unable to convert &[u8] to DataIdentifier due to slize being over maximum size")
        } else {
            let mut d_id = DataIdentifier{identifier: 0};

            let mut i: usize = 0;

            while i < buff.len() {
                let cur_sequence: u8;
                if i == 0 {
                    //First byte has the data length maybe in it so lets purge that
                    cur_sequence = buff[i] & 0x3F;
                } else {
                    cur_sequence = buff[i];
                }
                d_id.identifier = (d_id.identifier << 8) | (cur_sequence as u32);
                i += 1;
            };

            Ok(d_id)
        }
    }
}

impl DataIdentifier {
    pub fn len(&self) -> usize {
        match self.identifier {
            0x00..=0x3F => 1,
            0x40..=0x3FFF => 2,
            0x4000..=0x3FFFFF => 3,
            _ => 4
        }
    }

    pub fn write(&self, buff: &mut [u8]) -> Result<usize, &'static str> {
        let my_len = self.len();
        if buff.len() < my_len {
            Err("Buffer is too small to write DataIdentifier to")
        } else {
            let mut i: usize = 0; 
            while i < my_len {
                let index = my_len - 1 - i;
                buff[index] = (self.identifier >> (8 * i)) as u8;
                i += 1;
            }
            Ok(my_len)
        }
    }
}

pub struct DataMessage{
    message_data_len_bytes: usize,
    data_identifier: DataIdentifier,
    data: [u8; MAX_DATA_LENGTH]
}

impl DataMessage {
    pub fn new(m_d_len: usize, data_id: DataIdentifier, data: [u8; MAX_DATA_LENGTH]) -> Result<DataMessage, &'static str> {
        if m_d_len > MAX_DATA_LENGTH {
            Err("Unable to create DataMessage due to data length being larger than max")
        } else {
            //So I decided not to validate the data just assume that if you put in the wrong length then you will get truncated data
            Ok(DataMessage{message_data_len_bytes: m_d_len, data_identifier: data_id, data})
        }
    }

    pub fn addr_len(&self) -> usize {
        self.data_identifier.len()
    }

    pub fn data_len(&self) -> usize {
        self.message_data_len_bytes
    }

    pub fn len(&self) -> usize {
        self.data_identifier.len() + self.message_data_len_bytes
    }

    pub fn data_identifier(&self) -> DataIdentifier {
        self.data_identifier
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..self.message_data_len_bytes]
    }

    pub fn write(&self, buff: &mut [u8]) -> Result<usize, &'static str> {
        if buff.len() < self.data_identifier.len() + self.message_data_len_bytes {
            Err("buffer too short to write DataMessage to")
        } else {
            self.data_identifier.write(&mut buff[0..self.data_identifier.len()])?;
            buff[0] |= (self.message_data_len_bytes << 6) as u8;
            
            let mut i: usize = 0;
            while i < self.message_data_len_bytes {
                buff[i + self.data_identifier.len()] = self.data[i]; 
                i += 1;
            }
            Ok(self.data_identifier.len() + self.message_data_len_bytes)
        }
    }
}

impl TryFrom<&[u8]> for DataMessage {
    type Error = &'static str;
    fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
        if buff.len() == 0 {
            Err("unable to convert from &[u8] to DataMessage if buff is empty")
        } else if buff.len() > MAX_DATA_LENGTH + MAX_IDENTIFIER_LENGTH {
            Err("unable to convert from &[u8] to DataMessage due to oversized buffer")
        } else {
            let data_len: usize = ((buff[0] & 0xC0) >> 6) as usize;

            if buff.len() == data_len {
                Err("unable to convert from &[u8] to DataMessage due to buffer being less than required")
            } else {
                let id_len: usize = buff.len() - data_len;
                let d_id = DataIdentifier::try_from(&buff[0..id_len])?;
                let mut data = [0u8; 4];
                let mut i: usize = 0;
                while i < data_len {
                    data[i] = buff[id_len + i];
                    i += 1;
                };
                Ok(DataMessage{message_data_len_bytes: data_len, data_identifier: d_id, data})
            }
        }
    }
}