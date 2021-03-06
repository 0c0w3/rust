// error-pattern:assigning to immutable field
struct cat {
  priv {
    mut meows : uint,
  }

  how_hungry : int,

}

fn cat(in_x : uint, in_y : int) -> cat {
    cat {
        meows: in_x,
        how_hungry: in_y
    }
}

fn main() {
  let nyan : cat = cat(52u, 99);
  nyan.how_hungry = 0;
}
