fn linear<T: std::cmp::PartialEq>(arr: &[T], target: T) -> i32 {
	for (index, value) in arr.iter().enumerate() {
		if *value == target {
			return index as i32;
		}
	}
	-1
}

fn main(){
	let arr = [1,2,3,4,5];
	let target = 3;
	let index = linear(&arr, target);
	match index {
		-1 => println!("Element Not Found"),
		_ => println!("Element at {index}"),
	}
}