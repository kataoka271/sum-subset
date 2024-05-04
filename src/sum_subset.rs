use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::thread;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct Node {
    pub current_index: usize,
    pub visited_indices: HashSet<usize>,
    pub union_values: HashSet<u32>,
    pub distance: u32,
}

pub fn resolve<F>(value: Vec<HashSet<u32>>, weight: Vec<u32>, calc_distance: F) -> Vec<Node>
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
                distance: item.distance + item.visited_indices.iter().map(|i| calc_distance(weight[*i], weight[j])).sum::<u32>(),
            };
            node.visited_indices.insert(node.current_index);
            node.union_values.extend(&value[node.current_index]);
            q.push_back(node);
        }
    }
    let result: Vec<_> = result.into_iter().filter(|node| node.distance <= min_distance).collect();
    result
}

pub fn resolve_rec<F>(value: Vec<HashSet<u32>>, weight: Vec<u32>, calc_distance: F) -> Vec<Node>
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
    let mut result = Vec::new();

    for item in q.into_iter() {
        min_distance = resolve_rec_sub(
            item,
            min_distance,
            &mut result,
            Param {
                k,
                n,
                value: &value,
                weight: &weight,
                calc_distance: &calc_distance,
            },
        );
    }

    let result: Vec<_> = result.into_iter().filter(|node| node.distance <= min_distance).collect();
    result
}

pub fn resolve_rec_spawn<F>(value: Vec<HashSet<u32>>, weight: Vec<u32>, calc_distance: F) -> Vec<Node>
where
    F: Fn(u32, u32) -> u32 + Send + Sync + 'static,
{
    let value = Arc::new(value);
    let weight = Arc::new(weight);
    let calc_distance = Arc::new(calc_distance);

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

    let min_distance = Arc::new(Mutex::new(u32::MAX));
    let mut handlers = Vec::new();

    for item in q.into_iter() {
        let value = value.clone();
        let weight = weight.clone();
        let calc_distance = calc_distance.clone();
        let min_distance = min_distance.clone();
        let handler = thread::spawn(move || {
            let mut result: Vec<Node> = Vec::new();
            resolve_rec_spawn_sub(
                item,
                min_distance,
                &mut result,
                Param {
                    k,
                    n,
                    value: &value,
                    weight: &weight,
                    calc_distance: calc_distance.as_ref(),
                },
            );
            result
        });
        handlers.push(handler);
    }

    let mut result = Vec::new();
    for handler in handlers {
        let item = handler.join().unwrap();
        result.extend(item);
    }

    let min_distance_sync = min_distance.lock().unwrap();

    let result: Vec<_> = result.into_iter().filter(|node| node.distance <= *min_distance_sync).collect();
    remove_duplicates(result, |node| &node.visited_indices)
}

pub fn resolve_rec_rayon<F>(value: Vec<HashSet<u32>>, weight: Vec<u32>, calc_distance: F) -> Vec<Node>
where
    F: Fn(u32, u32) -> u32 + Send + Sync + 'static,
{
    let mut union_set: HashSet<u32> = HashSet::new();
    for s in value.iter() {
        union_set.extend(s);
    }
    let k: usize = union_set.len();
    let n: usize = value.len();

    println!("k = {}, n = {}", k, n);

    let mut q: Vec<Node> = Vec::new();
    q.extend(value.iter().enumerate().map(|(i, v)| Node {
        current_index: i,
        visited_indices: HashSet::from([i]),
        union_values: v.clone(),
        distance: 0,
    }));

    let min_distance = Arc::new(Mutex::new(u32::MAX));

    let result = q
        .into_par_iter()
        .flat_map(|item| {
            let mut result = Vec::new();
            resolve_rec_spawn_sub(
                item,
                min_distance.clone(),
                &mut result,
                Param {
                    k,
                    n,
                    value: &value,
                    weight: &weight,
                    calc_distance: &calc_distance,
                },
            );
            result
        })
        .filter(|node| node.distance <= *min_distance.lock().unwrap())
        .collect();

    remove_duplicates(result, |node| &node.visited_indices)
}

struct Param<'a, F> {
    k: usize,
    n: usize,
    value: &'a Vec<HashSet<u32>>,
    weight: &'a Vec<u32>,
    calc_distance: &'a F,
}

fn resolve_rec_sub<F>(item: Node, mut min_distance: u32, result: &mut Vec<Node>, param: Param<F>) -> u32
where
    F: Fn(u32, u32) -> u32,
{
    if item.union_values.len() == param.k {
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
    for j in item.current_index + 1..param.n {
        let mut node = Node {
            current_index: j,
            visited_indices: item.visited_indices.clone(),
            union_values: item.union_values.clone(),
            distance: item.distance
                + item
                    .visited_indices
                    .iter()
                    .map(|i| (param.calc_distance)(param.weight[*i], param.weight[j]))
                    .sum::<u32>(),
        };
        node.visited_indices.insert(node.current_index);
        node.union_values.extend(param.value[node.current_index].iter());
        min_distance = resolve_rec_sub(node, min_distance, result, Param { ..param });
    }
    min_distance
}

fn remove_duplicates<T, F, K>(vec: Vec<T>, key: F) -> Vec<T>
where
    F: Fn(&T) -> &HashSet<K>,
    K: Eq + Ord + Hash + Clone + Copy,
{
    let mut unique_vec = Vec::new();
    let mut seen_elems = HashSet::new();

    for elem in vec {
        let mut keys: Vec<_> = key(&elem).iter().copied().collect();
        keys.sort();
        if seen_elems.insert(keys) {
            unique_vec.push(elem);
        }
    }

    unique_vec
}

fn resolve_rec_spawn_sub<F>(item: Node, min_distance: Arc<Mutex<u32>>, result: &mut Vec<Node>, param: Param<F>)
where
    F: Fn(u32, u32) -> u32,
{
    {
        let mut min_distance_sync = min_distance.lock().unwrap();
        if item.union_values.len() == param.k {
            if item.distance < *min_distance_sync {
                *min_distance_sync = item.distance;
                result.push(item);
                return;
            } else {
                result.push(item);
                return;
            }
        }
        if item.distance >= *min_distance_sync {
            return;
        }
    }
    for j in item.current_index + 1..param.n {
        let mut node = Node {
            current_index: j,
            visited_indices: item.visited_indices.clone(),
            union_values: item.union_values.clone(),
            distance: item.distance
                + item
                    .visited_indices
                    .iter()
                    .map(|i| (param.calc_distance)(param.weight[*i], param.weight[j]))
                    .sum::<u32>(),
        };
        node.visited_indices.insert(node.current_index);
        node.union_values.extend(&param.value[node.current_index]);
        resolve_rec_spawn_sub(node, min_distance.clone(), result, Param { ..param });
    }
}
