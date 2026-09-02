# dhist

A simple library designed to compute and draw histograms.

## Example

```rust
let mut rng = rng();
let rnd = Binomial::new(40, 0.5).expect("Invalid distribution parameters");

let mut hist = Histogram::new('█');

for _ in 0..10000 {
    hist.insert(rnd.sample(&mut rng) as u32);
}

println!("{}", hist.bucket(&LinearBucketer::new(10)));

// Example output:

// 8 - 10           (12)
// 11 - 12        █  (83)
// 13 - 14        ██████  (336)
// 15 - 17        █████████████████████████████████  (1657)
// 18 - 19        ███████████████████████████████████████████  (2177)
// 20 - 21        ██████████████████████████████████████████████████  (2475)
// 22 - 24        █████████████████████████████████████████████████  (2450)
// 25 - 26        ████████████  (606)
// 27 - 28        ███  (179)
// 29 - 31          (25)
```

## Features
- Different bucketing options, that work well with different data types
- Customizable drawing
- Extremely simple API
- 100% artesanal hand-written software

## More examples
This crate was made for [picstat](https://github.com/m-demare/picstat). You can find more
usage examples there.


