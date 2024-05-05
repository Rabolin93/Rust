
pub struct Config{
    pub maxx:i32,
    pub maxy:i32,
}

pub struct Board{
    maxx :usize,
    maxy :usize,
    data:Vec<bool>,
}

enum Dir{
    Up,
    Dn,
    Left,
    Right,
}
pub struct Ant{
    x:usize,
    y:usize,
    dir:Dir, //0:==> 1:^^^ 2:<=== 3: vvv
}

impl Board {
    fn new(x:usize,y:usize)->Self {
        let mut v:Vec<bool>=Vec::new();
        for _i in 0..x*y{
            v.push(false);
        }
        Self { maxx: x, maxy: y, data: v.clone() }
    }
    fn set(&mut self, x:usize,y:usize, val : bool){
        let index=(y)*self.maxx+x;
        //atxy(1+x as i32,1+y as i32,"$");
        self.data[index]=val;
    }
    fn get(&self, x:usize,y:usize)->bool{
        let index=y*self.maxx+x;
        self.data[index]
    }

    fn draw (&self){
        for i in 0..self.maxx{
            for j in 0..self.maxy{
                atxy(i as i32,j as i32, if self.get(i,j){"F"} else {""});
            }
        }
    }
}


fn main() {
    let cfg=setup();
    println!("rozmiar: {} x {}",cfg.maxx,cfg.maxy);
    //atxy(0,0,&format!("rozmiar:{} x{}", cfg.maxx,cfg.maxy));
    frame(0,0,cfg.maxx-1,cfg.maxy-1);
    let mut board= Board::new(cfg.maxx as usize-2, cfg.maxy as usize-2);
    let mut ant= Ant{x:20,y:10,dir:Dir::Right};
    board.set(ant.x,ant.y,true);
    board.draw();
    atxy(cfg.maxx/2,cfg.maxy/2,"#");
    atxy(0,cfg.maxy,"");

}

fn atxy(x:i32,y:i32,text:&str){
    print!("\x1b[{};{}H{}",y+1,x+1,text);
}
// │ ─ ┓┛┏┗
fn frame(x:i32,y:i32,w:i32,h:i32){
    atxy(x,y,"┏");
    for i in 1..w{
        atxy(x+i,y,"─");
    }
    atxy(x+w,y,"┓");
    let line=" ".repeat(w as usize-1);

    for i in 1..h{
        atxy(x,y+i,&format!("│{}│",line));
    }
    atxy(x,y+h,"┗");
    for i in 1..w{atxy(x+i,y+h,"─")}
    atxy(x+w,y+h,"┛")

}

pub fn setup() -> Config{
    let rows=std::env::var("LINES");
    let cols= std::env::var("COLUMNS");
    let mut r= rows.unwrap_or("25".to_string()).parse().unwrap_or(25);
    let mut c=cols.unwrap_or("80".to_string()).parse().unwrap_or(80);
    //let mut args=std::env::args();
    // c=args.nth(1).unwrap_or(c.to_string()).parse().unwrap_or(c); //sposob na odczyt z argumentow
    // r=args.nth(0).unwrap_or(r.to_string()).parse().unwrap_or(r); // przesuwam sie o 1 element dalej i teraz 2gi arg ma index 0

    let args: Vec<String> = std::env::args().collect(); //teraz sobie zbieram te argumenty do wektora

    if args.len()>1{
        c=args[1].parse().unwrap_or(c);
    }
    if args.len()>2{
        r=args[2].parse().unwrap_or(r);
    }


    Config{
        maxx:c,
        maxy:r,
    }
}


//kcharselect - tablica znakow w linuxie mozna tam sobie powybierac elementy

//sdl -biblioteka graficzna w rust. do rysowania okienek np