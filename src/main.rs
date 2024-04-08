fn fahrenheit_to_celsius(fahrenheit: String) -> f64 {
  let fahrenheit: f64 = fahrenheit.trim().parse().expect("Deve ser um número");
  let celsius = (fahrenheit-32 as f64)/(1.8);
  celsius
}

fn celsius_to_fahrenheit(celsius: String) -> f64 {
  let celsius: f64 = celsius.trim().parse().expect("Deve ser um número");
  let fahrenheit = 1.8*celsius+32 as f64;
  fahrenheit
}

fn main() {
  println!("Digite um número:");
  let mut num = String::new();
  std::io::stdin().read_line(&mut num).expect("nao deu pra ler num");
  println!("Aqui ó: {}", celsius_to_fahrenheit(num));
}