use lalrpop_util::lalrpop_mod;

pub mod ast;
lalrpop_mod!(lambda);

fn main() {
    let program = lambda::ProgramParser::new().parse(
        r#"
        let x = 2y;
        let s : Str = "Hello world!!";
        let b : Bool = false;
        3 + 1;
        "#,
    );

    for statement in program.unwrap() {
        println!("{:?}", statement);
    }
}
