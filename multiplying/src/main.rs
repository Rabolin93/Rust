


fn main() {


    let arg1  =std::env::args()
    .nth(1)
    .unwrap_or_else(|| {
        eprintln!("Uwaga! Podaj argument liczbowy z zakresu u64");
        std::process::exit(1);
    })
    .parse::<u64>()
    .unwrap_or_else(|e| {
        eprintln!("Uwaga! Podano nieprawidlowa wartosc. Musi byc to liczba naturalna wielkosci w zakresie u64.");
        eprintln!("Blad: '{e}'");
        std::process::exit(1);
    });

    let arg2  =std::env::args()
    .nth(2)
    .unwrap_or_else(|| {
        eprintln!("Uwaga! Podaj argument liczbowy z zakresu u64");
        std::process::exit(1);
    })
    .parse::<u64>()
    .unwrap_or_else(|e| {
        eprintln!("Uwaga! Podano nieprawidlowa wartosc. Musi byc to liczba naturalna wielkosci w zakresie u64.");
        eprintln!("Blad: '{e}'");
        std::process::exit(1);
    });
    let lower;
    let upper;

    if arg1<arg2{
        lower = arg1;
        upper = arg2;
    }
    else{
        lower = arg2;
        upper = arg1
    }

 

    


    create_table(lower, upper);

}


// fn create_table(lower:u64, upper:u64,length:u64){
//     let length=upper-lower+1;
//     let length:usize=length as usize;
//     let mut result_string=String::new();
//     // let mut small_table:[u64;length];
//     // let mut big_table:[[u64;length];length];

//     print!(" ");
//     for el in lower..=upper{
//         print!("  {el}");
//     }
//     println!();
//     for i in lower..=upper{
//         let i_to_string:String=i.to_string();
//         result_string.push_str(&i_to_string);
//         result_string=result_string+" ";
//         for j in lower..=upper{
//             let result=i*j;
//             let result_to_string:String=result.to_string();
//             result_string=result_string+" ";
//             result_string.push_str(&result_to_string);
//         }
//         result_string=result_string+"\n";
        
//     }
//     print!("{}",result_string);

// }


fn create_table(lower:u64, upper:u64)->Vec<Vec<u64>>{
    let length=upper-lower+1;
    let length:usize=length as usize;
    let last_el:u64=upper*upper;
    let last_el_to_string=last_el.to_string();
    let width=last_el_to_string.len();
    println!("{}",last_el);
    println!("{}",width);
    
    let mut small_vec=vec![0;length];
    let mut big_vec = vec![vec![0; length]; length];
    small_vec.clear();
    big_vec.clear();
    small_vec.push(0);
    for e in lower..=upper{
        small_vec.push(e);
    }
    big_vec.push(small_vec.clone());
    small_vec.clear();
    println!();
    for i in lower..=upper{
        small_vec.push(i);
        for j in lower..=upper{
            let result=i*j;
            small_vec.push(result)
        }
        big_vec.push(small_vec.clone());

        small_vec.clear();
       
        
    }

    

    for element in &big_vec{
        for item in element{
            print!("{:width$}", item, width=width+3);
        }
        println!();
    }
    return big_vec;

}
