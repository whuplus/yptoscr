use bors::{deserialize, serialize};

pub fn execute(tx: String) {
    println!("{}", serialize(&deserialize(&tx), false));
}
