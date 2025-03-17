mod parser;

fn main() {
    let code = r#"
    #include <stdio.h>

    int main() {
        printf("Hello World"!)
        return 0;
    }
    "#;

    parser::parse_code(code, "c");
}
