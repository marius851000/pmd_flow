use crate::tool::{read_reference_u32, read_string_utf8, read_u16_le, read_u32_le};
use pmd_sir0::write_sir0_footer;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::num::TryFromIntError;
//TODO: ensure comment are also copied
//TODO: remove all panic!

//TODO: Error
#[derive(Debug)]
pub enum FlowDataError {
    IOError(io::Error),
    TryFromIntError(TryFromIntError),
    IDNameNotString(FlowDataValue),
    StringReferenceTooBig(u16, u32), // u16: the position, u32: the length
    KeyValTooBig(u16, u32),
    ValTooBig(u16, u32),
    DicReferenceTooBig(u16, u32),
    VecReferenceTooBig(u16, u32),
    UnrecognizedTypeForDic(u16),
    UnrecognizedTypeForVec(u16),
}

impl From<io::Error> for FlowDataError {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<TryFromIntError> for FlowDataError {
    fn from(err: TryFromIntError) -> Self {
        Self::TryFromIntError(err)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum FlowDataValue {
    String(String),
    RefDic(u16),
    RefVec(u16),
}

impl FlowDataValue {
    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::String(str) => Some(str.clone()),
            _ => None,
        }
    }

    pub fn get_vecid(&self) -> Option<usize> {
        match self {
            Self::RefVec(vecid) => Some(*vecid as usize),
            _ => None,
        }
    }

