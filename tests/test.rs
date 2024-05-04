use std::collections::HashSet;

#[test]
fn test() {
    let value: Vec<HashSet<u32>> = vec![
        HashSet::from([21, 26, 7, 22, 25]),
        HashSet::from([22, 26, 18, 9, 24]),
        HashSet::from([25, 24, 1, 6, 27]),
        HashSet::from([8, 14, 26, 27, 29]),
        HashSet::from([13, 26, 1, 20, 28]),
        HashSet::from([20, 22, 14, 19, 8]),
        HashSet::from([25, 21, 29, 23, 26]),
        HashSet::from([20, 4, 10, 11, 8]),
        HashSet::from([26, 25, 15, 8, 3]),
        HashSet::from([1, 9, 3, 7, 17]),
        HashSet::from([27, 24, 7, 13, 29]),
        HashSet::from([0, 2, 5, 12, 16]),
        HashSet::from([5]),
        HashSet::from([6, 9]),
        HashSet::from([7]),
        HashSet::from([6, 9, 13, 15]),
        HashSet::from([20, 22]),
        HashSet::from([5]),
        HashSet::from([6, 9]),
        HashSet::from([7]),
        HashSet::from([6, 9, 13, 15]),
        HashSet::from([20, 22]),
    ];
    let weight: Vec<u32> = vec![1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

    let result = sum_subset::sum_subset::resolve(value, weight, |x, y| x * x + y * y + 2 * x * y);

    for item in result.iter() {
        let mut v = Vec::from_iter(item.visited_indices.iter());
        v.sort();
        println!("{:?} {:?}", v, item.union_values.len());
    }
    println!("{}", result.len());
}
