use std;
use std::map;
use std::map::hashmap;
use std::map::map;

// Test that trait types printed in error msgs include the type arguments.

fn main() {
    let x: map<~str,~str> = map::str_hash::<~str>() as map::<~str,~str>;
    let y: map<uint,~str> = x;
    //~^ ERROR mismatched types: expected `@std::map::map<uint,~str>`
}
