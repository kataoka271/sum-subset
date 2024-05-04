import time
from typing import Callable

import sum_subset


def main():
    L = [
        {21, 26, 7, 22, 25},
        {22, 26, 18, 9, 24},
        {25, 24, 1, 6, 27},
        {8, 14, 26, 27, 29},
        {13, 26, 1, 20, 28},
        {20, 22, 14, 19, 8},
        {25, 21, 29, 23, 26},
        {20, 4, 10, 11, 8},
        {26, 25, 15, 8, 3},
        {1, 9, 3, 7, 17},
        {27, 24, 7, 13, 29},
        {0, 2, 5, 12, 16},
        {5},
        {6, 9},
        {7},
        {6, 9, 13, 15},
        {20, 22},
        {5},  # リストによるQueueを使う場合、Pure Pythonではここが限界で再帰によるスタックを使うと最後までできる
        {6, 9},
        {7},
        {6, 9, 13, 15},
        # {20, 22},
    ]
    W = [1 for _ in L]

    print("Queue In Rust")
    t = time.time()
    print(sum_subset.resolve_sum_of_subset(L, W, lambda x, y: x * x + y * y + 2 * x * y))
    print(time.time() - t)
    print()

    print("Stack In Rust")
    t = time.time()
    print(sum_subset.resolve_sum_of_subset_rec(L, W, lambda x, y: x * x + y * y + 2 * x * y))
    print(time.time() - t)
    print()

    # 距離関数をRust埋め込みにすると更に早い
    print("Queue In Rust (embed calc_distance)")
    t = time.time()
    print(sum_subset.resolve_sum_of_subset(L, W))
    print(time.time() - t)
    print()

    print("Stack In Rust (embed calc_distance)")
    t = time.time()
    print(sum_subset.resolve_sum_of_subset_rec(L, W))
    print(time.time() - t)
    print()

    print("Stack In Rust w/ parallel (embed calc_distance)")
    t = time.time()
    print(sum_subset.resolve_sum_of_subset_rec_spawn(L, W))
    print(time.time() - t)
    print()

    print("Stack In Rust w/ rayon (embed calc_distance)")
    t = time.time()
    print(sum_subset.resolve_sum_of_subset_rec_rayon(L, W))
    print(time.time() - t)
    print()

    # t = time.time()
    # print(resolve_sum_of_subset(L, W, lambda x, y: x * x + y * y + 2 * x * y))
    # print(time.time() - t)
    # print()

    print("Stack In Pure Python")
    t = time.time()
    print(resolve_sum_of_subset_rec(L, W, lambda x, y: x * x + y * y + 2 * x * y))
    print(time.time() - t)


def resolve_sum_of_subset(value: list[set[int]], weight: list[int], calc_distance: Callable[[int, int], int]):
    union_set = set.union(*value)
    k = len(union_set)
    n = len(value)

    print(f"k = {k}, n = {n}")

    min_distance = 0xFFFFFFFF
    q: list[tuple[int, set[int], set[int], int]] = [(i, set([i]), v, 0) for i, v in enumerate(value)]

    result = []

    while q:
        item = q.pop(0)
        current_index, visited_indices, union_values, distance = item
        if len(union_values) == k:
            if distance < min_distance:
                min_distance = distance
            result.append(item)
        if distance >= min_distance:
            continue
        for j in range(current_index + 1, n):
            q.append(
                (
                    j,
                    visited_indices | {j},
                    union_values | value[j],
                    distance + sum(calc_distance(weight[i], weight[j]) for i in visited_indices),
                )
            )

    return [node[1] for node in result if node[3] <= min_distance]


def resolve_sum_of_subset_rec(value: list[set[int]], weight: list[int], calc_distance: Callable[[int, int], int]):
    union_set = set.union(*value)
    k = len(union_set)
    n = len(value)

    print(f"k = {k}, n = {n}")

    def func(
        item: tuple[int, set[int], set[int], int], min_distance: int
    ) -> tuple[int, list[tuple[int, set[int], set[int], int]]]:
        current_index, visited_indices, union_values, distance = item
        if len(union_values) == k:
            if distance < min_distance:
                min_distance = distance
            return (min_distance, [item])
        if distance >= min_distance:
            return (min_distance, [])
        L = []
        for j in range(current_index + 1, n):
            min_distance, items = func(
                (
                    j,
                    visited_indices | {j},
                    union_values | value[j],
                    distance + sum(calc_distance(weight[i], weight[j]) for i in visited_indices),
                ),
                min_distance,
            )
            L.extend(items)
        return (min_distance, L)

    result = []
    min_distance = 0xFFFFFFFF

    for item in [(i, set([i]), v, 0) for i, v in enumerate(value)]:
        min_distance, items = func(item, min_distance)
        result.extend(items)

    return [node[1] for node in result if node[3] <= min_distance]


if __name__ == "__main__":
    main()
