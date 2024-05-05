pub mod point;
use crate::point::Point;

fn main() {


    let p= Point::new(3,4);
    let q = Point::new(1,2);
    let r = p+q;
    println!("{}, {}, Sum={}",p,q,r);
}
