fn insertion<T: std::cmp::PartialEq + std::cmp::PartialOrd>(arr: &mut[T]) {
	let n: usize = arr.len();
	for i in 1..n{
		let mut j = i;
		while j > 0 && arr[j-1] > arr[j]{
			arr.swap(j, j-1);
			j -= 1;
		}
	}
}

fn main() {
	let mut arr = [1,4,3,2,7,5,6];
	insertion(&mut arr);
	println!("{:?}", arr);
	let mut arr = ['c','a','d','f','b','e'];
	insertion(&mut arr);
	println!("{:?}", arr);
}