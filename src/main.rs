mod common;
mod error;
mod opt;

use crate::common::*;

fn run(args: &[&str]) -> Result<Vec<isize>, Error> {
  let (flags, program): (Vec<&str>, Vec<&str>) = args.iter().partition(|f| f.starts_with("--"));
  let opt = Opt::from_iter(flags);

  let mut stack = Vec::new();
  for word in program {
    match word {
      "add" => add(&mut stack)?,
      "sub" => sub(&mut stack)?,
      "mul" => mul(&mut stack)?,
      "div" => div(&mut stack)?,
      ":add" => add_all(&mut stack)?,
      ":sub" => sub_all(&mut stack)?,
      ":mul" => mul_all(&mut stack)?,
      ":div" => div_all(&mut stack)?,
      "pop" => pop(&mut stack).map(|_| ())?,
      "." => pop_print(&mut stack)?,
      ":." => pop_print_all(&mut stack)?,
      _ => num(&mut stack, word)?,
    }

    if opt.verbose {
      println!("Stack:\t\t{:?}", &stack);
    }
  }
  Ok(stack)
}

fn main() -> Result<(), Error> {
  let mut buffer = std::env::args().collect::<Vec<String>>();
  buffer.remove(0);
  let slice = buffer.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
  run(&slice)?;
  Ok(())
}

fn pop_print_all(stack: &mut Vec<isize>) -> Result<(), Error> {
  while !stack.is_empty() {
    pop_print(stack)?;
  }
  Ok(())
}

fn pop_print(stack: &mut Vec<isize>) -> Result<(), Error> {
  let item = pop(stack)?;
  println!(
    "dec: {}\t\thex: 0x{:x}\t\toct: o{:o}\t\tbin: b{:b}",
    item, item, item, item
  );
  Ok(())
}

fn pop(stack: &mut Vec<isize>) -> Result<isize, Error> {
  match stack.pop() {
    Some(x) => Ok(x),
    None => Err(Error::StackUnderflow),
  }
}

/// Pops top two items off the stack, adds them, and pushes the sum on the stack.
fn add(stack: &mut Vec<isize>) -> Result<(), Error> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let sum = b + a;
  stack.push(sum);
  Ok(())
}

/// Pops top two items off the stack, subtracts them, and pushes the difference on the stack.
fn sub(stack: &mut Vec<isize>) -> Result<(), Error> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let difference = b - a;
  stack.push(difference);
  Ok(())
}

/// Pops top two items off the stack, multiplies them, and pushes the product on the stack.
fn mul(stack: &mut Vec<isize>) -> Result<(), Error> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let product = b * a;
  stack.push(product);
  Ok(())
}

/// Pops top two items off the stack, divides them, and pushes the quotient on the stack.
fn div(stack: &mut Vec<isize>) -> Result<(), Error> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let quotient = b / a;
  stack.push(quotient);
  Ok(())
}

/// Pops all items before `:add` off the stack, adds them, and pushes the sum on the stack.
fn add_all(stack: &mut Vec<isize>) -> Result<(), Error> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let mut sum = b + a;

  while !stack.is_empty() {
    sum += stack.pop().unwrap();
  }
  stack.push(sum);
  Ok(())
}

/// Pops all items before `:sub` off the stack, subtracts them, and pushes the difference on the stack.
fn sub_all(stack: &mut Vec<isize>) -> Result<(), Error> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let mut difference = b - a;

  while !stack.is_empty() {
    difference = stack.pop().unwrap() - difference;
  }
  stack.push(difference);
  Ok(())
}

/// Pops all items before `:mul` off the stack, multiplies them, and pushes the product on the stack.
fn mul_all(stack: &mut Vec<isize>) -> Result<(), Error> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let mut product = b * a;

  while !stack.is_empty() {
    product *= stack.pop().unwrap();
  }
  stack.push(product);
  Ok(())
}

/// Pops all items before `:div` off the stack, divides them, and pushes the quotient on the stack.
fn div_all(stack: &mut Vec<isize>) -> Result<(), Error> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let mut quotient = b / a;

  while !stack.is_empty() {
    quotient = stack.pop().unwrap() / quotient;
  }
  stack.push(quotient);
  Ok(())
}

