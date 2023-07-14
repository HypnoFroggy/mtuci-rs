use std::collections::HashMap;
pub mod vector {
	use std::collections::HashMap;
	#[derive(Debug)]
	pub struct Gector <T> {
		pub val: HashMap<i32, Option<T>>,
		pub capacity: i32
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
	impl<T: std::cmp::PartialEq> Gector<T>{
		pub fn push(&mut self, value: T) {
			match self.val.get(&self.capacity) {
				Some(None) => {
					self.val.insert(self.capacity,Some(value));
					
				}
				None => todo!(),
				Some(&Some(_)) => {
					self.val.insert(self.capacity,Some(value));
					self.capacity += 1;
				}
			}
		}
	}
}

fn main() {
	use vector::Gector;
	let mut x: Gector<i32> = vector::new();
	x.push(5);
	x.push(4);
	//println!("{:?}",x.val);
}
