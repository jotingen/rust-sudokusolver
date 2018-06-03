use std::env;

const SIZE: usize = 9 * 9;

fn main() {
    println!("Sudoku Solver");

    let args: Vec<String> = env::args().collect();

    if args[1].chars().count() != SIZE {
        panic!("Hint string must be {} characters", SIZE);
    }

    let mut hint: [u8; SIZE] = [0; SIZE];
    let mut i = 0;
    for c in args[1].chars() {
        hint[i] = c.to_digit(10).unwrap() as u8;
        i += 1
    }

    print_sudoku(&hint);

    print_sudoku(&solve(&hint));
}

fn solve(hint: &[u8; SIZE]) -> [u8; SIZE] {
    let mut a: [u8; SIZE] = hint.clone();
    let mut i = 0;
    while i < SIZE {
        //if the hint was defined, skip this cell
        if hint[i] != 0 {
            i += 1;
            continue;
        }

        //If the cell is already a 9, go back
        if a[i] == 9 {
            a[i] = 0;
            i -= 1;
            //Keep decrementing if we backtrack to a hint
            while hint[i] != 0 {
                i -= 1;
            }
            continue;
        }

        //Increment the cell
        a[i] += 1;

        //Check
        let mut fail = false;

        //Horizontal
        let mut h = i / 9 * 9;
        while h < i / 9 * 9 + 9 {
            if h != i && a[h] == a[i] {
                fail = true;
            }
            h += 1
        }

        //Vertical
        let mut v = i % 9;
        while v < i % 9 + 9 * 9 {
            if v != i && a[v] == a[i] {
                fail = true;
            }
            v += 9;
        }

        //Block
        let mut b = 0;
        while b < SIZE {
            if i / 27 * 3 + (i / 3) % 3 == b / 27 * 3 + (b / 3) % 3 {
                if b != i && a[b] == a[i] {
                    fail = true;
                }
            }
            b += 1;
        }

        if !fail {
            i += 1;
        }
    }
    return a;
}

fn print_sudoku(array: &[u8; SIZE]) {
    let mut counter = 0;
    for x in array.iter() {
        if counter % 3 == 0 && counter % 27 != 0 {
            print!("|");
        }
        if counter % 9 == 0 && counter % 27 != 0 {
            println!();
            print!("|");
        }
        if counter % 27 == 0 {
            if counter != 0 {
                println!("|");
            } else {
                println!();
            }
            println!("+---------+---------+---------+");
            print!("|");
        }
        if *x == 0 {
            print!("   ")
        } else {
            print!(" {} ", x);
        }
        counter += 1
    }
    println!("|");
    println!("+---------+---------+---------+");
}
