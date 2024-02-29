use regex::Regex;
use std::io;

// Función para leer la entrada del usuario
fn obtener_entrada() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    input
}

// Función principal para ejecutar la calculadora
fn main() {
    println!("Ingrese una operación (por ejemplo, 5 + 4):");

    let input = obtener_entrada();

    // Definir la expresión regular para buscar un número seguido de un operador y otro número
    let re = match Regex::new(r"(\d+)\s*([-+*/])\s*(\d+)") {
        Ok(re) => re,
        Err(err) => {
            println!("Error al compilar la expresión regular: {}", err);
            return;
        }
    };

    // Buscar la coincidencia en la entrada del usuario
    if let Some(captures) = re.captures(&input) {
        // Obtener los números y el operador de las capturas
        let num1: i32 = captures[1].parse().unwrap();
        let operador = &captures[2];
        let num2: i32 = captures[3].parse().unwrap();

        // Realizar la operación correspondiente
        let resultado = match operador {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0 {
                    println!("Error: No se puede dividir por cero.");
                    return;
                }
                num1 / num2
            }
            _ => {
                println!("Operador no válido. Los operadores válidos son +, -, *, /");
                return;
            }
        };

        println!("El resultado es: {}", resultado);
    } else {
        println!("Formato de entrada incorrecto. Debe ser 'número operador número'.");
    }
}
