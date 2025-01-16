pub mod iterables;
pub mod iterator;

use iterables::vector::*;
use iterator::*;

fn main() {
    let mut list = VecII::<i32>::new();
    list.push(22);
    list.push(33);
    list.push(44);

    let mut it = list.into_iterII();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
}
