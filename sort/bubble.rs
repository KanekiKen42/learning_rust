fn bubble<T: std::cmp::PartialEq + std::cmp::PartialOrd>(arr: &mut[T]) {
	for i in 0..arr.len() {
		for j in 0..arr.len()-1-i {
			if arr[j] > arr[j+1]{
				arr.swap(j, j+1);
			}
		}
	}
}

fn main() {
	let mut arr = [1,4,3,2,7,5,6];
	bubble(&mut arr);
	println!("{:?}", arr);
	let mut arr = ['c','a','d','f','b','e'];
	bubble(&mut arr);
	println!("{:?}", arr);
}