use std::io;

fn convert_to_int(data_input: &String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x 
}

fn main() {
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Erro ao ler number1");
    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Erro ao ler number1");

    if convert_to_int(&number1) > convert_to_int(&number2){
        println!("O numero {} é maior que o numero {}", number1, number2);
    }else if convert_to_int(&number1) == convert_to_int(&number2){
        println!("O numero {} é exatamente igual ao numero {}", number1, number2);
    }
    else{
        println!("O numero {} é menor o numero {}", number1, number2);
    }
}
