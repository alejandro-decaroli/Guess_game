use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let mut jugar: String = "s".to_string();
    
    while jugar != "n" {

        let mut vidas: u8 = 7;
        
        println!("\nBienvenido a adivina el numero\n");
    
        let numero_secreto: u8 = rand::rng().random_range(1..=100);
    
        while vidas > 0 {
            
            println!("Por favor, ingresa un numero: ");
            
            let mut numero_1: String = String::new();
            
            io::stdin() 
                .read_line(&mut numero_1)
                .expect("No se pudo leer el numero");
        
            let numero_1: u8 = numero_1.trim().parse().expect("Ingresa el número: ");

            match numero_1.cmp(&numero_secreto) {
                Ordering::Less => {println!("El número es mayor");
                    vidas = vidas - 1;
                },
                Ordering::Greater => {println!("El número es menor");
                    vidas = vidas - 1;
                },
                Ordering::Equal => {println!("Ganaste!");
                    break;
                },
            }
        
            
            if vidas == 0 {
                println!("\n¡Game Over! Has perdido.\n");
                println!("El numero secreto era: {}", numero_secreto);
                break;
            }

            println!("Te quedan {} vidas", vidas);
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
    