/// Parse arg as a number and push it onto the stack
fn num(stack: &mut Vec<isize>, arg: &str) -> Result<(), Error> {
  if arg.starts_with("0x") {
    let arg = &arg[2..arg.len()];
    stack.push(isize::from_str_radix(&arg, 16).unwrap());
  } else if arg.starts_with("x") {
    let arg = &arg[1..arg.len()];
    stack.push(isize::from_str_radix(&arg, 16).unwrap());
  } else if arg.starts_with("0o") {
    let arg = &arg[2..arg.len()];
    stack.push(isize::from_str_radix(&arg, 8).unwrap());
  } else if arg.starts_with("o") {
    let arg = &arg[1..arg.len()];
    stack.push(isize::from_str_radix(&arg, 8).unwrap());
  } else if arg.starts_with("0b") {
    let arg = &arg[2..arg.len()];
    stack.push(isize::from_str_radix(&arg, 2).unwrap());
  } else if arg.starts_with("b") {
    let arg = &arg[1..arg.len()];
    stack.push(isize::from_str_radix(&arg, 2).unwrap());
  } else {
    stack.push(arg.parse().unwrap());
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  /* Test helper function and macros */

  fn lex(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
  }

  fn test(text: &str) -> Result<Vec<isize>, Error> {
    run(&lex(text))
  }

  /// Tests intended to succeed
  macro_rules! test {
        {
            name: $name:ident,
            text: $text:expr,
            want: $want:expr $(,)?
        } => {
            #[test]
            fn $name() {
                let have = test($text).expect("Expected success");
                let want = $want.iter().cloned().map(|x| x as isize).collect::<Vec<isize>>();
                assert_eq!(have, want);
            }
        };
    }

  /// Tests intended to fail
  macro_rules! error {
        {
            name: $name:ident,
            text: $text:expr,
            want: $want:expr $(,)?
        } => {
            #[test]
            fn $name() {
                let have = test($text).expect_err("Expected error");
                let want = $want;
                assert_eq!(have, want);
            }
        };
    }

  /// Test lexer
  #[test]
  fn lex_empty() {
    let text = "";
    let have = lex(text);
    let want: Vec<String> = Vec::new();
    assert_eq!(have, want);
  }

  #[test]
  fn lex_blank() {
    let text = "       ";
    let have = lex(text);
    let want: Vec<String> = Vec::new();
    assert_eq!(have, want);
  }

  #[test]
  fn lex_word() {
    let text = "foo";
    let have = lex(text);
    let want = vec!["foo".to_string()];
    assert_eq!(have, want);
  }

  #[test]
  fn lex_args() {
    let text = "1 2 add";
    let have = lex(text);
    let want = vec!["1".to_string(), "2".to_string(), "add".to_string()];
    assert_eq!(have, want);
  }

  /* Test errors */

  // Error when popping on empty stack
  // $ qc pop
  error! {
      name: pop_empty,
      text: "pop",
      want: Error::StackUnderflow,
  }

  // $ qc 1 add
  error! {
      name: add_underflow,
      text: "1 add",
      want: Error::StackUnderflow,
  }

  /* Test add */

  // $qc 1 2 add
  test! {
      name: add_two_args,
      text: "1 2 add",
      want: [3],
  }

  // $qc 1 2 3 add
  test! {
      name: add_three_args,
      text: "1 2 3 add",
      want: [1, 5],
  }

  // $qc 1 2 :add
  test! {
      name: add_all_two_args,
      text: "1 2 :add",
      want: [3],
  }

  // $qc 1 2 3 :add
  test! {
      name: add_all_three_args,
      text: "1 2 3 :add",
      want: [6],
  }

  /* Test sub */

  // $ qc 1 2 sub
  test! {
      name: sub_two_args,
      text: "1 2 sub",
      want: [-1],
  }

  // $ qc 1 2 3 sub
  test! {
      name: sub_three_args,
      text: "1 2 3 sub",
      want: [1, -1],
  }

  // $ qc 1 2 :sub
  test! {
      name: sub_all_two_args,
      text: "1 2 :sub",
      want: [-1],
  }

  // $qc 1 2 3 :sub
  test! {
      name: sub_all_three_args,
      text: "1 2 3 :sub",
      want: [2],
  }

  /* Test mul */

  // $ qc 2 3 mul
  test! {
      name: mul_two_args,
      text: "2 3 mul",
      want: [6],
  }

  // $ qc 1 2 3 mul
  test! {
      name: mul_three_args,
      text: "1 2 3 mul",
      want: [1, 6],
  }

  // $ qc 2 4 :mul
  test! {
      name: mul_all_two_args,
      text: "2 4 :mul",
      want: [8],
  }

  // $ qc 1 2 3 :mul
  test! {
      name: mul_all_three_args,
      text: "1 2 3 :mul",
      want: [6],
  }

  /* Test div */

  // $ qc 9 3 div
  test! {
      name: div_two_args,
      text: "9 3 div",
      want: [3],
  }

  // $ qc 1 6 2 div
  test! {
      name: div_three_args,
      text: "1 6 2 div",
      want: [1, 3],
  }

  // $ qc 9 3 :div
  test! {
      name: div_all_two_args,
      text: "9 3 :div",
      want: [3],
  }

  // $ qc 6 2 1 :div
  test! {
      name: div_all_three_args,
      text: "6 2 1 :div",
      want: [3],
  }

  /* Test miscellaneous binary calcs */

  // $ qc 4 7 9 add 2 8 mul
  test! {
      name: add_three_mul_two,
      text: "4 7 9 add 2 8 mul",
      want: [4, 16, 16],
  }

  // $ qc 4 7 9 add add 2 8 mul mul
  test! {
      name: add_two_mul_two,
      text: "4 7 9 add add 2 8 mul mul",
      want: [320],
  }
}
