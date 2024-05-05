
#[derive(Clone, Copy)] //To dodaje zeby nie zostalo zezarte przy dzialaniu. On sobie robi kopie na poziomie p+q i one nie gina. To dla prostych typow dziala
pub struct Point{
    pub x:i32,
    pub y:i32,
}

impl Point{
    pub fn new(x:i32,y:i32)->Self{
        Self{x,y}
    }
}


impl std::fmt::Display for Point{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point [{},{}]",self.x,self.y) // tutaj uzywam makro write, f bedzie moim "pisaczem"- formaterem. I mu mowie co ma napisac. nie daje srednika bo on ma to zwrocic co ja tu pisze
    }
}

impl std::ops::Add for Point{ 
    type Output = Point;//rhs -right hand side -prawa strona. Tu okreslam ze typ outputu to point
    fn add(self, rhs: Point) -> Point{
        Point{x:self.x+rhs.x, y:self.y+rhs.y}
    }

}



#[cfg(test)]
mod test{
use super::*; // importuje wszystko co powyzej w tym pliku. Test jest podmodulem pointa wiec musze pointa z gory zaimportowac tutaj

    #[test]
    fn test_new(){
        let p=Point::new(11,22);
        assert_eq!(p.x,11);
        assert_eq!(p.y,22);
    }
}