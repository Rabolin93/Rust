

//Collatz
#![allow(non_snake_case)] // to wylacza snake case czyli wymuszanie pisowni duzych liter. jak dodam wykrzyknik do hasha to dotyczy to calego programu


fn collatz( n:u64){
    let mut n:u64=n; // przyslaniamy nasza niemutowalna zmienna nowa mutowalna o tej samej wartosci.
    print!("{n} ");
    while n>1 {
        if n%2==0{
            n=n/2;
        } else{
            n=n*3+1;
        }
        print!("{n} ")
    } 
    println!();
}

fn main() {

    //let N  =std::env::args().nth(1).unwrap().parse::<u64>().unwrap() //tak odczytuje parametry z wiersza polecen. TUtaj mowi - podaj mi pierwszy argument (zerowy to nazwa funkcji)
    //unwrap powoduje ze dostaje stringa- wypakowuje z options to co tam jest zakopane
    //<u64> wymusza zmiane typu resultatu funkcji na u64. Mowie mu na co parser ma parsowac

    //Zamiast unwrap mozna uzyc expect, nie jako dopiske

    let N  =std::env::args()
    .nth(1)
    .unwrap_or_else(|| {
        eprintln!("Uwaga! Podaj argument liczbowy, Np. 10");
        std::process::exit(1);
    })
    .parse::<u64>()
    .unwrap_or_else(|e| {
        eprintln!("Uwaga! Podano nieprawidlowa wartosc. Musi byc to liczba naturalna. Np. 10.");
        eprintln!("Blad: '{e}'");
        std::process::exit(1);
    });

    for n in 1..=N{
      collatz(n);
    }
}
