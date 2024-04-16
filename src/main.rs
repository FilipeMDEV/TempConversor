fn get_number_chose() -> i8 {
  let mut option = String::new();
  std::io::stdin().read_line(&mut option).expect("Falha ao ler option");
  let option: i8 = option.trim().parse().expect("Deve ser um número");
  match option {
    1 | 2 => option,
    _ => {
      println!("Deve ser 1 ou 2");
      get_number_chose()
    }
  }
}

fn get_temperature_chose() -> f64 {
  let mut temperature = String::new();
  std::io::stdin().read_line(&mut temperature).expect("Falha ao ler");
  let temperature: f64 = temperature.trim().parse().expect("Deve ser um número");
  temperature
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
  let celsius = (fahrenheit-32 as f64)/(1.8);
  celsius
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
  let fahrenheit = 1.8*celsius+32 as f64;
  fahrenheit
}

fn main() {
  println!("============CONVERSOR DE TEMPERATURA============\nSelecione uma das opções abaixo:\n1. Converter Fahrenheit para Celsius\n2. Converter Celsius para Fahrenheit");
  let option = get_number_chose();

  if option == 1 {
    println!("Digite a temperatura em fahrenheit:");
    let fahrenheit = get_temperature_chose();
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("A temperatura em celsius é {}ºC", celsius);
  } else if option == 2 {
    println!("Digite a temperatura em celsius:");
    let celsius = get_temperature_chose();
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("A temperatura em fahrenheit é {}ºF", fahrenheit);
  }
}