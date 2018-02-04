extern crate rand;

use rand::{thread_rng, Rng};

fn main() {

    // index.choose(&choices)

    let choices = [0,1,2,3,4];
    let choices_two = [0,1,2,3];
    let mut rng = thread_rng();

    let mut array = [0; 5];
    array[0] = 1;
    array[1] = 2;
    array[2] = 3;
    array[3] = 4;
    array[4] = 5;

    for x in 0..20 {
        let mut midl = rng.choose(&choices).unwrap();
        print!("{}, {}\n", x, midl);
    }
}
