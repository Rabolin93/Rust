fn main() {
    	let a:f32=1.0;
	let b:f32=2.0;
	let c:f32=-3.0;


	let  delta:f32=(b*b)-4.0*a*c;
	let mut  x1:f32=(-b-delta.sqrt())/(2.0*a);
	let  x2:f32=(-b+delta.sqrt())/(2.0*a);

	println!("Delta: {}",delta);
	if delta>0.0
	{

		let  delta:f32=(b*b)-4.0*a*c;
		let mut  x1:f32=(-b-delta.sqrt())/(2.0*a);
		let  x2:f32=(-b+delta.sqrt())/(2.0*a);
	}
	else if delta==0.0
	{
	x1=-b/(2.0*a);
	}

	println!("Pierwiastek z delty:{}",delta.sqrt());
	println!("Wartosc x1: {}", x1);
	println!("Wartosc x2: {}", x2);

	
}
