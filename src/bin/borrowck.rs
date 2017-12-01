use std::cell::RefCell;

fn moving() {
  let v = vec![1, 2, 3];
  let v2 = v;

  println!("{:?}", v2);
}

/* fn move_not_ok() {
  let v = vec![1, 2, 3];
  let v2 = v;

  println!("{:?}", v);
}*/

fn copy() {
  let mut v = vec![1, 2, 3];
  let v2 = v.clone();

  v.push(4);

  println!("{:?} {:?}", v, v2);
}

fn slice_fn(s: &str) {
  println!("{:?}", s);
}

fn slices() {
  let s = String::from("hello world");

  let hello = &s[0..5];
  let world = &s[6..11];

  slice_fn(hello);
  slice_fn(world);
}

/*
fn slices_bad() {
  let s = String::from("hello world");

  let hello = &s[0..5];
  let world = &s[0..5];

  slice_fn(hello);
  slice_fn(world);
}
*/

fn refcells() {
  let x = RefCell::new(3);
  let w = &x;

  {
    let mut y = x.borrow_mut();
    *y += 1;
  }

  {
    let mut z = x.borrow_mut();
    *z += 1;
  }

  println!("{:?} {:?}", x, w);
}

fn main() {
  moving();
  copy();
  slices();
  refcells();

  1;
}
