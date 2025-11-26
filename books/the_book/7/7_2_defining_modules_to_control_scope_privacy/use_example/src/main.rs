use std::collections::HashMap;

// Brings two Result types into scope
use std::fmt; // Has Result type
use std::io; // Also has Result type

// Must specify which Result to use (idiomatic)
fn f1() -> fmt::Result {}

// Must specify which Result to use (idiomatic)
fn f2() -> io::Result {}

// Alternatively provide a new name using 'as' (idiomatic)
use std::io::Result as IoResult;

fn f3() -> IoResult<()> {}

///////////////////////////
use std::cmp::Ordering;
use std::io;

// Alternative to above, on one line
use std::{cmp::Ordering, io};

///////////////////////////
use std::io;
use std::io::Write;

// Alternative to above, on one line
use std::io::{self, Write};

////////////////////////////

// Glob operator usage BE CAREFUL not recommended
use std::collections::*; // brings in all public items from std::collections

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
