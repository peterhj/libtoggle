use std::marker::{PhantomData};

pub trait Toggle<T> {
  fn as_ref(&self) -> Option<&T>;
  fn as_mut(&mut self) -> Option<&mut T>;
}

pub type Disabled<T> = Disable<T>;
pub type Enabled<T> = Enable<T>;

pub struct Disable<T> {
  _marker:  PhantomData<T>,
}

impl<T> Disable<T> {
  #[inline]
  pub fn new() -> Disable<T> {
    Disable{_marker: PhantomData}
  }
}

impl<T> Toggle<T> for Disable<T> {
  #[inline]
  fn as_ref(&self) -> Option<&T> {
    None
  }

  #[inline]
  fn as_mut(&mut self) -> Option<&mut T> {
    None
  }
}

impl<T> Clone for Disable<T> where T: Clone {
  fn clone(&self) -> Disable<T> {
    Disable{_marker: PhantomData}
  }
}

impl<T> Copy for Disable<T> where T: Copy {
}

pub struct Enable<T> {
  value:    T,
}

impl<T> Enable<T> {
  #[inline]
  pub fn new(value: T) -> Enable<T> {
    Enable{value: value}
  }
}

impl<T> Toggle<T> for Enable<T> {
  #[inline]
  fn as_ref(&self) -> Option<&T> {
    Some(&self.value)
  }

  #[inline]
  fn as_mut(&mut self) -> Option<&mut T> {
    Some(&mut self.value)
  }
}

impl<T> Clone for Enable<T> where T: Clone {
  fn clone(&self) -> Enable<T> {
    Enable{value: self.value.clone()}
  }
}

impl<T> Copy for Enable<T> where T: Copy {
}

#[cfg(test)]
mod tests {

  use super::{Toggle, Disable, Enable};

  struct Hello<Tg> where Tg: Toggle<String> {
    t: Tg,
  }

  #[test]
  fn test_disable() {
    let hello = Hello{t: Disable::new()};
    assert!(hello.t.as_ref().is_none());
  }

  #[test]
  fn test_disable_mut() {
    let mut hello = Hello{t: Disable::new()};
    assert!(hello.t.as_mut().is_none());
  }

  #[test]
  fn test_enable() {
    let hello = Hello{t: Enable::new(format!("Hello, world!"))};
    assert!(hello.t.as_ref().is_some());
  }

  #[test]
  fn test_enable_mut() {
    let mut hello = Hello{t: Enable::new(format!("Hello, world!"))};
    *hello.t.as_mut().unwrap() = format!("Goodbye, cruel world!");
    assert_eq!("Goodbye, cruel world!", hello.t.as_ref().unwrap());
  }

}
