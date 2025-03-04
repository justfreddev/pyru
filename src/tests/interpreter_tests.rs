use crate::{
    evaluator::Evaluator,
    lexer::Lexer,
    parser::Parser,
    semanticanalyser::SemanticAnalyser,
};

#[allow(unused)]
pub fn run(source: &str) -> Vec<String> {
    let mut lexer = Lexer::new(source.to_string(), 4);
    let tokens = match lexer.run() {
        Ok(tokens) => tokens,
        Err(_) => {
            return vec!["error".to_string()];
        }
    };

    // for token in &tokens {
    //     println!("{token}");
    // }

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("A parser error occured: {e}");
            return vec!["error".to_string()];
        }
    };

    let mut semantic_analyser = SemanticAnalyser::new(ast.clone());
    match semantic_analyser.run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("A semantic error occured: {e}");
        }
    }

    let mut evaluator = Evaluator::new();
    match evaluator.interpret(ast) {
        Ok(output) => return output,
        Err(e) => {
            eprintln!("An evaluator error occured: {e}");
            return vec!["error".to_string()];
        }
    }
}

#[test]
fn test_bool() {
    assert_eq!(
        run(r#"
print(true == true);
print(true == false);
print(false == true);
print(false == false);
print(true == 1);
print(false == 0);
print(true == \"true\");
print(false == \"false\");
print(false == \"\");
print(true != true);
print(true != false);
print(false != true);
print(false != false);
print(true != 1);
print(false != 0);
print(true != \"true\");
print(false != \"false\");
print(false != \"\");
"#),
        vec![
            "true".to_string(),
            "false".to_string(),
            "false".to_string(),
            "true".to_string(),

            "false".to_string(),
            "false".to_string(),
            "false".to_string(),
            "false".to_string(),
            "false".to_string(),

            "false".to_string(),
            "true".to_string(),
            "true".to_string(),
            "false".to_string(),

            "true".to_string(),
            "true".to_string(),
            "true".to_string(),
            "true".to_string(),
            "true".to_string(),
        ]
    );

    assert_eq!(
        run(
            r#"
            print(!true);
            print(!false);
            print(!!true);
            "#
        ),
        vec![
            "false".to_string(),
            "true".to_string(),
            "true".to_string(),
        ]
    );
}

#[test]
fn test_call() {
    assert_eq!(
        run("true();"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("null();"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("123();"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("\"str\"();"),
        vec!["error".to_string()]
    );
}

#[test]
fn test_closures() {
    assert_eq!(
        run(r#"
def makeCounter():
    let i = 0;
    def count():
        i++;
        print(i);
    return count;
let counter = makeCounter();
counter();
counter();
"#),
        vec!["1".to_string(), "2".to_string()]
    );
}

#[test]
fn test_evaluations() {
    assert_eq!(
        run(
            "
            print((5 - (3 - 1)) + -1);
            "
        ),
        vec!["2".to_string()]
    );

    assert_eq!(
        run(
            "
            let f1;
            let f2;
            let f3;
            for i in 1..3:
                let j = i;
                def ():
                    print(i);
                    print(j);
                if j == 1:
                    f1 = f;
                elif j == 2:
                    f2 = f;
                else:
                    f3 = f;
            f1();
            f2();
            f3();
            "
        ),
        vec![
            "4".to_string(),
            "1".to_string(),
            "4".to_string(),
            "2".to_string(),
            "4".to_string(),
            "3".to_string(),
        ]
    )
}