fn main() {
    print!("0\n1\n");
    fibonacci(10,1,0);


    


}


fn fibonacci(number:i32,prev1:i32,prev2:i32){

    if number<3{
        return ()
    }
    let fibonacci_number=prev1+prev2;
    let prev2=prev1;
    let prev1=fibonacci_number;
    println!("{}",prev1);
    fibonacci(number-1,prev1,prev2);
    
}