use std::env;
use std::fs::File;
use std::process::exit;


use std::io::{self, Read, Error};
use std::iter::FromIterator;

fn create_buffer(buffer: &mut String) -> Result<(), Error>{
    if let Some(arg) = env::args().nth(1) {
        let mut f = File::open(arg)?;
        f.read_to_string(buffer)?;
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(buffer)?;
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
enum BFCommand {
    Inc,
    Dec,
    Right,
    Left,
    Put,
    Get,
    Bra(i32),
    Ket(i32),
    Debug
}

use BFCommand::*;
fn parse(instructions: Vec<char>) -> Vec<BFCommand> {
    let mut compiled_instructions = vec![];
    let mut bra_stack = vec![];
    for (i, &c) in instructions.iter().enumerate() {
        match c {
            '+' => compiled_instructions.push(Inc),
            '-' => compiled_instructions.push(Dec),
            '>' => compiled_instructions.push(Right),
            '<' => compiled_instructions.push(Left),
            '.' => compiled_instructions.push(Put),
            ',' => compiled_instructions.push(Get),
            '[' => {
                bra_stack.push(i);
                compiled_instructions.push(Bra(-1))
            },
            ']' => {
                if let Some(bra_loc) = bra_stack.pop() {
                    if let Some(matching_bra) = compiled_instructions.get_mut(bra_loc) {
                        *matching_bra = Bra(i as i32);
                    }
                    compiled_instructions.push(Ket(bra_loc as i32))
                } else {
                    eprintln!("ERROR: Unmatched ket");
                    exit(1);
                }
            },
            '#' => compiled_instructions.push(Debug),
            _ => ()
        }
    }

    for ci in compiled_instructions.iter() {
        match ci {
            &Bra(-1) => {
                eprintln!("Error: Unmatched bra");
                exit(1);
            },
            _ => ()
        }
    }

    compiled_instructions
}

fn interpret(commands: Vec<BFCommand>) {
    let mut pc = 0;
    let mut tape: [u8; 3000] = [0; 3000];
    let mut i = 0;

    while let Some(command) = commands.get(pc) {
        match *command {
            Inc => tape[i] = tape[i].wrapping_add(1),
            Dec => tape[i] = tape[i].wrapping_sub(1),
            Right => i += 1,
            Left => i -= 1,
            Put => print!("{}", tape[i] as char),
            Get => tape[i] = if let Some(Ok(b)) = io::stdin().bytes().next() {
                b
            } else {
                panic!("no input?");
            },
            Bra(j) => if tape[i] == 0 {
                pc = j as usize
            },
            Ket(j) => if tape[i] != 0 {
                pc = j as usize
            },
            Debug => println!("{:?}", &tape[i..i+20]),
        }
        pc += 1;
    }
        
}

fn main() {
    let mut buffer = String::new();
    
    if let Err(e) =  create_buffer(&mut buffer) {
        eprintln!("Problem reading input: {}", e);
        exit(1);
    }

    let instructions = Vec::from_iter(buffer.chars());
    let parsed = parse(instructions);
    println!("{:?}", parsed);
    interpret(parsed);
}


#[test]
fn instruction_set() {
    let instructions = vec!['+', '-', '>', '<', '.', ',', '[', ']'];
    let parsed = parse(instructions);
    assert_eq!(vec![Inc, Dec, Right, Left, Put, Get, Bra(7), Ket(6)],
               parsed);
}

