fn selection<T: std::cmp::PartialOrd>(arr: &mut[T]){
	let n: usize = arr.len()-1;
	for i in 0..n{
		let mut min = i;
		for j in i+1..=n{
			if arr[min] > arr[j]{
				min = j;
			}
		}
		arr.swap(i,min);
	}
}

fn main() {
	let mut arr = [1,4,3,2,7,5,6];
	selection(&mut arr);
	println!("{:?}", arr);
	let mut arr = ['c','a','d','f','b','e'];
	selection(&mut arr);
	println!("{:?}", arr);
}