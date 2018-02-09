use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let max = 200_000;
    let mut vec: Vec<u32> = Vec::with_capacity(max);
    let mut iterator = 0;
    let mut number: u32 = 1;

    while iterator < max {
        let mut prime = true;
        number += 1;
        for x in &vec {
            if number / x < 2{
                break;
            }
            if number % x == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            vec.push(number);
            iterator += 1;
        }
    }

    print!("{:?}", (start_time.elapsed().as_secs() as f64) + (start_time.elapsed().subsec_nanos() as f64 / 1000_000_000.0));
    //print!("{}", number);
    /*for x in &vec {
        print!("{}\n", x);
    }*/
    print!("{:?}", number);
}
