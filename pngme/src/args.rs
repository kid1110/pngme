use structopt::StructOpt;
#[derive(StructOpt,Debug)]
#[structopt(name="pgname")]
pub enum PngMeArgs {
    #[structopt(name="encode",about="you can encode a hidden message in png")]
    Encode(EncodeArgs),
    #[structopt(name="decode",about="you can decode a hidden message from png by chunk type")]
    Decode(DecodeArgs),
    #[structopt(name="remove",about="you can remove the hidden message from png by chunk type")]
    Remove(RemoveArgs),
    #[structopt(name="print",about="you can print the hidden messages from png by chunk type")]
    Print(PrintArgs),
}
#[derive(StructOpt,Debug)]
pub struct EncodeArgs {
    // Write me!
    #[structopt(about = "Path to the PNG file")]
    pub png_path:String,
    #[structopt(about = "Chunk type for png")]
    pub chunk_type:String,
    #[structopt(about = "your hidden message data")]
    pub message_data:String
}
#[derive(StructOpt,Debug)]
pub struct DecodeArgs {
    #[structopt(about = "Path to the PNG file")]
    // Write me!
    pub png_path:String,
    #[structopt(about = "Chunk type for png")]
    pub chunk_type:String
}
#[derive(StructOpt,Debug)]
pub struct RemoveArgs {
    #[structopt(about = "Path to the PNG file")]
    // Write me!
    pub png_path:String,
    #[structopt(about = "Chunk type for png")]
    pub chunk_type:String
}
#[derive(StructOpt,Debug)]
pub struct PrintArgs {
    #[structopt(about = "Path to the PNG file")]
    // Write me!
    pub png_path:String
}

