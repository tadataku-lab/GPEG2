use std::cell::Cell;
use std::cell::RefCell;

/**
const SYMBOLS: [&'static str; 3] = ["S", "A", "B"];

fn e1() -> Box<Fn(& ParserContext) -> bool> {
    choice(nonterm(1, e2(), succ()), nonterm(2, e3(), succ()), succ())
}

fn e2() -> Box<Fn(& ParserContext) -> bool> {
    ch('c', succ())
}

fn e3() -> Box<Fn(& ParserContext) -> bool> {
    ch('a', ch('b', succ()))
}
*/

const SYMBOLS: [&'static str; 2] = ["S", "S'"];

fn dispatch(num: usize) -> Box<Fn(& ParserContext) -> bool> {
    match num {
        0 => e0(),
        1 => e1(),
        _ => panic!("this number can't dispatch {} ", num)
    }
}

fn e0() -> Box<Fn(& ParserContext) -> bool> {
    choice(nonterm(1, nonterm(0, nonterm(0, succ()))), nonterm(1, succ()), succ())
}

fn e1() -> Box<Fn(& ParserContext) -> bool> {
    choice(ch('b', nonterm(1, succ())), ch('b', succ()), succ())
}

/**
const SYMBOLS: [&'static str; 1] = ["S"];

fn dispatch(num: usize) -> Box<Fn(& ParserContext) -> bool> {
    match num {
        0 => e0(),
        _ => panic!("this number can't dispatch {} ", num)
    }
}

fn e0() -> Box<Fn(& ParserContext) -> bool> {
    choice(ch('b', nonterm(0, succ())), ch('b', succ()), succ())
}
*/

#[derive(Debug)]
struct ParserContext{
    input: Vec<u8>,
    pos: Cell<i32>,
    tree: RefCell<Vec<Tree>>
}

#[derive(Debug, Clone)]
pub enum Tree{
    Leaf(char),
    Node{sym: usize, child: Vec<Tree>}
}

impl Tree{
    fn to_string(&self) -> String {
        match self {
            &Tree::Leaf(c) => format!("{}", c),
            &Tree::Node{ref sym, ref child} => format!("[{}{}]", SYMBOLS[*sym], child.iter().fold("".to_string(), |ts, t| format!("{} {}",ts, t.to_string()))),
        }
    }
}

fn main() {
    let p = ParserContext{ input: String::from("bbb").into_bytes(), pos: Cell::new(0), tree: RefCell::new(Vec::new())};
    //println!("{}", char1(&mut p, 'a' as u8) && char1(&mut p, 'b' as u8))
    println!("{}", e0()(&p));
    println!("{}", Tree::Node{sym: 0, child: p.tree.into_inner()}.to_string());
}

fn make_leaf(c: char, p: & ParserContext) -> bool{
    let mut mut_child = p.tree.borrow_mut();
    mut_child.push(Tree::Leaf(c));
    true
}

fn make_node(sym: usize, prev: RefCell<Vec<Tree>>, p: & ParserContext) -> bool {
    {
        let mut mut_prev = prev.borrow_mut();
        let moved_tree = p.tree.clone();
        mut_prev.push(Tree::Node{sym: sym, child: moved_tree.into_inner()});
    }
    {
        let mut mut_child = p.tree.borrow_mut();
        mut_child.clear();
        mut_child.append(&mut prev.into_inner()); 
    }
    true
}

fn next1(p: & ParserContext) -> bool{
    p.pos.set(p.pos.get() + 1);
    true
}

#[allow(unused_variables)]
fn succ() -> Box<Fn(& ParserContext) -> bool> {
    Box::new(move |p: & ParserContext| -> bool {
        true
    })
}

fn ch(c: char, e: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
    Box::new(move |p: & ParserContext| -> bool {
        if p.pos.get() as usize >= p.input.len() {
            false
        }else {
           if p.input[p.pos.get() as usize] == c as u8 { make_leaf(c, p) && next1(p) && e(p) } else {false} 
        }
    })
}

fn nonterm(symbol: usize, e: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
    Box::new(move |p: & ParserContext| -> bool {
        let prev_tree = p.tree.clone();
        p.tree.borrow_mut().clear();
        if dispatch(symbol)(p) {make_node(symbol, prev_tree, p) && e(p)} else {false}
    })
}

fn choice(left: Box<Fn(& ParserContext) -> bool>, right: Box<Fn(& ParserContext) -> bool>, e: Box<Fn(& ParserContext) -> bool>) -> Box<Fn(& ParserContext) -> bool> {
    Box::new(move |p: & ParserContext| -> bool {
        let back_pos = p.pos.get();
        if left(p) { e(p) } else{ 
            p.pos.set(back_pos);
            right(p) && e(p)
        } 
    })
}
