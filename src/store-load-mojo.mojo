import benchmark

fn store_load() -> Int:
    var stuff = DynamicVector[Int](42)
    stuff[1] = 2
    return stuff[1]

fn main():
    for i in range(9_999_999):
        benchmark.keep(store_load())
