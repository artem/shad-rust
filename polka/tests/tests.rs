use polka::{Interpreter, Value};

use pretty_assertions::assert_eq;

fn test(inter: &mut Interpreter, expr: &str, stack: &[Value]) {
    inter.eval(expr);
    assert_eq!(inter.stack(), stack);
}

#[test]
fn test_simple() {
    let mut inter = Interpreter::new();
    test(&mut inter, "3 2 +", &[Value::Number(5.)]);
    test(&mut inter, "5 -", &[Value::Number(0.)]);
    test(
        &mut inter,
        "5 5 5 * *",
        &[Value::Number(0.), Value::Number(125.)],
    );
    test(&mut inter, "250 /", &[Value::Number(0.), Value::Number(2.)]);
}

#[test]
fn test_order() {
    let mut inter = Interpreter::new();
    test(&mut inter, "1 2 -", &[Value::Number(1.)]);
    test(&mut inter, "2 /", &[Value::Number(2.)]);
}

#[test]
fn test_variables() {
    let mut inter = Interpreter::new();
    test(&mut inter, "4 5 * 'x set", &[]);
    test(&mut inter, "4 $x +", &[Value::Number(24.)]);
    test(
        &mut inter,
        "'x",
        &[Value::Number(24.), Value::Symbol("x".to_string())],
    );
    test(&mut inter, "set $x $x *", &[Value::Number(24. * 24.)]);
}

#[test]
fn test_symbol_variable() {
    let mut inter = Interpreter::new();
    test(&mut inter, "'y 'x set", &[]);
    test(&mut inter, "$x", &[Value::Symbol("y".to_string())]);
    test(&mut inter, "20 $x set", &[Value::Symbol("y".to_string())]);
    test(
        &mut inter,
        "$y $y *",
        &[Value::Symbol("y".to_string()), Value::Number(400.)],
    );
}

#[test]
fn test_whitespace() {
    let mut inter = Interpreter::new();
    test(&mut inter, "3\n5\t10\r+   \n\r*", &[Value::Number(45.)]);
}

#[test]
#[should_panic]
fn test_set_type_error() {
    let mut inter = Interpreter::new();
    inter.eval("5 10 set");
}

#[test]
#[should_panic]
fn test_arithmetic_error() {
    let mut inter = Interpreter::new();
    inter.eval("5 'foo +");
}

#[test]
#[should_panic]
fn test_not_a_number() {
    let mut inter = Interpreter::new();
    inter.eval("hello");
}

#[test]
#[should_panic]
fn test_name_error() {
    let mut inter = Interpreter::new();
    inter.eval("5 $a +");
}

#[test]
#[should_panic]
fn test_empty_stack() {
    let mut inter = Interpreter::new();
    inter.eval("1 +");
}
