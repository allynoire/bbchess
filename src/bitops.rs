
pub trait BitOps {
    fn pop_highest_one(&mut self) -> Option<u32>;
    fn pop_lowest_one(&mut self) -> Option<u32>;
}

impl BitOps for u64 {
    fn pop_highest_one(&mut self) -> Option<u32> {
        if let Some(i) = self.highest_one() {
            *self &= !(1 << i);
            Some(i)
        }
        else { None }
    }
    
    fn pop_lowest_one(&mut self) -> Option<u32> {
        if let Some(i) = self.lowest_one() {
            *self &= !(1 << i);
            Some(i)
        }
        else { None }
    }
}
