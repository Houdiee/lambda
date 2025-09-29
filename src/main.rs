use lalrpop_util::lalrpop_mod;

pub mod ast;
lalrpop_mod!(lambda);

fn main() {
    let program = lambda::ProgramParser::new().parse(
        r#"
        let x = 1;
        let s : Str = 2;
        let b : Bool = false;
        3 + 1;
        "#,
    );
    println!("{:?}", program);
}
