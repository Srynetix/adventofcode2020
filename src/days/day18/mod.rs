//! # Day 18: Operator Order
//!
//! As you look out the window and notice a heavily-forested continent slowly appear over the horizon, you are interrupted by the child sitting next to you. They're curious if you could help them with their math homework.
//!
//! Unfortunately, it seems like this "math" follows different rules than you remember.
//!
//! The homework (your puzzle input) consists of a series of expressions that consist of addition (+), multiplication (*), and parentheses ((...)). Just like normal math, parentheses indicate that the expression inside must be evaluated before it can be used by the surrounding expression. Addition still finds the sum of the numbers on both sides of the operator, and multiplication still finds the product.
//!
//! However, the rules of operator precedence have changed. Rather than evaluating multiplication before addition, the operators have the same precedence, and are evaluated left-to-right regardless of the order in which they appear.
//!
//! For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are as follows:
//!
//! ```text
//! 1 + 2 * 3 + 4 * 5 + 6
//!   3   * 3 + 4 * 5 + 6
//!       9   + 4 * 5 + 6
//!          13   * 5 + 6
//!              65   + 6
//!                  71
//! ```
//!
//! Parentheses can override this order; for example, here is what happens if parentheses are added to form 1 + (2 * 3) + (4 * (5 + 6)):
//!
//! ```text
//! 1 + (2 * 3) + (4 * (5 + 6))
//! 1 +    6    + (4 * (5 + 6))
//!      7      + (4 * (5 + 6))
//!      7      + (4 *   11   )
//!      7      +     44
//!             51
//! ```
//!
//! Here are a few more examples:
//!
//! ```text
//! 2 * 3 + (4 * 5) becomes 26.
//! 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
//! 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
//! ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
//! ```
//!
//! Before you can help with the homework, you need to understand it yourself. Evaluate the expression on each line of the homework; what is the sum of the resulting values?
//!
//! # Part Two
//!
//! You manage to answer the child's questions and they finish part 1 of their homework, but get stuck when they reach the next section: advanced math.
//!
//! Now, addition and multiplication have different precedence levels, but they're not the ones you're familiar with. Instead, addition is evaluated before multiplication.
//!
//! For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are now as follows:
//!
//! ```text
//! 1 + 2 * 3 + 4 * 5 + 6
//!   3   * 3 + 4 * 5 + 6
//!   3   *   7   * 5 + 6
//!   3   *   7   *  11
//!      21       *  11
//!          231
//! ```
//!
//! Here are the other examples from above:
//!
//! ```text
//! 1 + (2 * 3) + (4 * (5 + 6)) still becomes 51.
//! 2 * 3 + (4 * 5) becomes 46.
//! 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 1445.
//! 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 669060.
//! ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 23340.
//! ```
//!
//! What do you get if you add up the results of evaluating the homework problems using these new rules?

use std::collections::HashMap;

use thiserror::Error;

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
#[allow(clippy::cast_sign_loss)]
pub fn run_ex1() -> usize {
    INPUT_VALUES
        .trim()
        .lines()
        .map(|l| {
            let precedences = ExpressionParser::default_token_precedences();
            ExpressionParser::parse_and_compute_expression(l.trim(), &precedences).unwrap()
        })
        .sum::<isize>() as usize
}

/// Part two answer.
#[allow(clippy::cast_sign_loss)]
pub fn run_ex2() -> usize {
    INPUT_VALUES
        .trim()
        .lines()
        .map(|l| {
            let precedences = ExpressionParser::addition_token_precedences();
            ExpressionParser::parse_and_compute_expression(l.trim(), &precedences).unwrap()
        })
        .sum::<isize>() as usize
}

/// Day error.
#[derive(Debug, Error)]
pub enum DayError {
    /// Token parse error.
    #[error("Parse token error: {0}")]
    ParseTokenError(String),

    /// Expression parse error.
    #[error("Parse expression error: {0}")]
    ParseExpressionError(String),
}

/// Expression token.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ExpressionToken {
    /// Digit.
    Digit(u32),
    /// Operator sign.
    OperatorSign(OperatorSign),
    /// Parenthese.
    Parenthese(Parenthese),
    /// End.
    End,
}

