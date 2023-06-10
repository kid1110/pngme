use std::fmt;
use crc::{Crc, CRC_32_ISO_HDLC};
use crate::chunk_type::ChunkType;
use crate::{Error, Result};

#[derive(Debug, Clone)]
struct Chunk{
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc:u32
}
//x^32+x^26+x^23+x^22+x^16+x^12+x^11+x^10+x^8+x^7+x^5+x^4+x^2+x+1
//1_0000_0100_1100_0001_0001_1101_1011_0111 -> 04C11DB7
impl Chunk {
    fn new(chunk_type:ChunkType,chunk_data:Vec<u8>)->Chunk{
        let length = chunk_data.len() as u32;
        let crce = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let  chain_data = ChunkType::bytes(&chunk_type);
        let combined:Vec<u8> = chain_data.iter().chain(chunk_data.iter()).copied().collect();
        // combined.extend(&[0,0,0,0]);
        let crc_data = combined.as_slice();
        let crc = crce.checksum(crc_data);
        Chunk { length, chunk_type, chunk_data, crc }
    
    }
    fn length(&self)->u32{
        self.length
    }
    fn chunk_type(&self)->&ChunkType{
        &self.chunk_type
    }
    // fn data(&self)->&[u8]{}
    fn crc(&self)->u32{
        self.crc
    }
    fn data_as_string(&self)->Result<String>{
        let s:Result<String> = String::from_utf8(self.chunk_data.clone().into()).map_err(|e| Box::new(e) as Error);
        match s {
            Ok(s)=>Ok(s),
            Err(e)=>Err(e)
        }
    }
    fn data(&self)->&[u8]{
        &self.chunk_data.as_slice()
    }
    #[warn(dead_code)]
    fn as_bytes(&self)->Vec<u8>{
        self.length.to_be_bytes().iter().chain(self.chunk_type.bytes().iter()).chain(self.chunk_data.iter()).chain(self.crc.to_be_bytes().iter()).copied().collect()
    }
}


impl TryFrom<&[u8]> for Chunk{
    type Error = Error;
    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        //get chunk_length
        let length = value[3] as u32;
        //get chunk_data
        let chunk_data = value[8..value.len()-4].to_vec();
        //get crc code from bytes
        let crcw = &value[value.len()-4..];
        let mut array:[u8;4] = [0;4];
        array.copy_from_slice(crcw);
        let crc = u32::from_be_bytes(array);

        //get chunk_type and valid reserve bit
        let chunk_type = &value[4..8];
        array.copy_from_slice(chunk_type);
        let chunk_type = ChunkType::try_from(array).unwrap();
        
        //checksum crc from chun_data
        let crc_check_data = &value[8..value.len()-4];
        let crce = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let chain_data = ChunkType::bytes(&chunk_type);
        let combined:Vec<u8> = chain_data.iter().chain(crc_check_data.iter()).copied().collect();
        let crc_data = combined.as_slice();
        let crc_valid = crce.checksum(crc_data);

        if crc != crc_valid{
            Err(Error::from("Invalid crc code for chunk"))
        }else{
            let res = Chunk{
                length,
                chunk_type,
                chunk_data,
                crc,
            };
            Ok(res)
        }
        
        
    }
}

impl fmt::Display for Chunk{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }
    //1010_1101_1010_1001_0000_1010_0110_0110
    #[test]
    fn test_chunk_length() {
        //[84, 104, 105, 115, 32, 105, 115, 32, 119, 104, 101, 114, 101, 32, 121, 111, 117, 114, 32, 115, 101, 99, 114, 101, 116, 32, 109, 101, 115, 115, 97, 103, 101, 32, 119, 105, 108, 108, 32, 98, 101, 33]
        let chunk = testing_chunk();
        //[84, 104, 105, 115, 32, 105, 115, 32, 119, 104, 101, 114, 101, 32, 121, 111, 117, 114, 32, 115, 101, 99, 114, 101, 116, 32, 109, 101, 115, 115, 97, 103, 101, 32, 119, 105, 108, 108, 32, 98, 101, 33]
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}

