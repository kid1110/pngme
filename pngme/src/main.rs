mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use args::PngMeArgs;
use clap;
use structopt::StructOpt;
use std::{path::Path, fs::File, io::Write};
use crate::png::Png;
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let pngme_args = PngMeArgs::from_args();
    match pngme_args{
        PngMeArgs::Encode(encode_args)=>{

            let res = Png::from_file(Path::new(&encode_args.png_path),&encode_args.chunk_type,&encode_args.message_data);
            match res{
                Err(e)=>{
                    println!("encode png fail: {}",e)
                },
                Ok(_ok)=>{
                    println!("encode png success")
                }
            }
            println!("{:?}",encode_args);
            
            
        }
        PngMeArgs::Decode(decode_args) => {
            // 处理 decode 子命令的逻辑
            let res = Png::get_chunk(&decode_args.chunk_type, &decode_args.png_path);
            match res{
                Ok(r)=>{
                   println!("decode success: chunk: {:?} message: {:?}",r,r.data_as_string()); 
                }
                Err(e)=>{
                    println!("decode fail: {}",e)
                }
            }
            println!("{:?}", decode_args);
        }
        PngMeArgs::Remove(remove_args) => {
            let res = Png::remove_chunk(&remove_args.chunk_type, &remove_args.png_path);
            match res{
                Ok(r)=>{
                    println!("remove success: chunk:{:?}",r);
                },
                Err(e)=>{
                    println!("remove fail: {}",e)
                }
            }
            println!("{:?}", remove_args);
        }
        PngMeArgs::Print(print_args) => {
            // 处理 print 子命令的逻辑
            let res = Png::get_chunks_from_file(&print_args.png_path);
            match res{
                Ok(r)=>{
                    println!("print success: chunks:{:?}",r)
                },
                Err(e)=>{
                    println!("print fail: {}",e)
                }
            }
            println!("{:?}", print_args);
        }
    }
    Ok(())
}
