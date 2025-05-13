use std::io;
use rand::Rng;

fn main() {

    let mut jugar: String = "s".to_string();
    
    while jugar != "n" {

        let mut vidas: u8 = 7;
        
        println!("\nBienvenido a adivina el numero\n");
    
        let numero_secreto: u8 = rand::thread_rng().gen_range(1..=100);
    
        while vidas > 0 {
            
            println!("Por favor, ingresa un numero: ");
            
            let mut numero_1: String = String::new();
            
            io::stdin() 
                .read_line(&mut numero_1)
                .expect("No se pudo leer el numero");
        
            if numero_1.trim().parse::<u8>().unwrap() == numero_secreto {
                println!("¡Correcto! Has adivinado el numero.");
                break;
            } else if numero_1.trim().parse::<u8>().unwrap() > numero_secreto {
                println!("¡Incorrecto! El numero es menor. Intentalo de nuevo.");
                vidas -= 1;
            } else {
                println!("¡Incorrecto! El numero es mayor. Intentalo de nuevo.");
                vidas -= 1;
            }
        
            println!("Te quedan {} vidas", vidas);

            if vidas == 0 {
                println!("\n¡Game Over! Has perdido.\n");
                println!("El numero secreto era: {}", numero_secreto);
                break;
            }
        }
        
        println!("\n¿Quieres jugar de nuevo?:\n");
        jugar = String::new();

        io::stdin()
            .read_line(&mut jugar)
            .expect("No se pudo leer la respuesta");

        jugar = jugar.trim().to_lowercase();

        if jugar == "n" || jugar == "no" {
            println!("\nGracias por jugar. ¡Hasta la próxima!\n");
            break;
        }
    }
}
    


