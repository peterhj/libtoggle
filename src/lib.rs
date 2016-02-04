use std::marker::{PhantomData};

pub trait Toggle<T> {
  fn as_ref(&self) -> Option<&T>;
  fn as_mut(&mut self) -> Option<&mut T>;
}

pub struct Disabled<T> {
  _marker:  PhantomData<T>,
}

impl<T> Disabled<T> {
  #[inline]
  pub fn new() -> Disabled<T> {
    Disabled{_marker: PhantomData}
  }
}

impl<T> Toggle<T> for Disabled<T> {
  #[inline]
  fn as_ref(&self) -> Option<&T> {
    None
  }

  #[inline]
  fn as_mut(&mut self) -> Option<&mut T> {
    None
  }
}

pub struct Enabled<T> {
  value:    T,
}

impl<T> Enabled<T> {
  #[inline]
  pub fn new(value: T) -> Enabled<T> {
    Enabled{value: value}
  }
}

impl<T> Toggle<T> for Enabled<T> {
  #[inline]
  fn as_ref(&self) -> Option<&T> {
    Some(&self.value)
  }

  #[inline]
  fn as_mut(&mut self) -> Option<&mut T> {
    Some(&mut self.value)
  }
}

#[cfg(test)]
mod tests {

  use super::{Toggle, Disabled, Enabled};

  struct Hello<Tg> where Tg: Toggle<String> {
    t: Tg,
  }

  #[test]
  fn test_disabled() {
    let hello = Hello{t: Disabled::new()};
    assert!(hello.t.as_ref().is_none());
  }

  #[test]
  fn test_disabled_mut() {
    let mut hello = Hello{t: Disabled::new()};
    assert!(hello.t.as_mut().is_none());
  }

  #[test]
  fn test_enabled() {
    let hello = Hello{t: Enabled::new(format!("Hello, world!"))};
    assert!(hello.t.as_ref().is_some());
  }

  #[test]
  fn test_enabled_mut() {
    let mut hello = Hello{t: Enabled::new(format!("Hello, world!"))};
    *hello.t.as_mut().unwrap() = format!("Goodbye, cruel world!");
    assert_eq!("Goodbye, cruel world!", hello.t.as_ref().unwrap());
  }

}
