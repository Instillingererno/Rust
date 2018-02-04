fn main() {
    let mut prime_array: [i64; 500000] = [0; 500000];
    let mut iterator = 0;
    let mut number = 1;

    while iterator < 500000 {
        let mut prime = true;
        number += 1;
        for x in 2..(number-1) {
            if number % x == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            prime_array[iterator] = number;
            iterator += 1;
        }
    }
    /*for x in prime_array.iter() {
        print!("{}\n", x);
    }*/
}