/// Operator sign.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OperatorSign {
    /// Addition.
    Addition,
    /// Multiplication.
    Multiplication,
}

/// Parenthese.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Parenthese {
    /// Open.
    Open(usize),
    /// Close.
    Close(usize),
}

/// Expression node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExpressionNode {
    /// Number
    Number(isize),
    /// Addition
    Addition(Box<Self>, Box<Self>),
    /// Multiplication
    Multiplication(Box<Self>, Box<Self>),
}

/// Expression lexer.
pub struct ExpressionLexer;

/// Expression parser.
pub struct ExpressionParser;

/// Expression lexer context.
#[derive(Debug, Default)]
pub struct ExpressionLexerContext {
    last_parens_index: usize,
}

impl ExpressionLexerContext {
    /// Creates a new expression lexer context.
    pub const fn new() -> Self {
        Self {
            last_parens_index: 0,
        }
    }
}

impl ExpressionLexer {
    /// Parse tokens.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    ///
    /// # Errors
    ///
    /// * Parse error
    pub fn parse_tokens(input: &str) -> Result<Vec<ExpressionToken>, DayError> {
        let mut context = ExpressionLexerContext::new();
        let mut tokens = input
            .chars()
            .filter_map(|c| {
                if c == ' ' {
                    None
                } else {
                    Some(Self::parse_token(&mut context, c))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        tokens.push(ExpressionToken::End);
        Ok(tokens)
    }

    /// Parse token.
    ///
    /// # Arguments
    ///
    /// * `input` - Input char
    ///
    /// # Errors
    ///
    /// * Parse error
    pub fn parse_token(
        context: &mut ExpressionLexerContext,
        input: char,
    ) -> Result<ExpressionToken, DayError> {
        match input {
            '+' => Ok(ExpressionToken::OperatorSign(OperatorSign::Addition)),
            '*' => Ok(ExpressionToken::OperatorSign(OperatorSign::Multiplication)),
            '(' => {
                let node = ExpressionToken::Parenthese(Parenthese::Open(context.last_parens_index));
                context.last_parens_index += 1;
                Ok(node)
            }
            ')' => {
                if context.last_parens_index == 0 {
                    return Err(DayError::ParseTokenError(
                        "Unmatching closing parenthese".to_string(),
                    ));
                }

                context.last_parens_index -= 1;
                let node =
                    ExpressionToken::Parenthese(Parenthese::Close(context.last_parens_index));
                Ok(node)
            }
            other => other
                .to_digit(10)
                .map(ExpressionToken::Digit)
                .ok_or_else(|| {
                    DayError::ParseTokenError(format!("Could not parse digit: {}", other))
                }),
        }
    }
}

impl ExpressionParser {
    /// Parse and compute expression from input.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    /// * `token_precedences` - Precedence map
    ///
    /// # Errors
    ///
    /// * Empty or bad expression
    pub fn parse_and_compute_expression(
        input: &str,
        token_precedences: &HashMap<ExpressionToken, isize>,
    ) -> Result<isize, DayError> {
        let tokens = ExpressionLexer::parse_tokens(input)?;
        let tree = Self::generate_tree_from_tokens(&tokens, token_precedences)?;

        Ok(Self::resolve_expression_tree(tree))
    }

    /// Resolve expression tree to a number.
    ///
    /// # Arguments
    ///
    /// * `tree` - Expression node
    pub fn resolve_expression_tree(tree: ExpressionNode) -> isize {
        use ExpressionNode::{Addition, Multiplication, Number};

        match tree {
            Number(n) => n,
            Addition(a, b) => Self::resolve_expression_tree(*a) + Self::resolve_expression_tree(*b),
            Multiplication(a, b) => {
                Self::resolve_expression_tree(*a) * Self::resolve_expression_tree(*b)
            }
        }
    }

    /// Generate node tree from tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - Token stream
    /// * `token_precedences` - Precedence map
    ///
    /// # Errors
    ///
    /// * Expression parse error
    #[allow(clippy::cast_possible_wrap)]
    pub fn generate_tree_from_tokens(
        tokens: &[ExpressionToken],
        token_precedences: &HashMap<ExpressionToken, isize>,
    ) -> Result<ExpressionNode, DayError> {
        let mut cursor = 0;
        Self::parse_expr(tokens, &mut cursor, token_precedences)
    }

    /// Default token precedences.
    pub fn default_token_precedences() -> HashMap<ExpressionToken, isize> {
        maplit::hashmap! {
            ExpressionToken::OperatorSign(OperatorSign::Addition) => 1,
            ExpressionToken::OperatorSign(OperatorSign::Multiplication) => 1,
        }
    }

    /// Token precedences with addition priority.
    fn addition_token_precedences() -> HashMap<ExpressionToken, isize> {
        maplit::hashmap! {
            ExpressionToken::OperatorSign(OperatorSign::Addition) => 2,
            ExpressionToken::OperatorSign(OperatorSign::Multiplication) => 1,
        }
    }

    fn parse_expr(
        tokens: &[ExpressionToken],
        cursor: &mut usize,
        token_precedences: &HashMap<ExpressionToken, isize>,
    ) -> Result<ExpressionNode, DayError> {
        let lhs = Self::parse_lhs(tokens, cursor, token_precedences)?;
        Self::parse_rhs(tokens, cursor, lhs, 0, token_precedences)
    }

    fn parse_lhs(
        tokens: &[ExpressionToken],
        cursor: &mut usize,
        token_precedences: &HashMap<ExpressionToken, isize>,
    ) -> Result<ExpressionNode, DayError> {
        match Self::peek_token(tokens, cursor)? {
            ExpressionToken::Parenthese(Parenthese::Open(_)) => {
                Self::parse_parens_expr(tokens, cursor, token_precedences)
            }
            ExpressionToken::Digit(_) => Self::parse_digit_expr(tokens, cursor),
            other => Err(DayError::ParseExpressionError(format!(
                "Unsupported lhs: {:?}",
                other
            ))),
        }
    }

    fn peek_token<'a>(
        tokens: &'a [ExpressionToken],
        cursor: &mut usize,
    ) -> Result<&'a ExpressionToken, DayError> {
        tokens
            .get(*cursor)
            .ok_or_else(|| DayError::ParseExpressionError("No more tokens.".to_string()))
    }

    fn consume_token<'a>(
        tokens: &'a [ExpressionToken],
        cursor: &mut usize,
    ) -> Option<&'a ExpressionToken> {
        let token = tokens.get(*cursor);
        *cursor += 1;
        token
    }

    fn parse_rhs(
        tokens: &[ExpressionToken],
        cursor: &mut usize,
        lhs: ExpressionNode,
        precedence: isize,
        token_precedences: &HashMap<ExpressionToken, isize>,
    ) -> Result<ExpressionNode, DayError> {
        let mut curr_lhs = lhs;

        loop {
            let token = Self::peek_token(tokens, cursor)?;
            let token_precedence = token_precedences.get(token).copied().unwrap_or(-1);
            if token_precedence < precedence {
                return Ok(curr_lhs);
            }

            Self::consume_token(tokens, cursor);
            let mut curr_rhs = Self::parse_lhs(tokens, cursor, token_precedences)?;
            let next_token = Self::peek_token(tokens, cursor)?;
            let next_prec = token_precedences.get(next_token).copied().unwrap_or(-1);
            if token_precedence < next_prec {
                curr_rhs =
                    Self::parse_rhs(tokens, cursor, curr_rhs, precedence + 1, token_precedences)?;
            }

            curr_lhs = Self::parse_operation(token, curr_lhs, curr_rhs)?;
        }
    }

    fn parse_operation(
        token: &ExpressionToken,
        lhs: ExpressionNode,
        rhs: ExpressionNode,
    ) -> Result<ExpressionNode, DayError> {
        match token {
            ExpressionToken::OperatorSign(OperatorSign::Addition) => {
                Ok(ExpressionNode::Addition(Box::new(lhs), Box::new(rhs)))
            }
            ExpressionToken::OperatorSign(OperatorSign::Multiplication) => {
                Ok(ExpressionNode::Multiplication(Box::new(lhs), Box::new(rhs)))
            }
            other => Err(DayError::ParseExpressionError(format!(
                "Unsupported token in operation: {:?}",
                other
            ))),
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    fn parse_digit_expr(
        tokens: &[ExpressionToken],
        cursor: &mut usize,
    ) -> Result<ExpressionNode, DayError> {
        match Self::peek_token(tokens, cursor)? {
            ExpressionToken::Digit(d) => {
                Self::consume_token(tokens, cursor);
                Ok(ExpressionNode::Number(*d as isize))
            }
            other => Err(DayError::ParseExpressionError(format!(
                "Unsupported digit expr: {:?}",
                other
            ))),
        }
    }

    fn parse_parens_expr(
        tokens: &[ExpressionToken],
        cursor: &mut usize,
        token_precedences: &HashMap<ExpressionToken, isize>,
    ) -> Result<ExpressionNode, DayError> {
        Self::consume_token(tokens, cursor);
        let expr = Self::parse_expr(tokens, cursor, token_precedences)?;
        match Self::peek_token(tokens, cursor)? {
            ExpressionToken::Parenthese(Parenthese::Close(_)) => {
                Self::consume_token(tokens, cursor);
                Ok(expr)
            }
            other => Err(DayError::ParseExpressionError(format!(
                "Bad token instead of close parens: {:?}",
                other
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 45_283_905_029_161;
    const EX2_OUTPUT: usize = 216_975_281_211_165;

    #[test]
    fn test_parse_token() {
        let mut context = ExpressionLexerContext::new();

        assert_eq!(
            ExpressionLexer::parse_token(&mut context, '+').unwrap(),
            ExpressionToken::OperatorSign(OperatorSign::Addition)
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, '*').unwrap(),
            ExpressionToken::OperatorSign(OperatorSign::Multiplication)
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, '1').unwrap(),
            ExpressionToken::Digit(1)
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, '(').unwrap(),
            ExpressionToken::Parenthese(Parenthese::Open(0))
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, '(').unwrap(),
            ExpressionToken::Parenthese(Parenthese::Open(1))
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, ')').unwrap(),
            ExpressionToken::Parenthese(Parenthese::Close(1))
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, ')').unwrap(),
            ExpressionToken::Parenthese(Parenthese::Close(0))
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, '(').unwrap(),
            ExpressionToken::Parenthese(Parenthese::Open(0))
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, ')').unwrap(),
            ExpressionToken::Parenthese(Parenthese::Close(0))
        );

        assert!(ExpressionLexer::parse_token(&mut context, ')').is_err());
        assert!(ExpressionLexer::parse_token(&mut context, 'a').is_err());
    }

    #[test]
    fn test_parse_tokens() {
        assert_eq!(
            ExpressionLexer::parse_tokens("1 + 2 * 3 + 4 * 5 + 6").unwrap(),
            vec![
                ExpressionToken::Digit(1),
                ExpressionToken::OperatorSign(OperatorSign::Addition),
                ExpressionToken::Digit(2),
                ExpressionToken::OperatorSign(OperatorSign::Multiplication),
                ExpressionToken::Digit(3),
                ExpressionToken::OperatorSign(OperatorSign::Addition),
                ExpressionToken::Digit(4),
                ExpressionToken::OperatorSign(OperatorSign::Multiplication),
                ExpressionToken::Digit(5),
                ExpressionToken::OperatorSign(OperatorSign::Addition),
                ExpressionToken::Digit(6),
                ExpressionToken::End
            ]
        );

        assert_eq!(
            ExpressionLexer::parse_tokens("1 + (2 * 3) + (4 * (5 + 6))").unwrap(),
            vec![
                ExpressionToken::Digit(1),
                ExpressionToken::OperatorSign(OperatorSign::Addition),
                ExpressionToken::Parenthese(Parenthese::Open(0)),
                ExpressionToken::Digit(2),
                ExpressionToken::OperatorSign(OperatorSign::Multiplication),
                ExpressionToken::Digit(3),
                ExpressionToken::Parenthese(Parenthese::Close(0)),
                ExpressionToken::OperatorSign(OperatorSign::Addition),
                ExpressionToken::Parenthese(Parenthese::Open(0)),
                ExpressionToken::Digit(4),
                ExpressionToken::OperatorSign(OperatorSign::Multiplication),
                ExpressionToken::Parenthese(Parenthese::Open(1)),
                ExpressionToken::Digit(5),
                ExpressionToken::OperatorSign(OperatorSign::Addition),
                ExpressionToken::Digit(6),
                ExpressionToken::Parenthese(Parenthese::Close(1)),
                ExpressionToken::Parenthese(Parenthese::Close(0)),
                ExpressionToken::End
            ]
        )
    }

    #[test]
    fn test_generate_tree_from_tokens_no_parens() {
        use ExpressionNode::{Addition, Multiplication, Number};

        let first_sample = ExpressionLexer::parse_tokens("1 + 2 * 3 + 4 * 5 + 6").unwrap();
        let token_precedences = ExpressionParser::default_token_precedences();

        assert_eq!(
            ExpressionParser::generate_tree_from_tokens(&first_sample, &token_precedences).unwrap(),
            Addition(
                Box::new(Multiplication(
                    Box::new(Addition(
                        Box::new(Multiplication(
                            Box::new(Addition(Box::new(Number(1)), Box::new(Number(2)))),
                            Box::new(Number(3))
                        )),
                        Box::new(Number(4))
                    )),
                    Box::new(Number(5))
                )),
                Box::new(Number(6))
            )
        );
    }

    #[test]
    fn test_generate_tree_from_tokens_with_parens() {
        use ExpressionNode::{Addition, Multiplication, Number};

        let second_sample = ExpressionLexer::parse_tokens("1 + (2 * 3) + (4 * (5 + 6))").unwrap();
        let token_precedences = ExpressionParser::default_token_precedences();

        assert_eq!(
            ExpressionParser::generate_tree_from_tokens(&second_sample, &token_precedences)
                .unwrap(),
            Addition(
                Box::new(Addition(
                    Box::new(Number(1)),
                    Box::new(Multiplication(Box::new(Number(2)), Box::new(Number(3)),))
                )),
                Box::new(Multiplication(
                    Box::new(Number(4)),
                    Box::new(Addition(Box::new(Number(5)), Box::new(Number(6))))
                ))
            )
        );
    }

    #[test]
    fn test_parse_and_compute_expression() {
        let precedences = ExpressionParser::default_token_precedences();

        assert_eq!(
            ExpressionParser::parse_and_compute_expression("1 + 2 * 3 + 4 * 5 + 6", &precedences)
                .unwrap(),
            71
        );
        assert_eq!(
            ExpressionParser::parse_and_compute_expression(
                "1 + (2 * 3) + (4 * (5 + 6))",
                &precedences
            )
            .unwrap(),
            51
        );
        assert_eq!(
            ExpressionParser::parse_and_compute_expression("2 * 3 + (4 * 5)", &precedences)
                .unwrap(),
            26
        );
        assert_eq!(
            ExpressionParser::parse_and_compute_expression(
                "5 + (8 * 3 + 9 + 3 * 4 * 3)",
                &precedences
            )
            .unwrap(),
            437
        );
        assert_eq!(
            ExpressionParser::parse_and_compute_expression(
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
                &precedences
            )
            .unwrap(),
            12240
        );
        assert_eq!(
            ExpressionParser::parse_and_compute_expression(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                &precedences
            )
            .unwrap(),
            13632
        );
    }

    #[test]
    fn test_generate_tree_from_tokens_with_another_precedences() {
        use ExpressionNode::{Addition, Multiplication, Number};

        let first_sample = ExpressionLexer::parse_tokens("1 + 2 * 3 + 4 * 5 + 6").unwrap();
        let token_precedences = ExpressionParser::addition_token_precedences();

        assert_eq!(
            ExpressionParser::generate_tree_from_tokens(&first_sample, &token_precedences).unwrap(),
            Multiplication(
                Box::new(Addition(Box::new(Number(1)), Box::new(Number(2)),)),
                Box::new(Multiplication(
                    Box::new(Addition(Box::new(Number(3)), Box::new(Number(4)),)),
                    Box::new(Addition(Box::new(Number(5)), Box::new(Number(6)),))
                ))
            )
        );
    }

    #[test]
    fn test_run_ex1() {
        assert_eq!(run_ex1(), EX1_OUTPUT);
    }

    #[test]
    fn test_run_ex2() {
        assert_eq!(run_ex2(), EX2_OUTPUT);
    }
}
