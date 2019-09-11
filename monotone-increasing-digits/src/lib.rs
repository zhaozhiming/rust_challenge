fn num_to_vec(n: i32) -> Vec<i32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

fn vec_to_num(v: Vec<i32>) -> i32 {
    let n_str: String = v.iter().map(|x| x.to_string()).collect();
    n_str.parse().unwrap()
}

pub fn monotone_increasing_digits(n: i32) -> i32 {
    let mut n_vec = num_to_vec(n);
    let n_len = n_vec.len();
    let mut result = vec![];
    // 从右边往左边遍历
    for i in (0..n_len).rev() {
        // 遍历到最后一位，则直接添加数字并退出
        if i == 0 {
            result.push(n_vec[i]);
            break;
        }
        // 如果左边数字 > 右边数字，则将左边数字 -1，右边的其他数字置为 9，否则添加该位数字
        if n_vec[i - 1] > n_vec[i] {
            n_vec[i - 1] = n_vec[i - 1] - 1;
            result.push(9);
            // 把右边的数字都置为9
            result = result.iter().map(|_| 9).collect();
        } else {
            result.push(n_vec[i]);
        }
    }

    // 因为结果数组是从低位往高位存储的，所以需要反转一下
    result.reverse();
    return vec_to_num(result);
}

// 老实人算法，请忽略
pub fn monotone_increasing_digits_first(n: i32) -> i32 {
    let mut result = n;
    loop {
        let n_vec = num_to_vec(result);
        println!("vec: {:?}", n_vec);
        let flag = n_vec.iter().fold(0, |f, x| {
            println!("f: {:?}, x: {:?}, compare: {:?}", f, x, &f > x);
            if &f > x {
                10
            } else {
                *x
            }
        });
        println!("flag: {:?}", flag);
        if flag < 10 {
            return vec_to_num(n_vec);
        } else {
            result = result - 1;
        }

        if result < 0 {
            break;
        }
    }
    n
}
