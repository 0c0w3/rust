// xfail-fast
use to_str::*;
use to_str::ToStr;

struct cat {
  priv {
    mut meows : uint,
    fn meow() {
      error!("Meow");
      self.meows += 1u;
      if self.meows % 5u == 0u {
          self.how_hungry += 1;
      }
    }
  }

  mut how_hungry : int,
  name : ~str,

  fn speak() { self.meow(); }

  fn eat() -> bool {
    if self.how_hungry > 0 {
        error!("OM NOM NOM");
        self.how_hungry -= 2;
        return true;
    }
    else {
        error!("Not hungry!");
        return false;
    }
  }
}

fn cat(in_x : uint, in_y : int, in_name: ~str) -> cat {
    cat {
        meows: in_x,
        how_hungry: in_y,
        name: in_name
    }
}

impl cat: ToStr {
  fn to_str() -> ~str { self.name }
}

fn print_out<T: ToStr>(thing: T, expected: ~str) {
  let actual = thing.to_str();
  debug!("%s", actual);
  assert(actual == expected);
}

fn main() {
  let nyan : ToStr = cat(0u, 2, ~"nyan") as ToStr;
  print_out(nyan, ~"nyan");
}
