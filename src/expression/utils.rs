use super::Expr;

pub fn parenthisize(name: &str, expresions: &[&Expr]) -> String {
    let mut result = String::new();

    result += &("(".to_string() + name);

    for expr in expresions {
        result = format!("{} {}", result, expr);
    }

    result.push(')');

    result
}
