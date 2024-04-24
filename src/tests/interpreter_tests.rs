use crate::{
    interpreter::Interpreter,
    lexer::Lexer,
    parser::Parser,
    semanticanalyser::SemanticAnalyser,
};

fn run(source: &str) -> Vec<String> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = match lexer.run() {
        Ok(tokens) => tokens,
        Err(_) => {
            return vec!["error".to_string()];
        }
    };

    for token in &tokens {
        println!("{token}");
    }
    println!("\n\n\n");

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

    let mut interpreter = Interpreter::new();
    match interpreter.interpret(ast) {
        Ok(output) => return output,
        Err(e) => {
            eprintln!("An interpreter error occured: {e}")
        }
    }

    return vec!["error".to_string()];
}

#[test]
fn test_blocks() {
    assert_eq!(
        run(
            "var a = \"outer\";

            {
              var a = \"inner\";
              print a;
            }
            
            print a;
            "
        ),
        vec![
            "inner".to_string(),
            "outer".to_string(),
        ]
    );

    assert_eq!(
        run(
            "
            {}

            if (true) {}
            if (false) {} else {}

            print \"ok\";"
        ),
        vec![
            "ok".to_string()
        ]
    )
}

#[test]
fn test_bool() {
    assert_eq!(
        run(
            "
            print true == true;
            print true == false;
            print false == true;
            print false == false;
            print true == 1;
            print false == 0;
            print true == \"true\";
            print false == \"false\";
            print false == \"\";
            print true != true;
            print true != false;
            print false != true;
            print false != false;
            print true != 1;
            print false != 0;
            print true != \"true\";
            print false != \"false\";
            print false != \"\";
            "
        ),
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
            "
            print !true;
            print !false;
            print !!true;
            "
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
        run(
            "
            var f;
            {
            var local = \"local\";
            def f_() {
                print local;
            }
            f = f_;
            }
            f();
            "
        ),
        vec!["local".to_string()]
    );

    assert_eq!(
        run(
            "
            def makeCounter() {
                var i = 0;
                def count() {
                  i++;
                  print i;
                }
                return count;
              }
              var counter = makeCounter();
              counter();
              counter();
            "
        ),
        vec!["1".to_string(), "2".to_string()]
    );
}

#[test]
fn test_evaluations() {
    assert_eq!(
        run(
            "
            print (5 - (3 - 1)) + -1;
            "
        ),
        vec!["2".to_string()]
    );

    assert_eq!(
        run(
            "
            var f1;
            var f2;
            var f3;
            for (var i = 1; i < 4; i++) {
            var j = i;
            def f() {
                print i;
                print j;
            }
            if (j == 1) f1 = f;
            else if (j == 2) f2 = f;
            else f3 = f;
            }
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