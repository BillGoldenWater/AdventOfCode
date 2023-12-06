/*
 * Copyright 2023 Golden_Water
 * All rights reserved
 */

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::{Arc, OnceLock};

pub trait Then {
  fn then<R>(self, f: impl FnOnce(Self) -> R) -> R
  where
    Self: Sized,
  {
    f(self)
  }

  fn some(self) -> Option<Self>
  where
    Self: Sized,
  {
    Some(self)
  }

  fn println(&self)
  where
    Self: Display,
  {
    println!("{self}");
  }

  fn println_dbg(&self)
  where
    Self: Debug,
  {
    println!("{self:#?}");
  }

  fn println_ret(self) -> Self
  where
    Self: Sized + Display,
  {
    println!("{self}");
    self
  }

  fn println_ret_dbg(self) -> Self
  where
    Self: Sized + Debug,
  {
    println!("{self:#?}");
    self
  }
}

pub trait With {
  fn with<T>(self, v: T) -> (Self, T)
  where
    Self: Sized,
  {
    (self, v)
  }
}

impl<T> Then for T {}
impl<T> With for T {}

pub trait HashMapExt<K, V> {
  fn inserted(self, k: K, v: V) -> Self
  where
    Self: Sized;

  fn inserted_box(self: Box<Self>, k: K, v: V) -> Box<Self>
  where
    Self: Sized;
}

impl<K, V> HashMapExt<K, V> for HashMap<K, V>
where
  K: Eq + Hash,
{
  fn inserted(mut self, k: K, v: V) -> Self
  where
    Self: Sized,
  {
    self.insert(k, v);
    self
  }

  fn inserted_box(mut self: Box<Self>, k: K, v: V) -> Box<HashMap<K, V>>
  where
    Self: Sized,
  {
    self.insert(k, v);
    self
  }
}

pub trait OnceLockExt<T> {
  fn inited_arc(self: Arc<Self>, v: T) -> Arc<Self>
  where
    Self: Sized;
}

impl<T> OnceLockExt<T> for OnceLock<T> {
  fn inited_arc(self: Arc<Self>, v: T) -> Arc<Self>
  where
    Self: Sized,
  {
    self.get_or_init(|| v);
    self
  }
}
