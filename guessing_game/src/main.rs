use std::io; //Musze skorzystac z biblioteki standard input io
use rand::Rng; // Do korzystania z generatorow liczb losowych
use std::cmp::Ordering; //Do robienia "switcha"- tutaj np porownywania liczb, nie trzeba leciec po ifie, tylko takim matchem


fn main() {
    
    let mut guess;
    let mut rng=rand::thread_rng();
    let random_number=rng.gen_range(0..=100);
    let mut answered:bool=false;
    let mut number_of_guesses:u32=0;
  
    
    println!("Guess the secret number!");
    
    
    
    while  answered!=true{

        guess=guess_the_number();
        number_of_guesses+=1;
        //let guess:i32=guess.trim().parse().expect("This is not a number"); //Jak parsuje to musze od razu sam okreslic typ, zeby bylo wiadomo do czego parsuje
        let guess:i32=match guess.trim().parse() {
            Ok(number) => number,
            Err(_)=> continue,
        };
        //Tutaj od razu zrobilem error handling - wykorzystalem match do rozpakowania wyniku guess.trim().parse() - jezeli jest ok to zwracam wartosc, jezeli jest error to wylapuje wszystko(_ - to zeby wszystko wylapac) i ide dalej w petli

        println!("You guess {guess}");
        


        match  guess.cmp(&random_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => 
            {
                println!("You've guessed it!");
                println!("Number of guesses  = {}", number_of_guesses);
                answered=true;
            },
            }
    }
    
}



fn guess_the_number() -> String{
    let mut guess=String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line"); // Tutaj korzystajac z biblioteki stdin odczytuje input i przypisuje go do
    // do guess'a, a w razie bledu wypisuje blad

    return guess;
}