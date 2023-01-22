fn rec_binary<T: std::cmp::PartialEq + std::cmp::PartialOrd>(arr: &[T], target: T, left: usize, right: usize) -> i32{
	let mid = (left+right)/2;
	if right >= left {
		if arr[mid] == target {
			return mid as i32
		} else if arr[mid] < target {
			return rec_binary(&arr, target, mid+1, right)
		} else if arr[mid] > target {
			return rec_binary(&arr, target, left, mid-1)
		}
	}
	return -1
}

fn itr_binary<T: std::cmp::PartialEq + std::cmp::PartialOrd>(arr: &[T], target: T) -> i32{
	let mut left: usize = 0;
	let mut right: usize = arr.len()-1;
	let mut mid: usize = 0;
	loop {
		mid = (left+right)/2;
		if right >= left {
			if arr[mid] == target {
				return mid as i32
			} else if arr[mid] > target {
				right = mid - 1 ;
			} else if arr[mid] < target {
				left = mid + 1;
			} 
		} else {
			break;
		}
	}
	return -1
}

fn main(){
	let arr = [1,2,3,4,5,6,7,8,9,10,11,12,13];
	let target = 9;
	let index = rec_binary(&arr, target, 0, arr.len()-1);
	match index {
		-1 => println!("Element Not Found"),
		_ => println!("Element at {index}"),
	}
	let index = itr_binary(&arr, target);
	match index {
		-1 => println!("Element Not Found"),
		_ => println!("Element at {index}"),
	}
}