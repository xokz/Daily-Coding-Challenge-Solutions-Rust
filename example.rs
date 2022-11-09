fn main() {
	let stair_len = 5;
	let steps = [1, 2, 3];
	get_paths(&steps[..], stair_len);
}

fn get_paths(steps: &[i32], len: i32) {
	let mut paths: Vec<Vec<i32>> = Vec::new(); // store working paths
	let mut current_path: Vec<i32> = Vec::new(); // store current path
	rec_paths(&mut paths, &mut current_path, &steps, len);
	println!("{:?}", paths);
}

fn rec_paths(paths: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, steps: &[i32], len: i32) {
	for step in steps {
		temp.push(*step);
		//println!("temp: {:?}", temp);
		let sum: i32 = temp.iter().sum(); // for comparing against the len
		match sum {
			i if i == len => {
				paths.push(temp.to_vec());
			},
			i if i < len => {
				rec_paths(paths, temp, steps, len);
			},
			_ => {
				
			}
		}
		temp.pop();
	}
}
