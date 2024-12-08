extern crate core;

use std::alloc::alloc;
use std::ptr::NonNull;
use crate::allocator::MyAllocator;
use crate::pages::Page;

mod linked_list;
mod order;
mod pages;
mod allocator;
mod node;


static mut MEMORY: Page = Page::ZERO;

fn main() {
    let ptr = NonNull::new(unsafe { MEMORY.0.as_mut_ptr() }).unwrap();
    let len = core::mem::size_of_val(unsafe { &MEMORY });

    let mut allocator = MyAllocator::<16>::new();
    println!("Hello, world!");
    unsafe { allocator.transfer(ptr, len) };
    println!("Hello, world!");
    
    println!("allocator: {allocator:?}");

    println!("Hello, world!");
}

