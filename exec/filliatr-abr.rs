/* arbres binaires de recherche */

struct BST {
  value: i32,
  sub: Vec<BST> // de taille 0 ou 2
}

fn null() -> BST {
    let r = BST { value: 42, sub: vec![] };
    r
}

fn is_null(a: & BST) -> bool {
    a.sub.len() == 0
}

fn left(a: & BST) -> & BST { & a.sub[0] }
fn right(a: & BST) -> & BST { & a.sub[1] }

fn left_mut(a: &mut BST) -> &mut BST { &mut a.sub[0] }
fn right_mut(a: &mut BST) -> &mut BST { &mut a.sub[1] }

fn leaf(v: i32) -> BST {
    let r = BST { value: v, sub: vec![null(), null()] };
    r
}

fn insert(a: &mut BST, x: i32) {
  if x == a.value { return; }
  if x < a.value {
    if is_null(left(a))
      { a.sub[0] = leaf(x); }
    else
      { insert(left_mut(a), x); }
  } else {
    if is_null(right(a))
      { a.sub[1] = leaf(x); }
    else
      { insert(right_mut(a), x); }
  }
}

fn contient(a: & BST, x: i32) -> bool {
  if x == a.value { return true; }
  if x < a.value && !is_null(left(a)) { return contient(left(a), x); }
  if !is_null(right(a)) { return contient(right(a), x); }
  return false;
}

fn print_bool(b: bool) {
    if b { print!("true\n") } else { print!("false\n") }
}

fn print_int(x: i32) {
    if x < 0 { print!("-") }
    else if x > 0 { print!("+") }
    else { print!("0") }
}

fn print(a: & BST) {
    print!("(");
    if !is_null(left(a)) { print(left(a)) }
    print_int(a.value);
    if !is_null(right(a)) { print(right(a)) }
    print!(")");
}

fn main() {
    let mut dict = leaf(1);
    let d = &mut dict;
    insert(d, 17);
    insert(d, -5);
    insert(d, 8);
    print_bool(contient(d, -5));
    print_bool(contient(d, 0));
    print_bool(contient(d, 17));
    print_bool(contient(d, 3));
    insert(d, 42);
    insert(d, 8);
    insert(d, -1000);
    insert(d, 0);
    print(d); print!("\n")
}
