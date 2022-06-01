use rand::Rng;


pub fn update_grid(x: [[i32; 15]; 15]) -> [[i32; 15]; 15] {
    let mut new_list = x;
    for idx in 0..x.len() {
        for idx1 in 0..x[0].len() {
            new_list[idx][idx1] = check_neighbors(idx, idx1, x);
        }
    }
    new_list
}

fn check_neighbors(y: usize, x: usize, l: [[i32; 15]; 15]) -> i32 {
    let mut live_cells = 0;
    let y_len = l.len() as i32 - 1;
    let x_len = l[0].len() as i32 - 1;
    let cur_cel = l[y][x];

    let y_min = {
        let mut m = y as i32 - 1;
        if y as i32 - 1 < 0 {
            m = y_len;
        }
        m as usize
    };

    let y_max = {
        let mut m = y + 1;
        if y as i32 + 1 > y_len {
            m = 0;
        }
        m
    };

    let x_min = {
        let mut m = x as i32 - 1;
        if x as i32 - 1 < 0 {
            m = x_len;
        }
        m as usize
    };

    let x_max = {
        let mut m = x + 1;
        if x as i32 + 1 > x_len {
            m = 0;
        }
        m
    };

    live_cells += l[y_min][x]; // Up Nei
    live_cells += l[y_max][x]; // Down Nei
    live_cells += l[y][x_min]; // left nei
    live_cells += l[y][x_max]; // right nei
    live_cells += l[y_min][x_min]; // Up Left Nei
    live_cells += l[y_max][x_max]; // Down Right Nei
    live_cells += l[y_min][x_max]; // Up Right nei
    live_cells += l[y_max][x_min]; // Down Left nei

    match cur_cel {
        1 => {
            if live_cells >= 2 && live_cells <= 3 {
                return 1;
            }
            return 0;
        }
        0 => {
            if live_cells == 3 {
                return 1;
            }
            return 0;
        }
        _ => return 0,
    }
}

pub fn random_grid() -> [[i32; 15]; 15] {
    let mut l = [[0; 15]; 15];
    let mut x = 0;

    while x < 100 {
        let rand_y = rand::thread_rng().gen_range(0..15);
        let rand_x = rand::thread_rng().gen_range(0..15);
        l[rand_y][rand_x] = 1;

        x += 1;
    }
    l
}
