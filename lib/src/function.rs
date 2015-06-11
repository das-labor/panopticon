use basic_block::BasicBlock;

#[derive(RustcDecodable,RustcEncodable,PartialEq,Eq,Debug)]
pub struct Function {
    name: String,
    basic_blocks: Vec<BasicBlock>,
}
