use std::{cmp::min, ops::{Deref, DerefMut}, str::Lines, sync::Arc};

use bitboards::bitops::*;


/*
8  38 39 3a 3b 3c 3d 3e 3f 
7  30 31 32 33 34 35 36 37 
6  28 29 2a 2b 2c 2d 2e 2f 
5  20 21 22 23 24 25 26 27 
4  18 19 1a 1b 1c 1d 1e 1f 
3  10 11 12 13 14 15 16 17 
2  08 09 0a 0b 0c 0d 0e 0f 
1  00 01 02 03 04 05 06 07

   a  b  c  d  e  f  g  h

*/

fn main() {


    let mut occ = 0u64
        | 1 << 0x20
        | 1 << 0x38
        | 1 << 0x30 
        | 1 << 0x31 
        | 1 << 0x2a 
        | 1 << 0x2b 
        | 1 << 0x3e;

        
        dump_bb(occ, 0xff);
        // dump_bb(ray, 0x00);
        
    // ray hit
    for i in 0..8 {
        let ray = ray_up(i);
        let hit: u64 = (occ & ray).lowest_one().map(|j| 1 << j).unwrap_or(0);
        // dump_bb(hit, 0xff);
    }


    // ray until and with hit
    for i in 0x20..0x28 {
        let mut ray = ray_up(i);
        if let Some(i) = (occ & ray).lowest_one() {
            while let Some(j) = ray.highest_one() && j > i {
                ray.pop_highest_one();
            }
        }
        dump_bb(ray, i);
    }




    let rook_atk = atk_table(rook_atk_bb);
    let bish_atk = atk_table(bish_atk_bb);
    let quee_atk = atk_table(|i| rook_atk_bb(i) & bish_atk_bb(i));

    let ray_up = atk_table(ray_up);

    // dump_table(&ray_up);
}

fn dump_pos() {
    for y in 0..8 {
        for x in 0..8 {
            let i = x + (7-y) * 8;
            print!("{i:02x} ")
        }
        println!()
    }
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


fn ray_up(i: usize) -> u64 {
    let mut bb = 0;
    bb |= 0x0101010101010100 << (i % 8) << (i & !0x7);
    bb
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