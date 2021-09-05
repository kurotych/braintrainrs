const BLTR: i32 = -1;
const EMPTY: i32 = 0;
const BRTL: i32 = 1;
const M: usize = 16; // rows
const N: usize = 16; // columns

fn rules_for_bltr_correct(i: usize, j: usize, matrix_square: &[[i32; 16]; 16]) -> bool {
    if i > 1 && matrix_square[i - 1][j] == BRTL {
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

fn rules_for_brtl_correct(i: usize, j: usize, matrix_square: &[[i32; 16]; 16]) -> bool {
    if j > 1 && matrix_square[i][j - 1] == BLTR {
        return false;
    }

    if j > 1 && i > 1 && matrix_square[i - 1][j - 1] == BRTL {
        return false;
    }

    if i > 1 && matrix_square[i - 1][j] == BLTR {
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

    return false;
}

// (i,j) = (row, column) position
fn is_diagonal_correct(i: usize, j: usize, matrix_square: &[[i32; 16]; 16]) -> bool {
    let diagonal_type = matrix_square[i][j];
    match diagonal_type {
        BLTR => rules_for_bltr_correct(i, j, matrix_square),
        BRTL => rules_for_brtl_correct(i, j, matrix_square),
        _ => true,
    }
}

fn extend(perm: &mut Vec<u32>, i: usize, j: usize) {
    if if i == M && j == N {
        println!("Solution {:?}x{:?} table: {:?}", n, n, perm);
        return;
    }
    for i in 0..n {
        if !perm.contains(&(i as u32)) {
            perm.push(i as u32);
            if is_solution(perm) {
                extend(perm, n)
            }
            perm.pop();
        }
    }
}

fn main() {
    let mut x = [[EMPTY; N]; M];
    println!("{:?}", x[11][4]);

    is_diagonal_correct(0, 0, &x);
    // println!("{:?}", state);
}
