#![feature(proc_macro, proc_macro_non_items)]

extern crate jsx_macro;

use jsx_macro::jsx;

fn main() {
  let a = jsx!(<div />);
  println!("\nfinal output = {:?}\n", a);
}
