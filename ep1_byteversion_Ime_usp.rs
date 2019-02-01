use std::time::{ Instant};


fn main(){
let now = Instant::now();
let  a:i16 = 2;
let  b:i16 = 100;
// let mut array_of_steps=Vec:: new();
let mut c = make_array(a,b);
for i in 0..c.len(){
	let steps = count_steps(& mut c[i]);
	// array_of_steps.push(steps);
};
let end = Instant::now();
let timeElapsed = (end-now)*1000;
println!("{:?}",timeElapsed);
// println!("{:?}",array_of_steps );
println!("{:?}",c );


}

fn make_array(first:i16, last:i16) ->Vec<i16>{
	let mut array_return = Vec::new();
	for i in first..last+1{
		array_return.push(i);
	}
	return array_return;

}

fn count_steps( a: &mut i16) {
let mut i:i16 = 0;
while *a!=1 {
	if 	*a&1==0{
		*a=*a/2;
		i=i+1;
	}else{
		*a = *a*3+1;
		i=i+1;
	}

}
*a=i;
// return i;
}







