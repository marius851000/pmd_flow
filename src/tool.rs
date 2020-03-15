use std::io::Error as IOError;
use std::io::{Read, Seek, SeekFrom, Write};

pub fn read_u32_le<T: Read>(file: &mut T) -> Result<u32, IOError> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

pub fn read_u16_le<T: Read>(file: &mut T) -> Result<u16, IOError> {
    let mut buffer = [0; 2];
    file.read_exact(&mut buffer)?;
    Ok(u16::from_le_bytes(buffer))
}

pub fn read_string_utf8<T: Read>(file: &mut T) -> Result<String, IOError> {
    //TODO: error management
    let mut result = Vec::new();
    let mut buffer = [0];
    loop {
        file.read_exact(&mut buffer)?;
        if buffer[0] == 0 {
            return Ok(String::from_utf8(result).unwrap());
        };
        result.push(buffer[0]);
    }
}

pub fn read_reference_u32<T, O, F>(file: &mut T, parse: F) -> Result<O, IOError>
where
    T: Read + Seek,
    F: Fn(&mut T) -> Result<O, IOError>,
{
    let ressource_address = read_u32_le(file)? as u64;
    let _current_address = file.seek(SeekFrom::Current(0))?;
    file.seek(SeekFrom::Start(ressource_address))?;
    let result = parse(file)?;
    file.seek(SeekFrom::Start(ressource_address))?;
    Ok(result)
}

pub fn write_sir0_footer<T>(file: &mut T, dic: Vec<u32>) -> Result<(), IOError>
where
T: Write
{
    let mut latest_written_pointer = 0;
    for original_to_write in dic {

        let mut remaining_to_write = original_to_write - latest_written_pointer;
        latest_written_pointer = original_to_write;
        let mut reversed_to_write = Vec::new();
        if remaining_to_write == 0 {
            //NOTE: this never happen in original game. This is an extrapolation of what will need to be written in such a situation.
            reversed_to_write.push(0);
        } else {
            loop {
                if remaining_to_write >= 128 {
                    let to_write = (remaining_to_write % 128) as u8;
                    remaining_to_write = remaining_to_write >> 7;
                    reversed_to_write.push(to_write);
                } else {
                    reversed_to_write.push(remaining_to_write as u8);
                    break;
                }
            }
        }
        for (counter, value_to_write) in reversed_to_write.iter().cloned().enumerate().rev() {
            if counter == 0 {
                file.write(&[value_to_write])?;
            } else {
                file.write(&[value_to_write + 0b10000000])?;
            }
        };
    };
    Ok(())
}

pub fn add_padding<T>(file: &mut T, pad_indication_number: u64) -> Result<(), IOError>
where T: Seek + Write {
    let remaining = pad_indication_number - file.seek(SeekFrom::Current(0))? % pad_indication_number;
    if remaining == pad_indication_number {
        return Ok(())
    }
    for _ in 0..remaining {
        file.write(&[0])?;
    };
    Ok(())
}
