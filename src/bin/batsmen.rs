#[macro_use]
extern crate approx;

use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Batsman<'a> {
  initials: &'a str,
  surname: &'a str,
  runs: u32,
  average: f32
}

impl<'a> PartialEq for Batsman<'a> {
  fn eq(&self, other: &Batsman) -> bool {
    return
      self.initials == other.initials &&
      self.surname == other.surname &&
      self.runs == other.runs &&
      relative_eq!(self.average, other.average);
  }
}

impl<'a> Eq for Batsman<'a> {
}

impl<'a> PartialOrd for Batsman<'a> {
  fn partial_cmp(&self, other: &Batsman) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<'a> Ord for Batsman<'a> {
  fn cmp(&self, other: &Batsman) -> Ordering {
    self.runs.cmp(&other.runs)
  }
}

fn sorted<T: Clone, F>(x: Vec<T>, cmp: F) -> Vec<T>
  where F: FnMut(&T, &T) -> Ordering
{
  let mut y = x.clone();
  y.sort_by(cmp);
  return y;
}

fn main() { 
  let args = env::args().collect::<Vec<String>>();
  let mut f = File::open(&args[1]).expect("File not found");
  let mut contents = String::new();
  match f.read_to_string (&mut contents) {
    Err(x) => panic!("Failed to read file, {:?}", x),
    Ok(x) => x
  };

  let batsmen = sorted(contents.lines().map(|l| {
    let v = l.split(",").map(|x| x.trim()).collect::<Vec<&str>>();
    let name = v[0].split(" ").collect::<Vec<&str>>();

    return Batsman {
      initials: name[0],
      surname: name[1],
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
    match b.surname.chars().next() {
      Some('C') => true,
      _ => false
    }
  }).collect::<Vec<Batsman>>(), |lhs, rhs| rhs.cmp(lhs));

  println!("{:?}", batsmen);
}