use std::vec;

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

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("A parser error occured: {e}");
            return vec!["error".to_string()];
        }
    };

    for node in &ast {
        println!("{node}");
    }

    let mut semantic_analyser = SemanticAnalyser::new(ast.clone());
    match semantic_analyser.run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("A semantic error occured: {e}");
            return vec!["error".to_string()];
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
fn test_assignment() {
    // Tests for associativity of assignments
    assert_eq!(
        run(r#"
let a = "a";
let b = "b";
let c = "c";
a = b = c;
print(a);
print(b);
print(c);

"#),
        vec!["c".to_string(), "c".to_string(), "c".to_string()]
    );

    assert_eq!(
        run(r#"
let a = "before";
let c = a = "var";
print(a);
print(c);

"#
        ),
        vec![
            "var".to_string(),
            "var".to_string(),
        ]
    );

    // Tests for invalid assignments
    assert_eq!(
        run(r#"
let a = "a";
(a) = "value";

"#
        ),
        vec!["error".to_string()]
    );

    assert_eq!(
        run(r#"
let a = "a";
!a = "value";

"#
        ),
        vec!["error".to_string()]
    );

    // Test for undefined variable
    assert_eq!(
        run("unknown = \"what\""),
        vec![
            "error".to_string()
        ]
    );
}

#[test]
fn test_bool() {
    // Test for equals with boolean values
    assert_eq!(
        run("print(true == true);"),
        vec!["true".to_string()]
    );
    assert_eq!(
        run("print(true == false);"),
        vec!["false".to_string()]
    );
    assert_eq!(
        run("print(false == true);"),
        vec!["false".to_string()]
    );
    assert_eq!(
        run("print(false == false);"),
        vec!["true".to_string()]
    );

    // Test for equals with mixed types
    assert_eq!(
        run("print(true == 1);"),
        vec!["false".to_string()]
    );
    assert_eq!(
        run("print(false == 0);"),
        vec!["false".to_string()]
    );
    assert_eq!(
        run("print(true == \"true\");"),
        vec!["false".to_string()]
    );
    assert_eq!(
        run("print(false == \"false\");"),
        vec!["false".to_string()]
    );
    assert_eq!(
        run("print(false == \"\");"),
        vec!["false".to_string()]
    );

    // Test for not equals with boolean values
    assert_eq!(
        run("print(true != true);"),
        vec!["false".to_string()]
    );
    assert_eq!(
        run("print(true != false);"),
        vec!["true".to_string()]
    );
    assert_eq!(
        run("print(false != true);"),
        vec!["true".to_string()]
    );
    assert_eq!(
        run("print(false != false);"),
        vec!["false".to_string()]
    );

    // Test for not equals with mixed types
    assert_eq!(
        run("print(true != 1);"),
        vec!["true".to_string()]
    );
    assert_eq!(
        run("print(false != 0);"),
        vec!["true".to_string()]
    );
    assert_eq!(
        run("print(true != \"true\");"),
        vec!["true".to_string()]
    );
    assert_eq!(
        run("print(false != \"false\");"),
        vec!["true".to_string()]
    );
    assert_eq!(
        run("print(false != \"\");"),
        vec!["true".to_string()]
    );
}

#[test]
fn test_call() {
    // Tests for calling non-functions
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
    // Test for generic closures
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

"#
        ),
        vec!["1".to_string(), "2".to_string()]
    );

    // Test for closures with parameters
    assert_eq!(
        run(r#"
let f;
def foo(param):
    def f_():
        print(param);
    f = f_;
foo("param");
f();

"#
        ),
        vec!["param".to_string()]
    );

    // Test for simple closures
    assert_eq!(
        run(r#"
def f():
    let a = "a";
    let b = "b";
    def g():
        print(b);
        print(a);
    g();
f();

"#
        ),
        vec!["b".to_string(), "a".to_string()]
    );

    // Tests for nested closures
    assert_eq!(
        run(r#"
let f;
def f1():
    let a = "a";
    def f2():
        let b = "b";
        def f3():
            let c = "c";
            def f4():
                print(a);
                print(b);
                print(c);
            f = f4;
        f3();
    f2();
f1();
f();

"#
        ),
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}

