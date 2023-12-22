
pub struct  Recast {
    pub points: Vec<Vec<i32>>,
}

impl Drop for Recast {
    fn drop(&mut self) {
        self.points.clear()
    }
}