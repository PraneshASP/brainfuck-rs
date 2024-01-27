use core::panic;
use std::{env, fs, io};

struct Op {
    operator: Ix,
    operhand: usize,
}

enum Ix {
    IncDp,
    DecDp,
    IncVal,
    DecVal,
    Read,
    Write,
    JmpFwd,
    JmpBck,
}

fn parse_source(source: String) -> Vec<Op> {
    let mut pc = 0;
    let mut jump_pc = 0;
    let mut jump_stack: Vec<usize> = Vec::new();
    let mut ops: Vec<Op> = Vec::new();
    for c in source.chars() {
        match c {
            '>' => ops.push(Op {
                operator: Ix::IncDp,
                operhand: 0,
            }),
            '<' => ops.push(Op {
                operator: Ix::DecDp,
                operhand: 0,
            }),
            '+' => ops.push(Op {
                operator: Ix::IncVal,
                operhand: 0,
            }),
            '-' => ops.push(Op {
                operator: Ix::DecVal,
                operhand: 0,
            }),
            ',' => ops.push(Op {
                operator: Ix::Read,
                operhand: 0,
            }),
            '.' => ops.push(Op {
                operator: Ix::Write,
                operhand: 0,
            }),
            '[' => {
                ops.push(Op {
                    operator: Ix::JmpFwd,
                    operhand: 0,
                });
                jump_stack.push(pc);
            }
            ']' => {
                if jump_stack.len() == 0 {
                    panic!("Compilation error, cannot jump");
                }
                jump_pc = jump_stack[jump_stack.len() - 1];
                jump_stack.pop();
                ops.push(Op {
                    operator: Ix::JmpBck,
                    operhand: jump_pc,
                });
                ops[jump_pc].operhand = pc;
            }
            _ => pc -= 1,
        }
        pc += 1;
    }
    if jump_stack.len() != 0 {
        panic!("Compilation error: jump_stack!=0");
    }
    ops
}

fn execute_compiled(compiled_bf: Vec<Op>) {
    let mut data: [u16; 65535] = [0; 65535];
    let mut data_ptr = 0;
    let mut i = 0;
    while i < compiled_bf.len() {
        match compiled_bf[i].operator {
            Ix::IncDp => {
                data_ptr += 1;
            }
            Ix::DecDp => {
                data_ptr -= 1;
            }
            Ix::IncVal => {
                data[data_ptr] = data[data_ptr].wrapping_add(1);
            }
            Ix::DecVal => {
                data[data_ptr] = data[data_ptr].wrapping_sub(1);
            }
            Ix::Read => {
                println!("Enter input:");
                data[data_ptr] = io::read_to_string(io::stdin())
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap();
            }
            Ix::Write => {
                print!("{}", char::from_u32(data[data_ptr] as u32).unwrap());
            }
            Ix::JmpFwd => {
                if data[data_ptr] == 0 {
                    i = compiled_bf[i].operhand;
                    continue;
                }
            }
            Ix::JmpBck => {
                if data[data_ptr] > 0 {
                    i = compiled_bf[i].operhand;
                    continue;
                }
            }
        }
        i += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage example: cargo run <BF_FILENAME>");
    }

    let source = fs::read_to_string(args[1].clone()).unwrap();
    let compiled = parse_source(source);
    execute_compiled(compiled);
}
