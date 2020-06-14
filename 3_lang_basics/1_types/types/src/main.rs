fn main() {
    // section: integers, floats, and basic math
    const G: f32 = 9.81;
    let mut v: f64 = 0.0;
    let t: i32 = 5;
    let m: f32 = 10.0;

    // notice the ugly, manual type-casting we have to do everywhere
    v = v + (G * (t as f32) / m) as f64;

    println!("The particle is going {0} m/s.", v);

    // let's play with booleans and boolean operators
    let is_fast: bool = v > 4.0;

    // it may not be fashionable, but the imperative boolean statements still exist
    if is_fast {
        println!("Zooooom!");
    }

    // let's try tuples!
    let mut posi: (f64, f64, f64) = (0.0, 0.0, 0.0);

    println!("Starting position: {}, {}, {}", posi.0, posi.1, posi.2);

    posi.2 += v * (t as f64);

    println!("Ending position: {}, {}, {}", posi.0, posi.1, posi.2);

    // Rust also considers arrays to be a type
    let a: [i32; 4] = [1, 3, 5, 7];
    let b: [i32; 4] = [0, 2, 1, 4];

    let mut sum: i32 = 0;

    sum += a.iter().sum::<i32>();
    sum += b.iter().sum::<i32>();

    println!("The sum of both arrays is: {}", sum);
}
