use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use crate::{Error, Result};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType{
    data:u32,
}

// impl PartialEq<ChunkType> for ChunkType{
//     fn eq(&self, other: &ChunkType) -> bool {
//         self.data == other.data
//     }
// }

impl ChunkType{
    fn is_valid(&self)->bool{
       let bytes:[u8;4] =  u32::to_le_bytes(self.data);
       for &byte in &bytes{
        if !self.is_reserved_bit_valid(){
            return false;
        }else if !((byte >=65 && byte <=90) || (byte >=97 && byte <=122)){
            return false;
        }
       }
       true
    }
    fn is_digit_valid(&self,bytes:[u8;4])->bool{
        for &byte in &bytes{
            if !((byte >=65 && byte <=90) || (byte >=97 && byte <=122)){
                return false;
            }
        }
        true
    }
    pub fn bytes(&self)->[u8;4]{
        u32::to_be_bytes(self.data)
    }
    

    fn is_critical(&self)->bool{
        let datas = self.bytes();
        let data = datas[0];
        let bit_position = 5;
        let bit_mask = 1 << bit_position;
        data & bit_mask == 0


    }

    fn is_public(&self)->bool{
        let datas = self.bytes();
        let data = datas[1];
        let bit_position = 5;
        let bit_mask = 1 << bit_position;
        data & bit_mask == 0
    }

    fn is_reserved_bit_valid(&self)->bool{
        let datas = self.bytes();
        let data = datas[2];
        let bit_position = 5;
        let bit_mask = 1 << bit_position;
        data & bit_mask == 0
    }

    fn is_safe_to_copy(&self)->bool{
        let datas = self.bytes();
        let data = datas[3];
        let bit_position = 5;
        let bit_mask = 1 << bit_position;
        data & bit_mask !=0
    }
}

impl TryFrom<[u8;4]> for ChunkType{
    type Error = Error;
    fn try_from(value: [u8;4]) -> Result<Self> {
        let res = ChunkType { data: u32::from_be_bytes(value) };
        if res.is_digit_valid(value){
            Ok(res)
        }else{
            Err(Error::from("Invalid ChunkType try from"))
        }
    }
}
impl  FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let bytes =  s.as_bytes().to_vec();
        let mut num_bytes : [u8;4] = [0;4];
        if bytes.len() >=4{
        num_bytes.copy_from_slice(&&bytes[0..4]);
        
        }
        let res_num = u32::from_be_bytes(num_bytes);
        let res = ChunkType { data: res_num };
        if res.is_digit_valid(num_bytes){
            Ok(res)
        }else{
            Err(Error::from("Invalid ChunkType from str"))
        }
    }
}
impl  fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.bytes();
        let res = std::str::from_utf8(&data).unwrap();
        write!(f,"{}",res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());
        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}




