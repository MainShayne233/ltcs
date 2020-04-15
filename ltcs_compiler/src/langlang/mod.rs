// based on: http://lisperator.net/pltut/parser/

lalrpop_mod!(pub grammar, "/langlang/grammar.rs");
mod ast;

use ast::{AssignOperator, BinaryOperator, Expression};

pub fn parse(input: &str) -> Result<Expression, String> {
    let parser = grammar::ProgramParser::new();
    match parser.parse(input) {
        Ok(program) => Ok(program),
        _ => Err(String::from("Failed to parse")),
    }
}

#[test]
fn test_parse_program_that_is_single_expression_that_is_just_a_number() {
    let parser = grammar::ProgramParser::new();
    let input = "1;";
    let result = parser.parse(input).unwrap();
    let expected = Expression::Program {
        lines: Box::new(vec![Expression::Num { value: 1 }]),
    };
    assert_eq!(result, expected);
}

#[test]
fn test_parse_program_that_is_two_expressions_that_are_just_numbers() {
    let parser = grammar::ProgramParser::new();
    let input = r#"
1;
2;
"#;
    let result = parser.parse(input).unwrap();
    let expected = Expression::Program {
        lines: Box::new(vec![
            Expression::Num { value: 1 },
            Expression::Num { value: 2 },
        ]),
    };
    assert_eq!(result, expected);
}

#[test]
fn test_parse_program_that_is_binary_expression() {
    let parser = grammar::ProgramParser::new();
    let input = "1 + 2;";
    let result = parser.parse(input).unwrap();
    let expected = Expression::Program {
        lines: Box::new(vec![Expression::Binary {
            operator: BinaryOperator::Plus,
            left: Box::new(Expression::Num { value: 1 }),
            right: Box::new(Expression::Num { value: 2 }),
        }]),
    };
    assert_eq!(result, expected);
}

#[test]
fn test_parse_program_that_is_assign_expression() {
    let parser = grammar::ProgramParser::new();
    let input = "a = 2;";
    let result = parser.parse(input).unwrap();
    let expected = Expression::Program {
        lines: Box::new(vec![Expression::Assign {
            operator: AssignOperator::Equals,
            left: Box::new(Expression::Var {
                value: String::from("a"),
            }),
            right: Box::new(Expression::Num { value: 2 }),
        }]),
    };
    assert_eq!(result, expected);
}

#[test]
fn test_var_that_is_just_a_single_word() {
    let parser = grammar::ProgramParser::new();
    let input = "somevar;";
    let result = parser.parse(input).unwrap();
    let expected = Expression::Program {
        lines: Box::new(vec![Expression::Var {
            value: String::from("somevar"),
        }]),
    };
    assert_eq!(result, expected);
}

#[test]
fn test_var_that_is_just_a_multiple_words_seperated_by_dashes() {
    let parser = grammar::ProgramParser::new();
    let input = "some-var-i-made;";
    let result = parser.parse(input).unwrap();
    let expected = Expression::Program {
        lines: Box::new(vec![Expression::Var {
            value: String::from("some-var-i-made"),
        }]),
    };
    assert_eq!(result, expected);
}

#[test]
fn test_single_call() {
    let parser = grammar::ProgramParser::new();
    let input = "add(x, 1);";
    let result = parser.parse(input).unwrap();
    let expected = Expression::Program {
        lines: Box::new(vec![Expression::Call {
            func: Box::new(Expression::Var {
                value: String::from("add"),
            }),
            args: Box::new(vec![
                Expression::Var {
                    value: String::from("x"),
                },
                Expression::Num { value: 1 },
            ]),
        }]),
    };
    assert_eq!(result, expected);
}

#[test]
fn test_string() {
    let parser = grammar::ProgramParser::new();
    let input = "\"Hello, world!\";";
    let result = parser.parse(input).unwrap();
    let expected = Expression::Program {
        lines: Box::new(vec![Expression::String {
            value: String::from("Hello, world!"),
        }]),
    };
    assert_eq!(result, expected);
}

#[test]
fn test_lambda() {
    let parser = grammar::ProgramParser::new();
    let input = r#"
lambda (x, y) {
  x + y;
};
"#;
    let result = parser.parse(input).unwrap();
    let expected = Expression::Program {
        lines: Box::new(vec![Expression::Lambda {
            vars: vec![String::from("x"), String::from("y")],
            body: Box::new(Expression::Program {
                lines: Box::new(vec![Expression::Binary {
                    operator: BinaryOperator::Plus,
                    left: Box::new(Expression::Var {
                        value: String::from("x"),
                    }),
                    right: Box::new(Expression::Var {
                        value: String::from("y"),
                    }),
                }]),
            }),
        }]),
    };
    assert_eq!(result, expected);
}

#[test]
fn test_complex_program_example() {
    let parser = grammar::ProgramParser::new();
    let input = r#"
sum = lambda (a, b) {
    a + b;
};
print(sum(1, 2));
"#;
    let result = parser.parse(input).unwrap();
    let expected = Expression::Program {
        lines: Box::new(vec![
            Expression::Assign {
                operator: AssignOperator::Equals,
                left: Box::new(Expression::Var {
                    value: String::from("sum"),
                }),
                right: Box::new(Expression::Lambda {
                    vars: vec![String::from("a"), String::from("b")],
                    body: Box::new(Expression::Program {
                        lines: Box::new(vec![Expression::Binary {
                            operator: BinaryOperator::Plus,
                            left: Box::new(Expression::Var {
                                value: String::from("a"),
                            }),
                            right: Box::new(Expression::Var {
                                value: String::from("b"),
                            }),
                        }]),
                    }),
                }),
            },
            Expression::Call {
                func: Box::new(Expression::Var {
                    value: String::from("print"),
                }),
                args: Box::new(vec![Expression::Call {
                    func: Box::new(Expression::Var {
                        value: String::from("sum"),
                    }),
                    args: Box::new(vec![
                        Expression::Num { value: 1 },
                        Expression::Num { value: 2 },
                    ]),
                }]),
            },
        ]),
    };
    assert_eq!(result, expected);
}

// NOTE: This is because the grammar needs to treat binary expressions as value expressions
#[test]
fn test_multiple_including_combo_assign_and_binary() {
    let parser = grammar::ProgramParser::new();
    let input = r#"
x = 2;
y = 4;
z = x + y;
print(z);
"#;
    let result = parser.parse(input).unwrap();
    let expected = Expression::Program {
        lines: Box::new(vec![
            Expression::Assign {
                operator: AssignOperator::Equals,
                left: Box::new(Expression::Var {
                    value: String::from("x"),
                }),
                right: Box::new(Expression::Num { value: 2 }),
            },
            Expression::Assign {
                operator: AssignOperator::Equals,
                left: Box::new(Expression::Var {
                    value: String::from("y"),
                }),
                right: Box::new(Expression::Num { value: 4 }),
            },
            Expression::Assign {
                operator: AssignOperator::Equals,
                left: Box::new(Expression::Var {
                    value: String::from("z"),
                }),
                right: Box::new(Expression::Binary {
                    operator: BinaryOperator::Plus,
                    left: Box::new(Expression::Var {
                        value: String::from("x"),
                    }),
                    right: Box::new(Expression::Var {
                        value: String::from("y"),
                    }),
                }),
            },
            Expression::Call {
                func: Box::new(Expression::Var {
                    value: String::from("print"),
                }),
                args: Box::new(vec![
                    Expression::Var { value: String::from("z")}
                ]),
            },
        ]),
    };
    assert_eq!(result, expected);
}
