pub struct LimitCounter
{
    value: usize,
    limit: usize,
}

impl LimitCounter {
    pub fn new(value: usize, limit: usize) -> LimitCounter { LimitCounter { value, limit } }
    pub fn count(&mut self)
    {
        if self.value < self.limit
        {
            self.value += 1;
        } else {
            self.reset();
        }
    }
    pub fn get_counter(&self) -> usize
    {
        self.value
    }
    fn reset(&mut self)
    {
        self.value = 0;
    }
}