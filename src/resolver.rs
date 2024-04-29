use std::collections::{HashSet, VecDeque};

pub struct Node {
    pub current_index: usize,
    pub visited_indices: HashSet<usize>,
    pub union_values: HashSet<u32>,
    pub distance: u32,
}

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
    let weight: Vec<u32> = vec![
        1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    let result = resolve_sum_of_subset(&value, &weight, |x, y| x * x + y * y + 2 * x * y);

    for item in result.iter() {
        let mut v = Vec::from_iter(item.visited_indices.iter());
        v.sort();
        println!("{:?} {:?}", v, item.union_values.len());
    }
    println!("{}", result.len());
}

pub fn resolve_sum_of_subset<F>(
    value: &Vec<HashSet<u32>>,
    weight: &Vec<u32>,
    calc_distance: F,
) -> Vec<Node>
where
    F: Fn(u32, u32) -> u32,
{
    let mut union_set: HashSet<u32> = HashSet::new();
    for s in value.iter() {
        union_set.extend(s);
    }
    let k: usize = union_set.len();
    let n: usize = value.len();

    println!("k = {}, n = {}", k, n);

    let mut q: VecDeque<Node> = VecDeque::new();
    q.extend(value.iter().enumerate().map(|(i, v)| Node {
        current_index: i,
        visited_indices: HashSet::from([i]),
        union_values: v.clone(),
        distance: 0,
    }));

    let mut min_distance = u32::MAX;
    let mut result: Vec<Node> = Vec::new();

    while let Some(item) = q.pop_front() {
        if item.union_values.len() == k {
            if item.distance < min_distance {
                min_distance = item.distance;
            }
            result.push(item);
            continue;
        }
        if item.distance >= min_distance {
            continue;
        }
        for j in item.current_index + 1..n {
            let mut node = Node {
                current_index: j,
                visited_indices: item.visited_indices.clone(),
                union_values: item.union_values.clone(),
                distance: item.distance
                    + item
                        .visited_indices
                        .iter()
                        .map(|i| calc_distance(weight[*i], weight[j]))
                        .sum::<u32>(),
            };
            node.visited_indices.insert(node.current_index);
            node.union_values.extend(&value[node.current_index]);
            q.push_back(node);
        }
    }
    let result: Vec<_> = result
        .into_iter()
        .filter(|node| node.distance <= min_distance)
        .collect();
    result
}

pub fn resolve_sum_of_subset_rec<F>(
    value: &Vec<HashSet<u32>>,
    weight: &Vec<u32>,
    calc_distance: F,
) -> Vec<Node>
where
    F: Fn(u32, u32) -> u32,
{
    let mut union_set: HashSet<u32> = HashSet::new();
    for s in value.iter() {
        union_set.extend(s);
    }
    let k: usize = union_set.len();
    let n: usize = value.len();

    println!("k = {}, n = {}", k, n);

    let mut q: VecDeque<Node> = VecDeque::new();
    q.extend(value.iter().enumerate().map(|(i, v)| Node {
        current_index: i,
        visited_indices: HashSet::from([i]),
        union_values: v.clone(),
        distance: 0,
    }));

    let mut min_distance = u32::MAX;
    let mut ret = Vec::new();

    for item in q.into_iter() {
        min_distance = resolve_sum_of_subset_rec_sub(
            item,
            min_distance,
            k,
            n,
            value,
            weight,
            &calc_distance,
            &mut ret,
        );
    }

    let result: Vec<_> = ret
        .into_iter()
        .filter(|node| node.distance <= min_distance)
        .collect();
    result
}

fn resolve_sum_of_subset_rec_sub<F>(
    item: Node,
    mut min_distance: u32,
    k: usize,
    n: usize,
    value: &Vec<HashSet<u32>>,
    weight: &Vec<u32>,
    calc_distance: &F,
    result: &mut Vec<Node>,
) -> u32
where
    F: Fn(u32, u32) -> u32,
{
    if item.union_values.len() == k {
        if item.distance < min_distance {
            min_distance = item.distance;
            result.push(item);
            return min_distance;
        } else {
            result.push(item);
            return min_distance;
        }
    }
    if item.distance >= min_distance {
        return min_distance;
    }
    for j in item.current_index + 1..n {
        let mut node = Node {
            current_index: j,
            visited_indices: item.visited_indices.clone(),
            union_values: item.union_values.clone(),
            distance: item.distance
                + item
                    .visited_indices
                    .iter()
                    .map(|i| calc_distance(weight[*i], weight[j]))
                    .sum::<u32>(),
        };
        node.visited_indices.insert(node.current_index);
        node.union_values.extend(&value[node.current_index]);
        min_distance = resolve_sum_of_subset_rec_sub(
            node,
            min_distance,
            k,
            n,
            value,
            weight,
            calc_distance,
            result,
        );
    }
    min_distance
}
