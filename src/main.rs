const SIZE: usize = 9 * 9;

fn main() {
    println!("Sudoku Solver");

    let mut array: [u8; SIZE] = [0; SIZE];

    array[40] = 1;

    print_sudoku(&array)
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
