const BLTR: i32 = -1;
const EMPTY: i32 = 0;
const BRTL: i32 = 1;
const M: usize = 5; // rows
const N: usize = 5; // columns

fn rules_for_bltr_correct(i: usize, j: usize, matrix_square: &[[i32; M]; N]) -> bool {
    if i > 0 && matrix_square[i - 1][j] == BRTL {
        return false;
    }

    if j < N - 1 && matrix_square[i][j + 1] == BRTL {
        return false;
    }

    if i > 0 && j < N - 1 && matrix_square[i - 1][j + 1] == BLTR {
        return false;
    }

    if i < M - 1 && matrix_square[i + 1][j] == BRTL {
        return false;
    }

    if i < M - 1 && j > 0 && matrix_square[i + 1][j - 1] == BLTR {
        return false;
    }

    if j > 0 && matrix_square[i][j - 1] == BRTL {
        return false;
    }

    return true;
}

fn rules_for_brtl_correct(i: usize, j: usize, matrix_square: &[[i32; M]; N]) -> bool {
    if j > 0 && matrix_square[i][j - 1] == BLTR {
        return false;
    }

    if j > 0 && i > 0 && matrix_square[i - 1][j - 1] == BRTL {
        return false;
    }

    if i > 0 && matrix_square[i - 1][j] == BLTR {
        return false;
    }

    if i < M - 1 && matrix_square[i + 1][j] == BLTR {
        return false;
    }

    if j < N - 1 && matrix_square[i][j + 1] == BLTR {
        return false;
    }

    if i < M - 1 && j < N - 1 && matrix_square[i + 1][j + 1] == BRTL {
        return false;
    }

    return true;
}

fn non_empty_count(matrix_square: &[[i32; M]; N]) -> usize {
    let mut counter = 0;
    for i in 0..M {
        for j in 0..N {
            if matrix_square[i][j] != EMPTY {
                counter += 1;
            }
        }
    }
    counter
}

// (i,j) = (row, column) position
fn is_diagonal_correct(matrix_square: &[[i32; N]; M], i: usize, j: usize) -> bool {
    let diagonal_type = matrix_square[i][j];
    match diagonal_type {
        BLTR => return rules_for_bltr_correct(i, j, matrix_square),
        BRTL => return rules_for_brtl_correct(i, j, matrix_square),
        _ => return true,
    }
}

fn extend(matrix_square: &mut [[i32; N]; M], i: usize, j: usize, var: i32) {
    matrix_square[i][j] = var;
    if !is_diagonal_correct(matrix_square, i, j) {
        return;
    }

    if i == M - 1 && j == N - 1 {
        if non_empty_count(matrix_square) == 16 {
            print_matrix(matrix_square);
        }
        return;
    }

    let (i2, j2) = if j < N - 1 {
        (i, j + 1)
    } else if j == M - 1 && i != N - 1 {
        (i + 1, 0)
    } else {
        panic!("WTF?!");
    };

    extend(matrix_square, i2, j2, BLTR);
    extend(matrix_square, i2, j2, BRTL);
    extend(matrix_square, i2, j2, EMPTY);
}

fn print_matrix(matrix_square: &[[i32; N]; M]) {
    for i in 0..M {
        for j in 0..N {
            print!("{:?} ", matrix_square[i][j])
        }
        print!("\n");
    }
    print!("\n");
}

fn main() {
    let mut x = [[EMPTY; N]; M];
    extend(&mut x, 0, 0, BLTR);
}
