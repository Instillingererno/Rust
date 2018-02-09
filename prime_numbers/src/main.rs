fn main() {
    let mut prime_array: [u32; 20000] = [0; 20000];
    let mut iterator = 0;
    let mut number = 1;

    while iterator < 20000 {
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
    print!("{}", number);
    /*for x in prime_array.iter() {
        print!("{}\n", x);
    }*/
}
