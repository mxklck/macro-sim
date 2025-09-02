use rand::Rng;

fn main() {
    let mut rng = rand::rng();

    let mut v = vec![1, 2, 3, 4, 5];
    for i in v.iter_mut() {
        *i = rng.random_range(0..10) // dereference
    }
    println!("{:?}", v);
}
