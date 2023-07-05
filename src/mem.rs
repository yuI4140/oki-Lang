#![allow(unused)]
use std::vec::Vec;
#[derive(Debug, Clone)]
pub struct Stack<T>
where
	T: Clone,
{
	stk: Vec<T>,
}

impl<T> Stack<T>
where
	T: Clone,
{
	pub fn init() -> Stack<T> {
		Stack { stk: Vec::new() }
	}

	pub fn insert(&mut self, item: T) {
		self.stk.push(item);
	}

	pub fn pop(&mut self) -> Option<T> {
		self.stk.pop()
	}

	pub fn search(&self, item: &T) -> Option<usize>
	where
		T: PartialEq,
	{
		self.stk.iter().rposition(|x| *x == *item)
	}
	pub fn len(&self) -> usize {
		self.stk.len()
	}
	pub fn get_vec(&self) -> Vec<T> {
		self.stk.clone()
	}
}

#[derive(Debug, Clone)]
struct Heap<T> {
	cap: usize,
	size: usize,
	mem: Vec<T>,
}

impl<T: std::clone::Clone> Heap<T> {
	fn alloc(&mut self, item: T) -> Option<usize> {
		if self.size < self.cap {
			self.mem.push(item);
			let index = self.size;
			self.size += 1;
			Some(index)
		} else {
			None
		}
	}

	fn dealloc(&mut self, index: usize) -> Option<T> {
		if index < self.size {
			let item = self.mem.remove(index);
			self.size -= 1;
			Some(item)
		} else {
			None
		}
	}

	fn calloc(&mut self, count: usize, default: T) -> Vec<usize> {
		let mut indices = Vec::new();
		for _ in 0..count {
			if let Some(index) = self.alloc(default.clone()) {
				indices.push(index);
			} else {
				break;
			}
		}
		indices
	}

	fn rewind(&mut self, s: usize) {
		self.size = s;
	}
}

#[derive(Debug, Clone)]
pub struct Ren<T> {
	heap: Heap<T>,
	size: usize,
}

impl<T: std::clone::Clone + std::default::Default> Ren<T> {
	pub fn new(capacity_bytes: usize) -> Ren<T> {
		let word_size = std::mem::size_of::<T>();
		let capacity_words = (capacity_bytes + word_size - 1) / word_size;
		let heap = Heap {
			cap: capacity_words,
			size: 0,
			mem: Vec::new(),
		};

		Ren { heap, size: 0 }
	}

	pub fn allocate(&mut self, size_bytes: usize) -> Option<&mut [T]> {
		let size_words = (size_bytes + std::mem::size_of::<T>() - 1) / std::mem::size_of::<T>();
		let indices = self.heap.calloc(size_words, Default::default());

		if indices.is_empty() {
			return None;
		}

		self.size += size_words;

		let start = indices[0];
		let end = indices[size_words - 1] + 1;

		Some(&mut self.heap.mem[start..end])
	}

	pub fn reset(&mut self) {
		self.size = 0;
		self.heap.size = 0;
	}

	fn occupied_bytes(&self) -> usize {
		self.size * std::mem::size_of::<T>()
	}

	fn save(&self) -> usize {
		self.size
	}

	fn rewind(&mut self, s: usize) {
		self.size = s;
		self.heap.rewind(s);
	}
}

impl<T> Drop for Ren<T> {
	fn drop(&mut self) {
		self.heap.size = self.size;
	}
}
