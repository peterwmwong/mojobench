fn recursive(x: Int):
    let stuff = DynamicVector[Int](42)
    if x == 0: return
    recursive(x - 1)

fn main():
    recursive(999_999_999)
