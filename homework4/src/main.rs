use std::collections::HashMap;
pub mod vector {
	use std::collections::HashMap;
	#[derive(Debug)]
	pub struct Gector <T> {
		val: HashMap<i32, Option<T>>,
		capacity: i32
	}
	
	pub fn new<T>() -> Gector <T> {
		let mut hash = HashMap::new();
		hash.insert(0,None);
		let y: Gector <T> = Gector  {
			val: hash,
			capacity: 0
		};
		y
	}
	impl<T: std::cmp::PartialEq + std::fmt::Display> Gector<T>{
		pub fn push(&mut self, value: T) {
			let s = self.capacity;
			match self.val.get(&s) {
				Some(None) => {
					self.val.insert(self.capacity,Some(value));
				},
				Some(_) => {
					self.capacity += 1;
					self.val.insert(self.capacity,Some(value));
				},
				None => print!("{}",value),
			}
		}
	}
}

fn main() {
	use vector::Gector;
	let mut x: Gector<i32> = vector::new();
	x.push(5);
	x.push(4);
	x.push(3);
	x.push(1);
	println!("{:?}",x);
}
