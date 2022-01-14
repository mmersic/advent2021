use itertools::min;
use took::Timer;

pub fn part_one() {
    let timer = Timer::new();

    let ty_min : i32 = -215;

    println!("day 17, part 1: {}, in time {}ns", ty_min.abs()*(ty_min.abs()-1)/2, timer.took().as_std().as_nanos());
}

pub fn part_two() {
    let timer = Timer::new();

    let tx_min = 34;
    let tx_max = 67;
    let ty_min:i32 = -215;
    let ty_max = -186;

    // let tx_min = 20;
    // let tx_max = 30;
    // let ty_min = -10;
    // let ty_max = -5;

    let min_x_range = ((2.0 * tx_min as f32).sqrt()).floor() as i32 - 1; //solving for t*(t-1)/2 = x_min, to find the time to just hit x_min
    
    let mut count = 0;
    for x in min_x_range .. tx_max+1 {
        for y in -ty_min.abs() .. ty_min.abs() {
            let mut t = 1;
            loop {
                let mut p_x = 0;
                if x > t {
                    p_x = x*t - ((t-1)*t)/2;
                } else {
                    p_x = x*x - ((x-1)*x)/2;
                }
                let p_y = y*t - ((t-1)*t)/2;
                if p_y <= ty_max && p_y >= ty_min && p_x <= tx_max && p_x >= tx_min {
                    count += 1;
                    break;
                }

                if p_y < ty_min || p_x > tx_max {
                    break;
                }
                t += 1;
            }
        }
    }

    println!("day 17, part 2: {}, in time {}us", count, timer.took().as_std().as_micros());

}