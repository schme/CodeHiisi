#[derive(Debug)]
pub struct Mat4<T> {
    dat: [[T; 4]; 4]
}

impl Mat4<f32> {
    pub fn new() -> Mat4<f32> {
        Mat4 {
            dat : {
                [[1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]]
            }
        }
    }
}
