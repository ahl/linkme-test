use lib::dummy;
use lib::DOERS;
use linkme::distributed_slice;

#[distributed_slice(DOERS)]
static MINE: fn() = mine;

fn mine() {
    println!("mine");
}

fn main() {
    dummy();
}
