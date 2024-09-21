fn draw_line<T: Copy>(
    canvas: &mut Vec<Vec<T>>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    brush: T,
) {
    // DDA algorithm implementation
    let dx = end_x as isize - start_x as isize;
    let dy = end_y as isize - start_y as isize;
    let steps = if dx.abs() > dy.abs() {
        dx.abs()
    } else {
        dy.abs()
    };
    let x_step = dx as f64 / steps as f64;
    let y_step = dy as f64 / steps as f64;
    let mut x: f64 = start_x as f64;
    let mut y: f64 = start_y as f64;
    for _ in 0..= steps {
        canvas[y.round() as usize][x.round() as usize] = brush;
        x += x_step;
        y += y_step;
    }
}

fn main() {
    let args_list = [
        [[1, 1], [8, 8]],
        [[8, 8], [1, 1]],
        [[1, 1], [8, 16]],
        [[1, 8], [8, 1]],
    ];
    for args in args_list {
        let mut canvas = vec![vec![false; 32]; 32];
        draw_line(
            &mut canvas,
            args[0][0],
            args[0][1],
            args[1][0],
            args[1][1],
            true,
        );
        for row in canvas.iter() {
            for &pixel in row {
                print!("{}", pixel as u8);
            }
            println!();
        }
        println!();
    }
}
