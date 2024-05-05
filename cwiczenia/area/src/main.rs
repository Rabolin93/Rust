use std::io;

fn main() {

    
    let mut input:String=String::new();

    println!("Enter the width of the rectangle!");
    io::stdin().read_line(&mut input).expect("Wrong input");
    let width:u32= input.trim().parse().expect("Parsing error");

    input.clear();

    println!("Enter the height of the rectangle!");
    io::stdin().read_line(&mut input).expect("Wrong input");
    let height:u32= input.trim().parse().expect("Parsing error");


    println!("{}",area(width,height));

    let sizes:(u32,u32)=(width,height);

    println!("{}",area_tuple(sizes));

    let rectangle = Rectangle{
        width,
        height,
    };
    let rectangle2= Rectangle{
        width:10,
        height:5,
    };
    let rectangle3=Rectangle{
        width:70,
        height:10,
    };

    println!{"Area using struct {}",area_struct(&rectangle)}

    println!("Printing a whole struct with debug mode {:?}",rectangle);
    println!("Printing a whole struct with debug mode {:#?}",rectangle);// tak rozbije na poszczegolne elementy linijki rectangla
    println!("Now area but with an implemented method {}",rectangle.area());

    println!("Rectangle 1 vs 2 {}",rectangle.can_hold(&rectangle2));
    println!("Rectangle 1 vs 3 {}",rectangle.can_hold(&rectangle3));

    println!("Create square {:#?}",Rectangle::square(10));
    //Tutaj ten konstruktur to associated function (nie metoda, bo metoda jest wywolywana na samym sobie) i do tego
    //sie odwoluje przez :: . tak samo sie odwoluje do namespace'ow tworzonych przez moduly np std::io
}







fn area(width:u32, height:u32)->u32{
    width*height
}

fn area_tuple(sizes: (u32,u32))->u32{
    sizes.0*sizes.1
}
fn area_struct(rectangle:&Rectangle)->u32{
    rectangle.width*rectangle.height
}

#[derive(Debug)] // czyli ten struct ktory tworze bedzie sobie bral ewentualne jakies metody, makra etc jak standardowo z opcja debug
struct Rectangle {
    width:u32,
    height:u32,
}

//Tak sie implemenetuje metody do danego structa. Na zewnatrz tego structa
//i Chce zeby sie odwolywal do &self a nie po prostu do self - nie chce zeby ta metoda brala wlasnosc nad structrem i nie zwracala tego
// i nie chce zeby to bylo mutowalne odwolanie, raczej nie bede zmienial wartosci w srodku structa
impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }
    fn can_hold(&self, other:&Rectangle)->bool{
        if self.width >= other.width && self.height >=other.height {
            return true;
        }
        else {
            return false;
        }
    }
    fn square(side:u32)->Self{ //To jest "konstruktor" - nie biore selfa, zwracam selfa czyli w tym przypadku rectangle. Biore side i tworze nowy rectangle
        Self{
            width:side,
            height:side,
        }
    }
}