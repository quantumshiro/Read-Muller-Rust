use iter_tools::Itertools;

fn eval_z(m: usize, z: usize, f: &Vec<u8>) -> u8 {
    if f == &vec![0; m] {
        return 1;
    } else {
        for index in 0..m {
            if f[index] == 1 && (z >> index) & 1 == 0 {
                return 0;
            }
        }
        return 1;
    }
}

fn eval(m: usize, f: &Vec<u8>) -> Vec<u8> {
    let mut eval_vec = vec![];
    for z in (0..(1 << m)).rev() {
        eval_vec.push(eval_z(m, z, f));
    }

    eval_vec
}

fn make_rm_code_generator_matrix(m: usize, r: usize) -> Vec<Vec<u8>> {
    let mut generator_matrix = vec![];
    for deg in 0..=r {
        for f_deg_list in (0..m).combinations(deg) {
            let mut f = vec![0; m];
            for f_deg_index in f_deg_list {
                f[f_deg_index] = 1;
            }
            generator_matrix.push(eval(m, &f));
        }
    }

    generator_matrix
}

fn make_rm_code(m: usize, r: usize) -> Vec<Vec<u8>> {
    let mut rm_code = vec![];
    let generator_matrix = make_rm_code_generator_matrix(m, r);
    let rm_code_dim = generator_matrix.len();
    for index in 0..(1 << rm_code_dim) {
        let mut codeword = vec![0; rm_code_dim];
        for i in 0..rm_code_dim {
            for j in 0..rm_code_dim {
                codeword[i] += ((index >> j) & 1) as u8 * generator_matrix[j][i];
            }
            codeword[i] %= 2;
        }
        rm_code.push(codeword);
    }

    rm_code
}



fn main() {
    let rm_code = make_rm_code(4, 2);
    for codeword in rm_code {
        println!("{:?}", codeword);
    }

    // 凖同型RM符号の生成行列を作る
    // let generator_matrix = make_rm_code_generator_matrix(4, 2);
    // for row in generator_matrix {
    //     println!("{:?}", row);
    // }

}
