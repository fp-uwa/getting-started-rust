/* Rust has no concept of floating point equality because floating
 * point numbers are not stable. You need to opt-in to using epsilon
 * based equality and this is not in the standard library, so we do that
 * here */
#[macro_use]
extern crate approx;

/* Necessary imports for what we want to do */
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::prelude::*;

/* #[derive] is a handy little macro for introspecting our
 * type and automatically implementing certain type traits
 * with sensible default implementations. In Rust, there's a
 * strong mantra that you "don't pay for what you don't use".
 *
 * Unlike other languages where you can just print() an object
 * and it will probably print something sensible because it
 * generates the code to do so via runtime introspection, Rust
 * has no runtime and it will not generate that code for you
 * ahead of time unless you explicitly ask it to. That's what
 * the Debug trait does.
 *
 * Same thing with Clone - this trait signals that we support
 * making a complete copy of the object and since the implementation
 * of how that might work would vary between different objects,
 * the code for that is not generated for you and you have
 * to either opt-in to using the default implementation
 * or implement it yourself.
 */
#[derive(Debug, Clone)]
/* <'a> Here is what is called a lifetime parameter. The two
 * str elements of Batsman are read only, so we can tell the
 * compiler that we either want them to sit in the .data
 * section of the binary during the lifetime of the program,
 * or we can tell the compiler that they should be allocated
 * on the stack alongside Batsman. This is what what the
 * default lifetime parameter does if we just create a
 * new Batsman without specifying where we want it to go. */
struct Batsman<'a> {
  initials: &'a str,
  surname: &'a str,
  runs: u32,
  average: f32
}

/* This is an implementation for a trait. If a struct supports
 * certain traits as indicated by whether it implements that
 * trait, then certain operations will be supported on the type.
 *
 * For instance, this type supports the PartialEq trait, standing
 * for "partial equality". This means that for some of its
 * attributes we can guarantee that the following properties:
 *
 * (1) a == a (reflexive),
 * (2) b == a -> a == b and vice-versa (symmetric)
 * (3) b == a, a == c -> a == c (transitive)
 *
 * Note that we can't compare equality by just deriving the trait
 * since Rust doesn't like to directly compare equality between
 * floating point numbers as there is no "right way to do it". Instead
 * it is up to you. We are using the within-epsilon method.
 */
impl<'a> PartialEq for Batsman<'a> {
  fn eq(&self, other: &Batsman) -> bool {
    return
      self.initials == other.initials &&
      self.surname == other.surname &&
      self.runs == other.runs &&
      relative_eq!(self.average, other.average);
  }
}

/* Empty Eq means we do not support full equavilence */
impl<'a> Eq for Batsman<'a> {
}

/* Batsman supports Partial Ordering. Which means that on one axis
 * the following properties are satisfied:
 *
 * (1) a == a (reflexive),
 * (2) b >= a and a <= b -> a == b and vice-versa (antisymmetric)
 * (3) b > a, a > c -> b > c (transitive)
 *
 * We are partially ordered if we are ordred, so we can always
 * return a value with an ordering. */
impl<'a> PartialOrd for Batsman<'a> {
  fn partial_cmp(&self, other: &Batsman) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

/* Batsman supports Total Ordering. Which means that on one axis
 * the following properties are satisfied:
 *
 * (1) a == a (reflexive),
 * (2) b >= a and a <= b -> a == b and vice-versa (antisymmetric)
 * (3) b >= a or a <= b (transitive)
 */
impl<'a> Ord for Batsman<'a> {
  fn cmp(&self, other: &Batsman) -> Ordering {
    self.runs.cmp(&other.runs)
  }
}

/* Rust's sort_by mutates the vector, we want to return a copy.
 *
 * Note that the vector's element type must also be cloneable. */
fn sorted<T: Clone, F>(x: Vec<T>, cmp: F) -> Vec<T>
  where F: FnMut(&T, &T) -> Ordering
{
  let mut y = x.clone();
  y.sort_by(cmp);
  return y;
}

fn main() {
  /* Coerces all commandline argumenst to strings */
  let args = env::args().collect::<Vec<String>>();
  /* Basically panics if the file was not found */
  let mut f = File::open(&args[1]).expect("File not found");

  /* Bizzarely, the return value for read_to_string is a Result for
   * the number of read bytes, with contents as a mutable outparam. No
   * idea why this was done, but we have to live with it */
  let mut contents = String::new();
  /* read_to_string returns Result, so we need to unwrap it and handle
   * each case */
  match f.read_to_string (&mut contents) {
    Err(x) => panic!("Failed to read file, {:?}", x),
    Ok(x) => x
  };

  /* Remember, we are composing sorted over the chain here */
  let batsmen = sorted(contents.lines().map(|l| {
    /* Need to explicitly trim each element of the split string, otherwise
     * parse() will get upset */
    let v = l.split(",").map(|x| x.trim()).collect::<Vec<&str>>();
    let name = v[0].split(" ").collect::<Vec<&str>>();

    /* Stack allocates and moves the result */
    return Batsman {
      initials: name[0],
      surname: name[1],
      /* Need to handle error cases */
      runs: match v[1].parse::<u32>() {
        Ok(x) => x,
        Err(_) => panic!("Expected second item to be an u32")
      },
      average: match v[2].parse::<f32>() {
        Ok(x) => x.round(),
        Err(_) => panic!("Expected third item to be an f32")
      },
    };
  }).filter(|b| {
    /* .chars() returns an iterator of characters, .next() will just get
     * the next, i.e first one */
    match b.surname.chars().next() {
      Some('C') => true,
      _ => false
    }
    /* Below, we are not automatically a vector, so collect the
     * iterable into one */
  }).collect::<Vec<Batsman>>(), |lhs, rhs| rhs.cmp(lhs));

  println!("{:?}", batsmen);
}