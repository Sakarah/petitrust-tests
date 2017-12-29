fn foo(x:&Vec<bool>){}

fn main()
{
	let v = vec![];
	let a = &v;
	let b = &v;
	foo(a);
	let mut t = &vec![1];
	t=b;
}
