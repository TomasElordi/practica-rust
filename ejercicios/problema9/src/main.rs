use std::io;

fn main() {

    let mut total : i32 = 0 ;
    //Ingrese cantidad de hijos
    
    println!("Ingrese la cantidad de hijos: ");
    let mut cant_hijos_string = String::new();
    io::stdin().read_line(&mut cant_hijos_string).expect("Error al ingresar los hijos.");

    let mut cant_hijos : i32 = cant_hijos_string.trim().parse().expect("Debe ingresar un numero.");
    //Ingrese si es viuda
    println!("Usted es viuda? (1 si , 2 no)");
    let mut es_viuda_string = String::new();
    io::stdin().read_line(&mut es_viuda_string).expect("Error al ingresar si es viuda.");
    
    let mut es_viuda : i32 = es_viuda_string.trim().parse().expect("Debe ingresar un numero.");
    
    if es_viuda == 1 {
        total = total + 20;
    }
    //For por la cantidad de hijos
    
    
    for i in 0..cant_hijos {
        println!("Ingrese la edad de su hijo {}: ",i);
        let mut edad_string = String::new();
        io::stdin().read_line(&mut edad_string).expect("Error al ingresar la edad.");
        
        let mut edad : i32 = edad_string.trim().parse().expect("Debe ingresar un numero.");

        if edad >= 6 && edad <= 18 {
            total = total + 10;
        }
    }

    if cant_hijos <= 2 {
        total = total + 70;
    }else if cant_hijos <= 5 {
        total = total + 90;
    }else{
        total = total + 120;
    }


    println!("El gobierno debe pagarle a su familia ${}", total);

}