#[test]
fn test_comparison() {
    // Test less than
    assert_eq!(
        run("print(1 < 2);"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(2 < 2);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(2 < 1);"),
        vec!["false".to_string()]
    );

    // Test less than or equal to
    assert_eq!(
        run("print(1 <= 2);"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(2 <= 2);"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(2 <= 1);"),
        vec!["false".to_string()]
    );

    // Test greater than
    assert_eq!(
        run("print(1 > 2);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(2 > 2);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(2 > 1);"),
        vec!["true".to_string()]
    );

    // Test greater than or equal to
    assert_eq!(
        run("print(1 >= 2);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(2 >= 2);"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(2 >= 1);"),
        vec!["true".to_string()]
    );

    // Test zeros and negatives
    assert_eq!(
        run("print(0 < -0);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(-0 < 0);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(0 <= -0);"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(-0 <= 0);"),
        vec!["true".to_string()]
    );
}

#[test]
fn test_equality() {
    // Test null equality
    assert_eq!(
        run("print(null == null);"),
        vec!["true".to_string()]
    );

    // Test boolean equality
    assert_eq!(
        run("print(true == true);"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(true == false);"),
        vec!["false".to_string()]
    );

    // Test numerical inequality
    assert_eq!(
        run("print(1 == 1);"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(1 == 2);"),
        vec!["false".to_string()]
    );

    // Test string equality
    assert_eq!(
        run("print(\"str\" == \"str\");"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(\"str\" == \"ing\");"),
        vec!["false".to_string()]
    );

    // Test mixed type equality
    assert_eq!(
        run("print(false == null);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(false == 0);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(0 == \"0\");"),
        vec!["false".to_string()]
    );
}

#[test]
fn test_for_loops() {
    // Test for simple for loop
    assert_eq!(
        run(r#"
for i in 0..3:
    print(i);

"#
        ),
        vec!["0".to_string(), "1".to_string(), "2".to_string()]
    );

    // Test for for loop in functions
    assert_eq!(
        run(r#"
def foo():
    for _ in 0..1:
        return "done";
print(foo());

"#
        ),
        vec!["done".to_string()]
    );

    // Test for closures in for loop
    assert_eq!(
        run(r#"
def f():
    for _ in 0..1:
        let i = "i";
        def g():
            print(i);
        return g;
let h = f();
h();

"#
        ),
        vec!["i".to_string()]
    );

    // Test for for loop with step
    assert_eq!(
        run(r#"
for i in 0..5 step 2:
    print(i);

"#
        ),
        vec!["0".to_string(), "2".to_string(), "4".to_string()]
    );
}

#[test]
fn test_functions() {
    // Test for extra arguments
    assert_eq!(
        run(r#"
def f(x, y):
    print(x);
    print(y);
f(1, 2, 3, 4);

"#
        ),
        vec!["error".to_string()]
    );

    // Test for missing arguments
    assert_eq!(
        run(r#"
def add(x, y):
    return x + y;
print(add(1));

"#
        ),
        vec!["error".to_string()]
    );

    // Tests mutual recursion
    assert_eq!(
        run(r#"
def isEven(n):
    if n == 0:
        return true;
    return isOdd(n - 1);
def isOdd(n):
    if n == 0:
        return false;
    return isEven(n - 1);
print(isEven(4));

"#
        ),
        vec!["error".to_string()]
    );

    // Test recursion
    assert_eq!(
        run(r#"
def fib(n):
    if n < 2:
        return n;
    return fib(n - 1) + fib(n - 2);
print(fib(8));

"#
        ),
        vec!["21".to_string()]
    );

    // Test missing comma in parameters
    assert_eq!(
        run(r#"
def f(a, b c, d, e, f):
    return a;

"#
        ),
        vec!["error".to_string()]
    );
}

#[test]
fn test_hash() {
    // Tests for hash function
    assert_eq!(
        run("print(hash(\"123\"));"),
        vec!["a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3".to_string()]
    );

    assert_eq!(
        run("print(hash(\"a4b j2%2@6HK\"));"),
        vec!["0ddff3ce9c7152874283c174235342d9e9dae2d9c4a486215beae162ace030b4".to_string()]
    );

    // Tests for hash equality
    assert_eq!(
        run("print(hash(\"abc\") == hash(\"abc\"));"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(hash(\"abc\") == hash(\"def\"));"),
        vec!["false".to_string()]
    );
}

#[test]
fn test_if() {
    // Test for simple if condition
    assert_eq!(
        run(r#"
if true:
    print("true");
else:
    print("false");

"#
        ),
        vec!["true".to_string()]
    );

    // Test for variable condition
    assert_eq!(
        run(r#"
let a = 3;
if a == 2:
    print("true");
else if a == 3:
    print("false");
else:
    print("else");

"#
        ),
        vec!["false".to_string()]
    );

    // Tests for truthy values in condition
    assert_eq!(
        run(r#"
if 1:
    print("true");
else:
    print("false");

"#
        ),
        vec!["true".to_string()]
    );

    assert_eq!(
        run(r#"
if "string":
    print("true");
else:
    print("false");

"#
        ),
        vec!["true".to_string()]
    );

    assert_eq!(
        run(r#"
if "":
    print("empty");

"#
        ),
        vec!["empty".to_string()]
    );

    // Test for assignment in if condition
    assert_eq!(
        run(r#"
let a = false;
if a = true:
    print(a);

"#
        ),
        vec!["true".to_string()]
    );

    // Test for not in if condition
    assert_eq!(
        run(r#"
let a = [1, 2, 3];
if 4 not in a:
    print("true");

"#
        ),
        vec!["true".to_string()]
    );

    // Tests for multiple conditions
    assert_eq!(
        run(r#"
let a = 3;
if a == 2 or a == 3:
    print("true");

"#
        ),
        vec!["true".to_string()]
    );

    assert_eq!(
        run(r#"
let a = 2;
let b = 3;
if a == 2 and b == 3:
    print("true");

"#
        ),
        vec!["true".to_string()]
    );

    // Test for nested if conditions
    assert_eq!(
        run(r#"
let a = 3;
if a == 2:
    print("false");
else:
    if a == 3:
        print("true");

"#
        ),
        vec!["true".to_string()]
    );

    // Test for nested if conditions
    assert_eq!(
        run(r#"
if 1 == 1:
    if 2 == 2:
        if 3 == 3:
            print(3);
        print(2);
    print(1);
print(0);

"#
        ),
        vec!["3".to_string(), "2".to_string(), "1".to_string(), "0".to_string()]

    )
}

#[test]
fn test_lists() {
    // Test for list creation
    assert_eq!(
        run(r#"
let a = [1, 2, 3];
print(a);

"#
        ),
        vec!["[1, 2, 3]".to_string()]
    );

    // Test for list indexing
    assert_eq!(
        run(r#"
let a = [1, 2, 3];
print(a[0]);
print(a[1]);
print(a[2]);

"#
        ),
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    );

    // Tests for list slicing
    assert_eq!(
        run(r#"
let a = [1, 2, 3, 4, 5];
print(a[1:3]);

"#
        ),
        vec!["[2, 3, 4]".to_string()]
    );

    assert_eq!(
        run(r#"
let a = [1, 2, 3, 4, 5];
print(a[:3]);

"#
        ),
        vec!["[1, 2, 3, 4]".to_string()]
    );

    assert_eq!(
        run(r#"
let a = [1, 2, 3, 4, 5];
print(a[2:]);

"#
        ),
        vec!["[3, 4, 5]".to_string()]
    );

    // Ensure that lists cannot be added together
    assert_eq!(
        run(r#"
let a = [1, 2, 3];
let b = [4, 5, 6];
print(a + b);

"#
        ),
        vec!["error".to_string()]
    );

    // Test for pushing items to the end of a list
    assert_eq!(
        run(r#"
let a = [1, 2, 3];
a.push(4);
print(a);

"#
        ),
        vec!["[1, 2, 3, 4]".to_string()]
    );

    // Test for popping items from a list
    assert_eq!(
        run(r#"
let a = ["apple", "banana", "cherry"];
let b = a.pop();
print(b);
print(a);

"#
        ),
        vec!["cherry".to_string(), "[\"apple\", \"banana\"]".to_string()]
    );

    // Test for removing items from a list
    assert_eq!(
        run(r#"
let a = ["apple", "banana", "cherry"];
a.remove(1);
print(a);

"#
        ),
        vec!["[\"apple\", \"cherry\"]".to_string()]
    );

    // Test for inserting items into a list
    assert_eq!(
        run(r#"
let a = [1, 2, 3];
a.insertAt(1, 4);
print(a);

"#
        ),
        vec!["[1, 4, 2, 3]".to_string()]
    );

    // Test for getting the index of an item in a list
    assert_eq!(
        run(r#"
let a = ["apple", "banana", "cherry"];
print(a.index("banana"));

"#
        ),
        vec!["1".to_string()]
    );

    // Test for getting the length of a list
    assert_eq!(
        run(r#"
let a = [1, 2, 3, 4, 5, 6, 7];
print(a.len());

"#
        ),
        vec!["7".to_string()]
    );

    // Test for sorting a list
    assert_eq!(
        run(r#"
let a = [3, 2, 1, 4, 5];
a.sort();
print(a);

"#
        ),
        vec!["[1, 2, 3, 4, 5]".to_string()]
    );

    // Test for looping through a list
    assert_eq!(
        run(r#"
let items = ["apple", "banana", "cherry"];
for i in 0..items.len():
    print(items[i]);

"#
        ),
        vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()]
    );
}

#[test]
fn test_logical_operators() {
    // Works because it returns the first non-true argument
    assert_eq!(
        run(r#"
print(false and 1);
print(true and 1);
print(1 and 2 and false);

"#
        ),
        vec!["false".to_string(), "1".to_string(), "false".to_string()]
    );

    // Works because it returns the last argument if all are true
    assert_eq!(
        run(r#"
print(1 and true);
print(1 and 2 and 3);

"#
        ),
        vec!["true".to_string(), "3".to_string()]
    );


    // Special cases which short-circuits at the first false argument
    assert_eq!(
        run(r#"
let a = "before";
let b = "before";
(a = true) and (b = false) and (a = "bad");
print(a);
print(b);

"#
        ),
        vec!["true".to_string(), "false".to_string()]
    );

    assert_eq!(
        run(r#"
let a = "before";
let b = "before";
(a = false) or (b = true) or (a = "bad");
print(a);
print(b);

"#
        ),
        vec!["false".to_string(), "true".to_string()]
    );

    // Test for and operator and its precedence
    assert_eq!(
        run(r#"
print(false and "bad");

"#
        ),
        vec!["false".to_string()]
    );

    assert_eq!(
        run(r#"
print(null and "bad");

"#
        ),
        vec!["null".to_string()]
    );

    assert_eq!(
        run(r#"
print(true and "ok");

"#
        ),
        vec!["ok".to_string()]
    );

    assert_eq!(
        run(r#"
print(0 and "ok");

"#
        ),
        vec!["ok".to_string()]
    );

    assert_eq!(
        run(r#"
print("" and "ok");

"#
        ),
        vec!["ok".to_string()]
    );

    // Tests for or operator and its precedence
    assert_eq!(
        run(r#"
print(1 or true);
print(false or 1);
print(false or false or true);

"#
        ),
        vec!["1".to_string(), "1".to_string(), "true".to_string()]
    );

    assert_eq!(
        run(r#"
print(false or false);
print(false or false or 0);

"#
        ),
        vec!["false".to_string(), "0".to_string()]
    );

    assert_eq!(
        run(r#"
print(false or "ok");
print(null or "ok");

"#
        ),
        vec!["ok".to_string(), "ok".to_string()]
    );

    assert_eq!(
        run(r#"
print(true or "ok");
print(0 or "ok");
print("s" or "ok");

"#
        ),
        vec!["true".to_string(), "0".to_string(), "s".to_string()]
    );
}

#[test]
fn test_math() {
    // Testing addition
    assert_eq!(
        run("print(123 + 456);"),
        vec!["579".to_string()]
    );

    assert_eq!(
        run("print(\"str\" + \"ing\");"),
        vec!["string".to_string()]
    );

    // Testing invalid addition
    assert_eq!(
        run("print(true + null);"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("print(true + 123);"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("print(true + \"str\");"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("print(null + null);"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("print(null + 123);"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("print(null + \"str\");"),
        vec!["error".to_string()]
    );

    // Test subtraction
    assert_eq!(
        run("print(123 - 456);"),
        vec!["-333".to_string()]
    );

    assert_eq!(
        run("print(1.2 - 1.2);"),
        vec!["0".to_string()]
    );

    // Testing invalid subtraction
    assert_eq!(
        run("print(true - null);"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("print(\"1\" - 1);"),
        vec!["error".to_string()]
    );

    // Test multiplication
    assert_eq!(
        run("print(5 * 3);"),
        vec!["15".to_string()]
    );

    assert_eq!(
        run("print(12.34 * 0.3);"),
        vec!["3.702".to_string()]
    );

    // Test invalid multiplication
    assert_eq!(
        run("print(true * \"str\");"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("print(\"123\" * 123);"),
        vec!["error".to_string()]
    );

    // Test division
    assert_eq!(
        run("print(10 / 2);"),
        vec!["5".to_string()]
    );

    assert_eq!(
        run("print(12.34 / 12.34);"),
        vec!["1".to_string()]
    );

    // Test invalid division
    assert_eq!(
        run("print(true / null);"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("print(\"123\" / 123);"),
        vec!["error".to_string()]
    );

    // Test negation
    assert_eq!(
        run("print(-1);"),
        vec!["-1".to_string()]
    );

    assert_eq!(
        run("print(-(-1));"),
        vec!["1".to_string()]
    );
}

#[test]
fn test_membership() {
    // Test for membership in lists
    assert_eq!(
        run(r#"
let a = [1, 2, 3];
print(1 in a);
print(4 in a);
print(1 not in a);
print(4 not in a);

"#
        ),
        vec![
            "true".to_string(),
            "false".to_string(),
            "false".to_string(),
            "true".to_string()
        ]
    );

    // Test for membership in membership in condition
    assert_eq!(
        run(r#"
let a = [1, 2, 3];
if 1 in a:
    print("1");

if 4 in a:
    print("2");

if 1 not in a:
    print("3");

if 4 not in a:
    print("4");
"#
        ),
        vec![
            "1".to_string(),
            "4".to_string(),
        ]
    );
}

#[test]
fn test_negation() {
    // Tests for negating booleans
    assert_eq!(
        run("print(!true);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(!false);"),
        vec!["true".to_string()]
    );

    // Tests for double negation
    assert_eq!(
        run("print(!!true);"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(!!false);"),
        vec!["false".to_string()]
    );

    // Tests for negating different values
    assert_eq!(
        run("print(!123);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(!0);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(!null);"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(!\"\");"),
        vec!["false".to_string()]
    );

    // Test for negating a function
    assert_eq!(
        run(r#"
def foo():
    return true;
print(!foo());

"#
        ),
        vec!["false".to_string()]
    );
}

#[test]
fn test_not_equals() {
    // Test null
    assert_eq!(
        run("print(null != null);"),
        vec!["false".to_string()]
    );

    // Test boolean
    assert_eq!(
        run("print(true != true);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(true != false);"),
        vec!["true".to_string()]
    );

    // Test numerical
    assert_eq!(
        run("print(1 != 1);"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(1 != 2);"),
        vec!["true".to_string()]
    );

    // Test strings
    assert_eq!(
        run("print(\"str\" != \"str\");"),
        vec!["false".to_string()]
    );

    assert_eq!(
        run("print(\"str\" != \"ing\");"),
        vec!["true".to_string()]
    );

    // Test mixed types
    assert_eq!(
        run("print(false != null);"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(false != 0);"),
        vec!["true".to_string()]
    );

    assert_eq!(
        run("print(0 != \"0\");"),
        vec!["true".to_string()]
    );
}

#[test]
fn test_nums()  {
    // Tests for decimal points in numbers
    assert_eq!(
        run("print(123.);"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("print(.123);"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("print(123.456);"),
        vec!["123.456".to_string()]
    );

    // Test for zero
    assert_eq!(
        run("print(0);"),
        vec!["0".to_string()]
    );

    assert_eq!(
        run("print(-0);"),
        vec!["-0".to_string()]
    );

    // Test for mix of decimals and negatives
    assert_eq!(
        run("print(-123.456);"),
        vec!["-123.456".to_string()]
    );

    assert_eq!(
        run("print(-0.001);"),
        vec!["-0.001".to_string()]
    );

    // NaN tests
    assert_eq!(
        run("print(0 / 0);"),
        vec!["NaN".to_string()]
    );

    assert_eq!(
        run(r#"
let nan = 0 / 0;
print(nan == 0);
print(nan != 1);
print(nan == nan);
print(nan != nan);

"#
        ),
        vec!["false".to_string(), "true".to_string(), "false".to_string(), "true".to_string()]
    )
}

#[test]
fn test_precedence() {
    // Tests for BODMAS precedence
    assert_eq!(
        run("print(2 + 3 * 4);"),
        vec!["14".to_string()]
    );

    assert_eq!(
        run("print(20 - 3 * 4);"),
        vec!["8".to_string()]
    );

    assert_eq!(
        run("print(2 + 6 / 3);"),
        vec!["4".to_string()]
    );

    assert_eq!(
        run("print(2 - 6 / 3);"),
        vec!["0".to_string()]
    );

    // Test for spacing with - symbol
    assert_eq!(
        run("print(1- 1);"),
        vec!["0".to_string()]
    );

    // Test for left associativity
    assert_eq!(
        run("print(1 - 1 - 1);"),
        vec!["-1".to_string()]
    );

    // Test for right associativity
    assert_eq!(
        run("print(1 - (1 - 1));"),
        vec!["1".to_string()]
    );

    // Test for precedence with grouping
    assert_eq!(
        run("print(2 * (6 - (2 + 2)));"),
        vec!["4".to_string()]
    );
}

#[test]
fn test_print() {
    // Test printing a simple string
    assert_eq!(
        run("print(\"Hello, World!\");"),
        vec!["Hello, World!".to_string()]
    );

    // Test an empty print statement
    assert_eq!(
        run("print();"),
        vec!["error".to_string()]
    );

    // Test printing an assignment statement
    assert_eq!(
        run(r#"
let a = 2;
print(a = a + 1);
print(a);

"#
        ),
        vec!["3".to_string(), "3".to_string()]
    );
}

#[test]
fn test_returns() {
    // Test for returning in an else branch
    assert_eq!(
        run(r#"
def f():
    if false:
        "no";
    else:
        return "ok";
print(f());

"#
        ),
        vec!["ok".to_string()]
    );

    // Test for returning in an if statement
    assert_eq!(
        run(r#"
def f():
    if true:
        return "ok";
print(f());

"#
        ),
        vec!["ok".to_string()]
    );

    // Test for returning in a while loop
    assert_eq!(
        run(r#"
def f():
    while true:
        return "ok";
print(f());

"#
        ),
        vec!["ok".to_string()]
    );

    // Test for returning at the top level
    assert_eq!(
        run("return \"at top level\";"),
        vec!["error".to_string()]
    );

    // Test for unreachable print after return
    assert_eq!(
        run(r#"
def f():
    return;
    print("unreachable");
print(f());

"#
        ),
        vec!["null".to_string()]
    );
}

#[test]
fn test_strings() {
    // Test for string concatenation
    assert_eq!(
        run("print(\"(\" + \"\" + \")\");"),
        vec!["()".to_string()]
    );

    // Test for simple strings
    assert_eq!(
        run("print(\"some string\");"),
        vec!["some string".to_string()]
    );
}

#[test]
fn test_variables() {
    // Test for simple variable declaration
    assert_eq!(
        run("let a = 1; print(a);"),
        vec!["1".to_string()]
    );

    // Test for variable name colliding with parameter
    assert_eq!(
        run(r#"
def foo(a):
    let a;

"#
        ),
        vec!["error".to_string()]
    );

    // Test for duplicate local variables
    assert_eq!(
        run("let a = \"value\"; let a = \"other\";"),
        vec!["error".to_string()]
    );

    // Test for duplicate parameters
    assert_eq!(
        run(r#"
def foo(arg, arg):
    return arg;

"#
        ),
        vec!["error".to_string()]
    );

    // Test for variable shadowing
    assert_eq!(
        run(r#"
let x = 10;
def f():
    let x = 5;
    print(x);
f();
print(x);

"#
        ),
        vec!["5".to_string(), "10".to_string()]
    );

    // Test for variable concatenation
    assert_eq!(
        run(r#"
let a = "a";
print(a);
let b = a + " b";
print(b);
let c = a + " c";
print(c);
let d = b + " d";
print(d);

"#
        ),
        vec!["a".to_string(), "a b".to_string(), "a c".to_string(), "a b d".to_string()]
    );

    // Tests for unassigned variables
    assert_eq!(
        run(r#"
let a = "1";
let a;
print(a);

"#
        ),
        vec!["error".to_string()]
    );

    assert_eq!(
        run("let a; print(a);"),
        vec!["null".to_string()]
    );

    // Tests for undefined variables
    assert_eq!(
        run("print(notDefined);"),
        vec!["error".to_string()]
    );

    assert_eq!(
        run(r#"
if false:
    print(notDefined);
print("ok");

"#
        ),
        vec!["error".to_string()]
    );

    // Tests for reserved keywords
    assert_eq!(
        run("let false = \"value\";"),
        vec!["error".to_string()]
    );

    
    assert_eq!(
        run("let null = \"value\";"),
        vec!["error".to_string()]
    );

    // Test for redefining variables
    assert_eq!(
        run(r#"
let a = "value";
let a = a;
print(a);

"#
        ),
        vec!["error".to_string()]
    );
}

#[test]
fn test_while() {
    // Test for while loop with return closure
    assert_eq!(
        run(r#"
def f():
    while true:
        let i = "i";
        def g():
            print(i);
        return g;
let h = f();
h();

"#
        ),
        vec!["i".to_string()]
    );

    // Test for return inside while loop
    assert_eq!(
        run(r#"
def f():
    while true:
        let i = "i";
        return i;
print(f());

"#
        ),
        vec!["i".to_string()]
    );

    // Test normal while loop
    assert_eq!(
        run(r#"
let i = 0;
while i < 5:
    print(i++);
print(i);

"#
        ),
        vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "5".to_string()
        ]
    );
}
