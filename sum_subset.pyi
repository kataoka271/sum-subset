from typing import Callable

def resolve_sum_of_subset(
    value: list[set[int]], weight: list[int], calc_distance: Callable[[int, int], int] | None = None
): ...
def resolve_sum_of_subset_rec(
    value: list[set[int]], weight: list[int], calc_distance: Callable[[int, int], int] | None = None
): ...
def resolve_sum_of_subset_rec_spawn(
    value: list[set[int]], weight: list[int], calc_distance: Callable[[int, int], int] | None = None
): ...
