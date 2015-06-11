use function::Function;

#[derive(RustcDecodable,RustcEncodable,PartialEq,Eq,Debug)]
pub struct Program {
    name: String,
    functions: Vec<Function>,
}
