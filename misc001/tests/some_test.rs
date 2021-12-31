use misc001;
use misc001::distance_distribution;

#[test]
fn it_adds_one() {
    assert_eq!(4, misc001::add_one(3));
}

#[test]
fn it_calculates_distance_distribution() {
    //let mut diffs = vec![];
    let distance_frequency = distance_distribution(10);

    assert_eq!(10, distance_frequency.len());
    assert_eq!(Some((1, 18)), distance_frequency.get(0).copied());
    assert_eq!(Some((9, 2)), distance_frequency.get(9).copied());
    println!("{:?}", distance_frequency);
}
