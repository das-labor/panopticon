use function::Function;

#[derive(RustcDecodable,RustcEncodable)]
pub struct Program {
    name: String,
    functions: Vec<Function>,
}
