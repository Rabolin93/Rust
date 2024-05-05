
fn main() {

    //Tuple i sposoby ich rozpakowania - odnoszenie sie przez kropke, nie index
    let test_tuple=(64,"Napis");
    let (x,y)=test_tuple;
    println!("{}",test_tuple.0);

    //stringi i string literal
    let test_string= String::from("This is a test string"); // to jest zwykla zmienna typu string, zmienna zlozona przezchowywana
    // na kopcu (heap)
    let mut test_string_2= String::from("This is a test strinmg number2");
    let mut test_string_3= String::from("This is a test strinmg number3");
    let test_string_literal="This is a test string literal"; //a to jest odnosnik do stringa tylko, nie caly string. String literal
    
    test_function(&test_string); 
    test_function(test_string_literal); 
    test_function_destroys_string(test_string);
   //  println!("{test_string}");- tego nie wykonamy, wywali blad - string test_string
   //zostal przekazany do funkcji testfunctiondestroysString i nie zostal zwrocony - tzn ze skoro nalezal do funkcji a funkcja umarl
   // to on umarl razem z ta funkcja. nie ma go. Inaczej jakby to byl string literal czyli tylko odnosnik - odnosnik mozna sobie przekazywac dowoli
   //Ale wtedy funkcja musi byc inaczej zrobiona.


   //Tutaj jest problem z pozyczaniem - czyli w sumie robieniem odniesien - odniesien zwyklych niemutowalnych moge sobie robic ile
   //tylko sobie wymysle. Ale odwolanie mutowalne tylko jedno - inaczej wywali blad ale dopiero przy probie wykorzystania tego, nie wczesniej.

   let ptrA=&test_string_2;
   let ptrB=&test_string_2; // moge sobie  odwolan robic ile chce zwyklych
   //let ptrC=&mut test_string_2;// - to jest odwolanie mutowalne, czyli teoretycznie powinienem byc cos w stanie z tym zrobic
   
   
   println!{"Testing the tests {ptrA}, {ptrB},"}
   println!("{ptrA}");

    let ptrC=&mut test_string_3;
    ptrC.push('Y');//Tutaj jest dowod na to ze moge sobie poprzez odwolanie zmieniac tego stringa do ktorego sie odnosze-dodalem sobie na koniec Y
    println!("{test_string_3}");






//STRUCT

    //Tak sie tworzy structa
    let user = User{
        is_active:true,
        name:String::from("bolek"),
        email:String::from("lolek@onet.pl"),
        just_a_number:5,
    };

    let jacek= build_struct(String::from("Jacek"), String::from("placek"));
    println!("{}",jacek.email);

    let user2 = User{
        name:String::from("Pipkos"),
        ..user
    };

    //println!("{}",user.email); - jak tworze nowy struct i korzystam z ..user to ja przenosze wszystkie bardziej
    //zlozone typy danych, np stringi z tego pierwszego structa- tam wiec zostaje figa z makiem i jest problem

}
fn test_function(string:&str)->&str {
 //tak jest najlepiej zapisywac funkcje ktore cos robia na stringu i zwracaja cos stringowego - dzieki temu moge
 //tam przekazac zarowno calego stringa( a raczej odnosnik do tego stringa)
 //albo po prostu string literal czyli w zasadzie odnosnik do stringa
    let slice= &string[0..=5];
    println!("{}",slice);
    return slice;

}

fn test_function_destroys_string(string:String){
    println!("This functions takes this string - {} - and destroys it, so its no longer of any use ",string);
}


fn build_struct(name: String, email: String)->User{
    User {
        is_active:true,
        name,
        email,
        just_a_number:1
    }
}

//Tak sie okresla skrukture structa. Wszystko mozna tam sobie wcisnac do woli, rozne typy danych
struct User{
    is_active:bool,
    name:String,
    email:String,
    just_a_number:u32,
}