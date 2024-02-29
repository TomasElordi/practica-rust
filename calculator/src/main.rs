use cursive::views::{Dialog, EditView};
use cursive::Cursive;
use std::str::FromStr;
use cursive::CursiveExt;
use cursive::view::Nameable;
use cursive::view::Resizable;

fn main() {
    let mut app = Cursive::default();

    app.add_layer(
        Dialog::new()
            .title("Calculadora")
            .content(
                EditView::new()
                    .on_submit(|s, input| calculate(s, input))
                    .with_name("input")
                    .fixed_width(30),
            )
            .button("Calcular", |s| {
                let input = s
                    .call_on_name("input", |view: &mut EditView| view.get_content())
                    .unwrap();
                calculate(s, &input);
            })
            .button("Salir", |s| s.quit()),
    );

    app.run();
}
fn calculate(s: &mut Cursive, input: &str) {
    match evaluate_expression(input) {
        Ok(result) => {
            s.add_layer(Dialog::info(format!("Resultado: {}", result)));
        }
        Err(err) => {
            s.add_layer(Dialog::info(format!("Error: {}", err)));
        }
    }
}

fn evaluate_expression(expression: &str) -> Result<f64, String> {
    // Parsear la expresión y realizar el cálculo
    let expr_parts: Vec<&str> = expression.split_whitespace().collect();
    if expr_parts.len() != 3 {
        return Err("Formato incorrecto. Debe ser 'número operador número'".to_string());
    }

    let num1 = f64::from_str(expr_parts[0]);
    let operator = expr_parts[1];
    let num2 = f64::from_str(expr_parts[2]);

    match (num1, operator, num2) {
        (Ok(num1), "+", Ok(num2)) => Ok(num1 + num2),
        (Ok(num1), "-", Ok(num2)) => Ok(num1 - num2),
        (Ok(num1), "*", Ok(num2)) => Ok(num1 * num2),
        (Ok(num1), "/", Ok(num2)) => {
            if num2 == 0.0 {
                Err("Error: No se puede dividir por cero.".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Operador no válido. Los operadores válidos son +, -, *, /".to_string()),
    }
}
