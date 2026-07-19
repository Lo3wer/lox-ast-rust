use lox_ast_rust::datastructs::expr::Expr;
use lox_ast_rust::datastructs::literal::Literal;
use lox_ast_rust::datastructs::stmt::Stmt;
use lox_ast_rust::datastructs::token::{Token, TokenType};
use lox_ast_rust::evaluator::Evaluator;
use lox_ast_rust::lexer::Lexer;
use lox_ast_rust::parser::Parser;
use lox_ast_rust::resolver::Resolver;

fn run_captured(source: &str, _buf: &mut Vec<u8>) -> Result<(), String> {
    let mut lexer = Lexer::new(source.to_string());
    let (tokens, lex_errors) = lexer.scan_tokens();
    if !lex_errors.is_empty() {
        return Err(format!("Lex errors: {:?}", lex_errors));
    }

    let mut parser = Parser::new(tokens);
    let statements = parser.parse().map_err(|e| format!("Parse errors: {:?}", e))?;

    let mut evaluator = Evaluator::new();
    let mut resolver = Resolver::new(&mut evaluator);
    resolver.resolve(&statements).map_err(|e| format!("Resolve error: {:?}", e))?;

    evaluator.interpret(statements).map_err(|e| format!("Runtime error: {:?}", e))
}

// --- Programs that should run successfully ---

#[test]
fn test_literal_expression_runs() {
    assert!(run_captured("42;", &mut Vec::new()).is_ok());
}

#[test]
fn test_print_runs() {
    assert!(run_captured("print 42;", &mut Vec::new()).is_ok());
}

#[test]
fn test_var_declaration_runs() {
    assert!(run_captured("var x = 10;", &mut Vec::new()).is_ok());
}

#[test]
fn test_var_assign_runs() {
    assert!(run_captured("var x = 1; x = 2;", &mut Vec::new()).is_ok());
}

#[test]
fn test_binary_arithmetic_runs() {
    assert!(run_captured("print 1 + 2 * 3;", &mut Vec::new()).is_ok());
}

#[test]
fn test_comparison_runs() {
    assert!(run_captured("print 1 < 2;", &mut Vec::new()).is_ok());
}

#[test]
fn test_equality_runs() {
    assert!(run_captured("print 1 == 2;", &mut Vec::new()).is_ok());
}

#[test]
fn test_logical_and_runs() {
    assert!(run_captured("print true and false;", &mut Vec::new()).is_ok());
}

#[test]
fn test_logical_or_runs() {
    assert!(run_captured("print true or false;", &mut Vec::new()).is_ok());
}

#[test]
fn test_ternary_runs() {
    assert!(run_captured("print true ? 1 : 2;", &mut Vec::new()).is_ok());
}

#[test]
fn test_block_scope_runs() {
    assert!(run_captured("{ var x = 1; print x; }", &mut Vec::new()).is_ok());
}

#[test]
fn test_if_true_runs() {
    assert!(run_captured("if (true) print 1; else print 2;", &mut Vec::new()).is_ok());
}

#[test]
fn test_if_false_runs() {
    assert!(run_captured("if (false) print 1; else print 2;", &mut Vec::new()).is_ok());
}

#[test]
fn test_while_loop_runs() {
    assert!(run_captured("var i = 0; while (i < 3) { i = i + 1; }", &mut Vec::new()).is_ok());
}

#[test]
fn test_for_loop_runs() {
    assert!(run_captured(
        "for (var i = 0; i < 3; i = i + 1) { print i; }",
        &mut Vec::new(),
    ).is_ok());
}

#[test]
fn test_function_call_runs() {
    assert!(run_captured(
        "fun f(x) { return x * 2; } print f(21);",
        &mut Vec::new(),
    ).is_ok());
}

#[test]
fn test_recursive_function_runs() {
    assert!(run_captured(
        "fun fib(n) { if (n < 2) return n; return fib(n - 1) + fib(n - 2); } print fib(10);",
        &mut Vec::new(),
    ).is_ok());
}

#[test]
fn test_class_runs() {
    assert!(run_captured(
        "class Foo { bar() { return 42; } } var f = Foo(); print f.bar();",
        &mut Vec::new(),
    ).is_ok());
}

#[test]
fn test_class_inheritance_super_runs() {
    assert!(run_captured(
        "class A { method() { return 1; } } \
         class B < A { method() { return super.method() + 1; } } \
         var b = B(); \
         print b.method();",
        &mut Vec::new(),
    ).is_ok());
}

#[test]
fn test_closure_runs() {
    assert!(run_captured(
        "fun outer() { var x = 1; fun inner() { return x; } return inner; } \
         var f = outer(); \
         print f();",
        &mut Vec::new(),
    ).is_ok());
}

#[test]
fn test_nested_block_scope_runs() {
    assert!(run_captured(
        "var x = 0; { var x = 1; print x; } print x;",
        &mut Vec::new(),
    ).is_ok());
}

// --- Programs that should produce errors ---

#[test]
fn test_undefined_variable_error() {
    assert!(run_captured("print nosuch;", &mut Vec::new()).is_err());
}

#[test]
fn test_type_error_addition() {
    assert!(run_captured("print true + 1;", &mut Vec::new()).is_err());
}

#[test]
fn test_division_by_zero() {
    assert!(run_captured("print 1 / 0;", &mut Vec::new()).is_err());
}

#[test]
fn test_wrong_arity() {
    assert!(run_captured("fun f(a) { } f(1, 2);", &mut Vec::new()).is_err());
}

// --- Constructed AST tests (bypassing parser) ---

#[test]
fn test_evaluator_direct_literal() {
    let mut evaluator = Evaluator::new();
    let stmt = Stmt::Print {
        expression: Box::new(Expr::Literal { value: Literal::Number(42.0), id: 1 }),
    };
    assert!(evaluator.interpret(vec![stmt]).is_ok());
}

#[test]
fn test_evaluator_direct_binary() {
    let mut evaluator = Evaluator::new();
    let lhs = Expr::Literal { value: Literal::Number(2.0), id: 1 };
    let rhs = Expr::Literal { value: Literal::Number(3.0), id: 2 };
    let op = Token::new(TokenType::Plus, "+".to_string(), None, 0);
    let expr = Expr::Binary { left: Box::new(lhs), operator: op, right: Box::new(rhs), id: 3 };
    let stmt = Stmt::Print { expression: Box::new(expr) };
    assert!(evaluator.interpret(vec![stmt]).is_ok());
}

#[test]
fn test_return_from_toplevel_error() {
    let source = "return 1;";
    let mut lexer = Lexer::new(source.to_string());
    let (tokens, _) = lexer.scan_tokens();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("parse should succeed");
    let mut evaluator = Evaluator::new();
    let mut resolver = Resolver::new(&mut evaluator);
    assert!(resolver.resolve(&statements).is_err());
}
