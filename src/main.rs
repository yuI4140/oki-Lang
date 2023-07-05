use crate::prelude::*;
mod oki_core;
//use oki_core::*;
mod error;
mod mem;
mod prelude;
mod utils;
use mem::*;
fn pop_stk<T: std::fmt::Display + std::clone::Clone>(mut stk: Stack<T>, total_pops: u32) {
	if total_pops == 0 {
		return;
	}
	stk.pop();
    stk.pop();
    stk.pop();
	if let Some(item) = stk.get_vec().last() {
		println!("cPopped: {}", item);
	}
	pop_stk(stk, total_pops - 1);
}
fn main() -> Result<()> {
	println!("running!");
	let var = 2;
	let var2 = 2;
	let mut stack: Stack<i32> = Stack::init();
	// Insert elements into the stack
	stack.insert(var);
	stack.insert(var2);
	stack.insert(10);
	stack.insert(20);
	stack.insert(30);
	stack.insert(40);

	// Print the stack
	println!("Stack: {:?}", stack);

	// Pop elements from the stack
	pop_stk(stack.clone(),6);
	// Try to pop an item from an empty stack
	if let Some(item) = stack.pop() {
		println!("ePopped: {}", item);
	} else {
		println!("Stack is empty");
	}
	Ok(())
}
