use std::any::Any;

fn main() {
    let ip4= IpAddressType::IPV4(100,100,100,100); // tak sie powoluje odpowiedni typ enumeratora
    let ip6=IpAddressType::IPV6(String::from("::1"));
    let m= Message::Write(String::from("A test message for the enum"));
    m.call();
    let m= Message::Quit;
    m.call(); 
}

//Ale mozna wiecej z enumami robic w rust
enum IpType{
    IPV4,
    IPV6
}

enum IpAddressType{
    IPV4(u8,u8,u8,u8), // moge zrobic sobie taki "konstruktor" -i przypisuje do typu 4 wartosci 4x u8, a do 6 stringa- czyli moge do roznych enumow rozne typy i ilosci tych typow
    IPV6(String) // moge nawet structa tutaj wsadzic, cokolwiek
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn call(&self){
        if matches!(self,Message::Quit){
            println!{"Testing enum methods"};
        }
        
    }
}