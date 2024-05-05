use std::io;




fn main() {

    let mut choice=String::new();
    let mut temperature=String::new();



    println!("Select an option from the menu");
    println!("1) Celsius to Farenheit");
    println!("2) Farenheit to Celsius");
    io::stdin().read_line(&mut choice).expect("Wrong");
    let choice:u32=choice.trim().parse().expect("Wrong Parsing");
    match choice{
        1 => {
            println!("Enter the temperature");
            io::stdin().read_line(&mut temperature).expect("Wrong temperature");
            let temperature:f64=temperature.trim().parse().expect("Temp parsing errors");
            let result =celsius_to_farenheit(temperature);
            println!("Temp in farenheit ={} ",result);
        },
        2 => {
            println!("Enter the temperature");
            io::stdin().read_line(&mut temperature).expect("Wrong temperature");
            let temperature:f64=temperature.trim().parse().expect("Temp parsing errors");
            let result =farenheit_to_celsius(temperature);
            println!("Temp in celsius ={} ",result);
        },
        _ => {
            println!("Wrong choice entered");
        }

        
    }


    
}


fn celsius_to_farenheit(temp:f64) -> f64{
    let temp= temp*1.8 +32.0;
    temp
}

fn farenheit_to_celsius(temp:f64) -> f64{
    let temp=(temp-32.0) / 1.8;
    temp
}