    pub fn get_dicid(&self) -> Option<usize> {
        match self {
            Self::RefDic(dicid) => Some(*dicid as usize),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct FlowData {
    // imperative
    dictionaries: Vec<HashMap<String, FlowDataValue>>,
    vectors: Vec<Vec<FlowDataValue>>,
    // cache
    idnames: HashMap<String, FlowDataValue>,
    backlink_dictionaries: HashMap<u16, FlowDataValue>,
    backlink_vector: HashMap<u16, FlowDataValue>,
    // file related
    pub unknown1: u32,
    pub unknown2: u16,
}

impl FlowData {
    pub fn push_dictionary(
        &mut self,
        values: HashMap<String, FlowDataValue>,
    ) -> Result<u16, FlowDataError> {
        let actual_dictionary_id: u16 = self.dictionaries.len().try_into()?;
        let reference_self = FlowDataValue::RefDic(actual_dictionary_id);
        if let Some(value) = values.get("idname") {
            match value {
                FlowDataValue::String(str) => {
                    self.idnames.insert(str.clone(), reference_self.clone());
                }
                err_type => return Err(FlowDataError::IDNameNotString(err_type.clone())),
            }
        };
        for (_, value) in values.iter() {
            match value {
                FlowDataValue::RefDic(dicid) => {
                    self.backlink_dictionaries
                        .insert(*dicid, reference_self.clone());
                }
                FlowDataValue::RefVec(vecid) => {
                    self.backlink_vector.insert(*vecid, reference_self.clone());
                }
                _ => (),
            };
        }
        self.dictionaries.push(values);
        Ok((self.dictionaries.len() - 1) as u16)
    }

    pub fn push_vector(&mut self, values: Vec<FlowDataValue>) -> Result<u16, FlowDataError> {
        let actual_dictionary_id: u16 = self.dictionaries.len().try_into()?;
        let reference_self = FlowDataValue::RefDic(actual_dictionary_id);

        for key in &values {
            if let FlowDataValue::RefDic(dicid) = key {
                self.backlink_dictionaries
                    .insert(*dicid, reference_self.clone());
            };
        }

        self.vectors.push(values);

        Ok((self.vectors.len() - 1) as u16)
    }

    pub fn dictionary_len(&self) -> usize {
        self.dictionaries.len()
    }

    pub fn vector_len(&self) -> usize {
        self.vectors.len()
    }

    pub fn get_dictionary(&self, dicid: usize) -> Option<&HashMap<String, FlowDataValue>> {
        if dicid >= self.dictionaries.len() {
            None
        } else {
            Some(&self.dictionaries[dicid])
        }
    }

    pub fn get_dictionary_mut(
        &mut self,
        dicid: usize,
    ) -> Option<&mut HashMap<String, FlowDataValue>> {
        if dicid >= self.dictionaries.len() {
            None
        } else {
            Some(&mut self.dictionaries[dicid])
        }
    }

    pub fn get_vector(&self, vecid: usize) -> Option<&Vec<FlowDataValue>> {
        if vecid >= self.vectors.len() {
            None
        } else {
            Some(&self.vectors[vecid])
        }
    }

    pub fn get_vector_mut(&mut self, vecid: usize) -> Option<&mut Vec<FlowDataValue>> {
        if vecid >= self.vectors.len() {
            None
        } else {
            Some(&mut self.vectors[vecid])
        }
    }
}

//TODO: review every variable name with something clearer

impl FlowData {
    pub fn new<T: Read + Seek>(file: &mut T) -> Result<FlowData, FlowDataError> {
        let mut flowdata = FlowData::default();

        //TODO: magic
        file.seek(SeekFrom::Start(4))?;
        let content_data_ptr = read_u32_le(&mut *file)?;
        let _pointer_offsets_ptr = read_u32_le(&mut *file)?;

        file.seek(SeekFrom::Start(content_data_ptr.into()))?;
        flowdata.unknown1 = read_u32_le(&mut *file)?;
        let info_ptr = read_u32_le(&mut *file)?;

        let dic_count = read_u32_le(&mut *file)?;
        let dic_section_ptr = read_u32_le(&mut *file)?;

        let vec_count = read_u32_le(&mut *file)?;
        let vec_section_ptr = read_u32_le(&mut *file)?;

        let strptr_section_ptr = read_u32_le(&mut *file)?;

        let val_section_ptr = read_u32_le(&mut *file)?;
        let keyval_section_ptr = read_u32_le(&mut *file)?;

        file.seek(SeekFrom::Start(info_ptr.into()))?;
        flowdata.unknown2 = read_u16_le(&mut *file)?;
        let _vector_number = read_u16_le(&mut *file)?;
        //TODO: seem to be the first dict and the first list
        let _first_dic = read_u32_le(&mut *file)?;
        //NOTE: THEY ARE NOT GUARANTED TO HAVE THIS SIZE
        let _first_vec = read_u32_le(&mut *file)?;
        let val_section_len = (keyval_section_ptr - val_section_ptr) / 4;
        let keyval_section_len = (info_ptr - keyval_section_ptr) / 4;

        let strptr_section_len: u32 = 3950; //TODO: compute

        let mut latest_dict_ptr = 0;
        let mut found_dict_ptr = HashSet::new();
        // list dictionaries
        for dic_id in 0..(dic_count as u16) {
            let dic_record_ptr = dic_section_ptr + 8 * dic_id as u32;
            file.seek(SeekFrom::Start(dic_record_ptr.into()))?;
            let dic_size = read_u32_le(&mut *file)?;
            let dic_ptr = read_u32_le(&mut *file)?;

            if latest_dict_ptr > dic_ptr {
                panic!("not well ordered");
            };
            latest_dict_ptr = dic_ptr;
            found_dict_ptr.insert(dic_ptr);

            let mut dic = HashMap::new();
            for counter_2 in 0..dic_size {
                //TODO: name counter_2
                file.seek(SeekFrom::Start((dic_ptr + 2 * counter_2).into()))?;
                let keyval_id = read_u16_le(&mut *file)?;
                if keyval_id as u32 >= keyval_section_len {
                    return Err(FlowDataError::KeyValTooBig(keyval_id, keyval_section_len));
                };

                let keyval_record_ptr = keyval_section_ptr + 4 * keyval_id as u32;
                file.seek(SeekFrom::Start(keyval_record_ptr as u64))?;
                let key_id = read_u16_le(&mut *file)?;
                let val_id = read_u16_le(&mut *file)?;
                if (key_id as u32) >= strptr_section_len {
                    return Err(FlowDataError::StringReferenceTooBig(
                        key_id,
                        strptr_section_len,
                    ));
                }
                if (val_id as u32) >= val_section_len {
                    return Err(FlowDataError::ValTooBig(val_id, val_section_len));
                }

                let strptr_record_ptr = strptr_section_ptr + (4 * key_id) as u32;
                file.seek(SeekFrom::Start(strptr_record_ptr as u64))?;
                let key = read_reference_u32(&mut *file, |f| read_string_utf8(f))?;

                let val_record_ptr = val_section_ptr + 4 * val_id as u32;
                file.seek(SeekFrom::Start(val_record_ptr as u64))?;
                let val_type = read_u16_le(&mut *file)?;
                let val_data = read_u16_le(&mut *file)?;
                let val: FlowDataValue;
                match val_type {
                    0 => {
                        if val_data as u32 >= strptr_section_len {
                            return Err(FlowDataError::StringReferenceTooBig(
                                val_data,
                                strptr_section_len,
                            ));
                        };
                        //TODO: reused name
                        let strptr_record_ptr = strptr_section_ptr + 4 * val_data as u32;
                        file.seek(SeekFrom::Start(strptr_record_ptr as u64))?;
                        val = FlowDataValue::String(read_reference_u32(&mut *file, |f| {
                            read_string_utf8(f)
                        })?);
                    }
                    1 => {
                        if val_data as u32 >= dic_count {
                            return Err(FlowDataError::DicReferenceTooBig(val_data, dic_count));
                        };
                        val = FlowDataValue::RefDic(val_data);
                    }
                    2 => {
                        if val_data as u32 >= vec_count {
                            return Err(FlowDataError::VecReferenceTooBig(val_data, vec_count));
                        };
                        val = FlowDataValue::RefVec(val_data);
                    }
                    unreconized_type => {
                        return Err(FlowDataError::UnrecognizedTypeForDic(unreconized_type))
                    }
                };
                dic.insert(key.clone(), val.clone());
            }

            flowdata.push_dictionary(dic)?;
        }

        // list vector
        for vec_id in 0..vec_count {
            let vec_record_ptr = vec_section_ptr + 8 * vec_id;
            file.seek(SeekFrom::Start(vec_record_ptr as u64))?;
            let vec_size = read_u32_le(&mut *file)?;
            let vec_ptr = read_u32_le(&mut *file)?;

            let mut vec = Vec::new();
            for counter_2 in 0..vec_size {
                file.seek(SeekFrom::Start(vec_ptr as u64 + 2 * counter_2 as u64))?;
                let keyval_id = read_u16_le(&mut *file)?;
                if keyval_id as u32 >= val_section_len {
                    return Err(FlowDataError::KeyValTooBig(keyval_id, val_section_len));
                };

                let val_record_ptr = val_section_ptr + 4 * keyval_id as u32;
                file.seek(SeekFrom::Start(val_record_ptr as u64))?;
                let val_type = read_u16_le(&mut *file)?;
                let val_data = read_u16_le(&mut *file)?;
                let val;
                match val_type {
                    0 => {
                        if val_data as u32 > strptr_section_len {
                            return Err(FlowDataError::StringReferenceTooBig(
                                val_data,
                                strptr_section_len,
                            ));
                        }
                        let strptr_record_ptr = strptr_section_ptr + 4 * val_data as u32;
                        file.seek(SeekFrom::Start(strptr_record_ptr as u64))?;
                        let s = read_reference_u32(&mut *file, |f| read_string_utf8(f))?;
                        val = FlowDataValue::String(s);
                    }
                    1 => {
                        if val_data as u32 > dic_count {
                            return Err(FlowDataError::ValTooBig(val_data, dic_count));
                        }
                        val = FlowDataValue::RefDic(val_data);
                    }
                    unreconized_type => {
                        return Err(FlowDataError::UnrecognizedTypeForVec(unreconized_type))
                    }
                }
                vec.push(val);
            }

            flowdata.push_vector(vec)?;
        }

        Ok(flowdata)
    }

    #[allow(clippy::map_entry)]
    #[allow(clippy::cognitive_complexity)]
    pub fn write<T: Write + Seek>(&self, mut file: &mut T) -> Result<(), FlowDataError> {
        //if set to true, it will compare some value (mainly the size of the part) to the file script_flow_data_us.bin in the EU version of the game
        const COMPARE: bool = false;

        let mut sir0_pointers = vec![4, 8, 20, 28, 36, 40, 44, 48];
        //write the header. most of the info will be filed at the end
        file.write_all(&[b'S', b'I', b'R', b'0'])?;
        file.write_all(&[0; 48])?;

        let mut unique_data = HashMap::new();
        let mut unique_data_vec = Vec::new();

        let mut unique_entries_dictionary = HashMap::new();
        let mut unique_entries_dictionary_vec = Vec::new();

        let mut strings = HashMap::new();
        //doesn't work
        //let mut strings_vec: Vec<String> = vec!["idname".into(), "$START".into(), "in".into(), "start".into()];
        //does work
        let mut strings_vec: Vec<String> = vec![
            "in".into(),
            "start".into(),
            "idname".into(),
            "$START".into(),
        ];

        for (counter, str) in strings_vec.iter().enumerate() {
            strings.insert(str.clone(), counter);
        }

        for dicid in 0..self.dictionary_len() {
            let dic = self.get_dictionary(dicid).unwrap();
            for (key, data) in dic {
                if !strings.contains_key(key) {
                    strings.insert(key.clone(), strings_vec.len());
                    strings_vec.push(key.clone());
                };
                if let FlowDataValue::String(str) = data {
                    if !strings.contains_key(str) {
                        strings.insert(str.clone(), strings_vec.len());
                        strings_vec.push(str.clone());
                    };
                }
                if !unique_entries_dictionary.contains_key(&(key, data)) {
                    unique_entries_dictionary
                        .insert((key, data), unique_entries_dictionary_vec.len());
                    unique_entries_dictionary_vec.push((key, data));
                };
                if !unique_data.contains_key(&data) {
                    unique_data.insert(data, unique_data_vec.len());
                    unique_data_vec.push(data);
                }
            }
        }
        for vecid in 0..self.vector_len() {
            let vec = self.get_vector(vecid).unwrap();
            for data in vec {
                if !unique_data.contains_key(&data) {
                    unique_data.insert(data, unique_data_vec.len());
                    unique_data_vec.push(data);
                }
                if let FlowDataValue::String(str) = data {
                    if !strings.contains_key(str) {
                        strings.insert(str.clone(), strings_vec.len());
                        strings_vec.push(str.clone());
                    };
                }
            }
        }

        // dictionary metadata
        let dictionary_meta_offset = file.seek(SeekFrom::Current(0))?;
        if COMPARE {
            assert_eq!(dictionary_meta_offset, 52);
        }
        for _ in 0..self.dictionary_len() {
            file.write_all(&[0; 4])?;
            sir0_pointers.push(file.seek(SeekFrom::Current(0))? as u32);
            file.write_all(&[0; 4])?;
        }

        // vector metadata
        let vector_meta_offset = file.seek(SeekFrom::Current(0))?;
        if COMPARE {
            assert_eq!(vector_meta_offset, 65_172);
        }
        for _ in 0..self.vector_len() {
            file.write_all(&[0; 4])?;
            sir0_pointers.push(file.seek(SeekFrom::Current(0))? as u32);
            file.write_all(&[0; 4])?;
        }

        // value data (both from dictionary and vector)
        let values_data_offset = file.seek(SeekFrom::Current(0))?;
        if COMPARE {
            assert_eq!(values_data_offset, 79988);
        }

        for data in unique_data_vec {
            match data {
                FlowDataValue::String(str) => {
                    file.write_all(&u16::to_le_bytes(0))?;
                    file.write_all(&u16::to_le_bytes(strings[str].try_into()?))?;
                }
                FlowDataValue::RefDic(refdic) => {
                    file.write_all(&u16::to_le_bytes(1))?;
                    file.write_all(&u16::to_le_bytes(*refdic))?;
                }
                FlowDataValue::RefVec(refvec) => {
                    file.write_all(&u16::to_le_bytes(2))?;
                    file.write_all(&u16::to_le_bytes(*refvec))?;
                }
            }
        }

        // dic entries
        let entries_dictionary_offset = file.seek(SeekFrom::Current(0))?;
        if COMPARE {
            assert_eq!(entries_dictionary_offset, 133_244);
        }
        for (str, data) in unique_entries_dictionary_vec {
            file.write_all(&u16::to_le_bytes(strings[str].try_into()?))?;
            file.write_all(&u16::to_le_bytes(unique_data[data].try_into()?))?;
        }

        //additional information
        let additional_info_offset = file.seek(SeekFrom::Current(0))?;
        if COMPARE {
            assert_eq!(additional_info_offset, 172_544);
        }
        file.write_all(&[0; 4])?;

        // The first dictionary entrie... I have no idea why it's here, and not with the other
        let mut dictionary_metadata = Vec::new();
        let dic = self.get_dictionary(0).unwrap();
        dictionary_metadata.push((file.seek(SeekFrom::Current(0))?, dic.len()));
        for entry in dic {
            file.write_all(&u16::to_le_bytes(
                unique_entries_dictionary[&entry].try_into()?,
            ))?;
        }

        // same, but with vec
        let mut vector_metadata = Vec::new();
        let vec = self.get_vector(0).unwrap();
        vector_metadata.push((file.seek(SeekFrom::Current(0))?, vec.len()));
        for entry in vec {
            file.write_all(&u16::to_le_bytes(unique_data[&entry].try_into()?))?;
        }

        // and an additional 0...
        //TODO: maybe padding
        file.write_all(&[0; 2])?;

        // string reference:
        let strptr_offset = file.seek(SeekFrom::Current(0))?;
        if COMPARE {
            assert_eq!(strptr_offset, 172_556);
        }
        for _ in 0..strings.len() {
            sir0_pointers.push(file.seek(SeekFrom::Current(0))? as u32);
            file.write_all(&[0; 4])?;
        }

        // dictionary entries
        if COMPARE {
            assert_eq!(file.seek(SeekFrom::Current(0))?, 186_060);
        }
        for dicid in 1..self.dictionary_len() {
            let dic = self.get_dictionary(dicid).unwrap();
            dictionary_metadata.push((file.seek(SeekFrom::Current(0))?, dic.len()));
            for entry in dic {
                file.write_all(&u16::to_le_bytes(
                    unique_entries_dictionary[&entry].try_into()?,
                ))?;
            }
        }

        // vector entries
        let vector_list_offset = file.seek(SeekFrom::Current(0))?;
        if COMPARE {
            assert_eq!(vector_list_offset, 233_016);
        }
        for vecid in 1..self.vector_len() {
            let vec = self.get_vector(vecid).unwrap();
            vector_metadata.push((file.seek(SeekFrom::Current(0))?, vec.len()));
            for entry in vec {
                file.write_all(&u16::to_le_bytes(unique_data[&entry].try_into()?))?;
            }
        }

        //TODO: strange. investigate (maybe padding)
        file.write_all(&[0; 2])?;

        // strings
        let string_data_offset = file.seek(SeekFrom::Current(0))?;
        if COMPARE {
            assert_eq!(string_data_offset, 243_156);
        }
        let mut string_correspondance: HashMap<String, u32> = HashMap::new();
        for string in &strings_vec {
            string_correspondance
                .insert(string.clone(), file.seek(SeekFrom::Current(0))?.try_into()?);
            file.write_all(string.as_bytes())?;
            file.write_all(&[0])?;
        }

        // pointer data
        let pointer_offset = file.seek(SeekFrom::Current(0))?;

        // write string reference
        file.seek(SeekFrom::Start(strptr_offset))?;
        for str in strings_vec {
            file.write_all(&u32::to_le_bytes(string_correspondance[&str]))?;
        }

        // write dictionary metadata
        file.seek(SeekFrom::Start(dictionary_meta_offset))?;
        for dic_meta in dictionary_metadata {
            file.write_all(&u32::to_le_bytes(dic_meta.1.try_into()?))?;
            file.write_all(&u32::to_le_bytes(dic_meta.0.try_into()?))?;
        }

        // write vector metadata
        file.seek(SeekFrom::Start(vector_meta_offset))?;
        for dic_meta in vector_metadata {
            file.write_all(&u32::to_le_bytes(dic_meta.1.try_into()?))?;
            file.write_all(&u32::to_le_bytes(dic_meta.0.try_into()?))?;
        }

        // write header entries
        file.seek(SeekFrom::Start(4))?;
        file.write_all(&u32::to_le_bytes(16))?;
        file.write_all(&u32::to_le_bytes(pointer_offset.try_into()?))?;
        file.write_all(&[0; 4])?; //TODO: figure out what this is
                              // normal header
        file.write_all(&u32::to_le_bytes(self.unknown1))?;
        file.write_all(&u32::to_le_bytes(additional_info_offset.try_into()?))?;

        // dic data
        file.write_all(&u32::to_le_bytes(self.dictionary_len().try_into()?))?;
        file.write_all(&u32::to_le_bytes(dictionary_meta_offset.try_into()?))?;

        // vec data
        file.write_all(&u32::to_le_bytes(self.vector_len().try_into()?))?;
        file.write_all(&u32::to_le_bytes(vector_meta_offset.try_into()?))?;

        // strptr data
        file.write_all(&u32::to_le_bytes(strptr_offset.try_into()?))?;

        // data data
        file.write_all(&u32::to_le_bytes(values_data_offset.try_into()?))?;

        // keyval data
        file.write_all(&u32::to_le_bytes(entries_dictionary_offset.try_into()?))?;

        // adittional header
        file.seek(SeekFrom::Start(additional_info_offset))?;
        file.write_all(&u16::to_le_bytes(self.unknown2))?;
        // initial vector
        file.write_all(&u16::to_le_bytes((self.vector_len() - 1).try_into()?))?;
        //TODO:
        file.seek(SeekFrom::Current(8))?;
        // string offset
        file.write_all(&u32::to_le_bytes(string_data_offset.try_into()?))?;

        //write pointer data
        file.seek(SeekFrom::Start(pointer_offset))?;
        write_sir0_footer(&mut file, sir0_pointers)?;

        file.write_all(&[0; 14])?;

        Ok(())
    }
}
