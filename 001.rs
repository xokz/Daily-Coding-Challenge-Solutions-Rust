use std::collections::HashMap;

fn main() {
	let arr = [10, 15, 3, 7];
	println!("arr contains sum? {}", has_sum(&arr[..], 19));
}

fn has_sum(arr: &[i32], sum: i32) -> bool{
	let mut nums: HashMap<i32, bool> = HashMap::new();
	for i in arr {
		if nums.contains_key(&(sum-i)) {
			return true
		} 
		nums.insert(*i, true);
	}
	false
}
