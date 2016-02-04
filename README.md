# libtoggle

`libtoggle` is a Rust library for [feature toggles](http://martinfowler.com/bliki/FeatureToggle.html).

Basic usage:
```
use toggle::{Toggle, Disabled, Enabled};

struct HelloStruct<MapTg> where MapTg: Toggle<HashMap<String, i32>> {
  t: MapTg,
}

fn main() {
  let nomap = HelloStruct{
    t: Disabled::new(),
  }
  nomap.t.as_ref().map(|_| { println!("i am a spooky ghost"); });

  let mut hasmap = HelloStruct{
    t: Enabled::new(HashMap::new()),
  };
  hasmap.t.as_mut().map(|t| { println!("hello world!"); });
}
```
