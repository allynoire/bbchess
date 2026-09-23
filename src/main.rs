use std::{cmp::min, ops::{Deref, DerefMut}, str::Lines, sync::Arc};

use bitboards::bitops::*;



fn main() {

    let rook_atk = atk_table(rook_atk_bb);
    let bish_atk = atk_table(bish_atk_bb);
    let quee_atk = atk_table(|i| rook_atk_bb(i) & bish_atk_bb(i));

    dump_table(&rook_atk);
}

fn dump_bb(bb: u64, pos: usize) {
    // the board is displayed flipped upside 
    println!("[{pos:02}] 0x{bb:016x}");
    println!("{}", bb_stringify(bb, pos));
}

fn dump_table(table: &[u64]) {
    for chunk in table.iter()
        .enumerate()
        .map(|(i, bb)| bb_stringify(*bb, i))
        .collect::<Vec<String>>()
        .chunks_exact(8)
    {
        let lines: Vec<Vec<&str>> = chunk.iter()
            .map(|s| s.lines().collect())
            .collect();
        
        for i in 0..8 { // line index
            for j in 0..8 { // chunk index
                print!("{}", lines[j][i]);
                if j < 7 {
                    print!("    ");
                }
            }
            println!("");
        }
        print!("\n\n")
    }
}

fn bb_stringify(bb: u64, pos: usize) -> String {
    let mut s = String::new();
    for y in 0..8 {
        for x in 0..8 {
            let i = x + (7-y) * 8;
            if bb & 1 << i != 0 {
                s.push('x');
            }
            else if i == pos {
                s.push('0');
            }
            else {
                s.push('.');
            }
            s.push(if x == 7 { '\n' } else { ' ' });
        }
    }
    s
}


fn atk_table<F>(f: F) -> [u64; 64]
where F: Fn(usize) -> u64
{
    let mut table = [0u64; 64];
    for i in 0..64 { table[i] = f(i) }
    table
}


/// computes the rook attacks for the given position
fn rook_atk_bb(i: usize) -> u64 {
    let mut bb = 0;

    // set position in rank
    bb |= 0xff << (i / 8) * 8;
    // set positions in file
    bb |= 0x0101010101010101 << (i % 8);
    // clear position
    bb &= !(1 << i);

    bb
}

/// computes the bishop attacks for the given position
fn bish_atk_bb(i: usize) -> u64 {
    let mut bb = 0;
    let x = i % 8;
    let y = i / 8;

    // towards top right (+9)
    for j in 0..min(8-x, 8-y) { 
        bb |= 1 << (i + j*9);
    }

    // towards bottom left (-9)
    for j in 0..min(1+x, 1+y) { 
        bb |= 1 << (i - j*9);
    }

    // towards top left (+7)
    for j in 0..min(1+x, 8-y) {
        bb |= 1 << (i + j*7);
    }

    // towards bottom right (-7)
    for j in 0..min(8-x, 1+y) {
        bb |= 1 << (i - j*7);
    }

    bb &= !(1 << i);

    bb
}