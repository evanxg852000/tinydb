
// use bincode::{config::Configuration, Decode, Encode};
// use crc::{Crc, CRC_32_ISCSI};
// use parking_lot::Mutex;
// use std::io::{Read, Write};

// const CONFIG: Configuration = bincode::config::standard();

// pub(crate) fn decode<T: Decode>(slice: &[u8]) -> Result<(T, usize), LiteDbError> {
//     let decode_response: (T, usize) = bincode::decode_from_slice(slice, CONFIG)?;
//     Ok(decode_response)
// }

// pub(crate) fn encode_into_writer<T: Encode, W: Write>(
//     value: &T,
//     writer: &mut W,
// ) -> Result<usize, LiteDbError> {
//     let num_encoded_bytes = bincode::encode_into_std_write(value, writer, CONFIG)?;
//     Ok(num_encoded_bytes)
// }

// pub(crate) fn decode_from_reader<T: Decode, R: Read>(reader: &mut R) -> Result<T, LiteDbError> {
//     let decoded_value = bincode::decode_from_std_read(reader, CONFIG)?;
//     Ok(decoded_value)
// }
