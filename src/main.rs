use crate::prelude::*; 
mod oki_core;
use oki_core::*;
mod error;
mod prelude;
mod utils;
fn main()->Result<()>{
    println!("running!");
    let input="var=2+2";
    let _r=calculator(input);
    println!("{}{}{}={}",_r.0,_r.1,_r.2,_r.3);
    Ok(())
}
