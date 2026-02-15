pub mod lexer;
pub mod ast;
pub mod parser;
pub mod ir;
pub mod codegen;
pub mod afm;

pub use lexer::Lexer;
pub use parser::Parser;
pub use codegen::Codegen;
pub use ir::Chunk;

pub fn compile(source: &str) -> anyhow::Result<Chunk> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let stmts = parser.parse();
    let codegen = Codegen::new();
    let chunk = codegen.compile(stmts);
    Ok(chunk)
}
