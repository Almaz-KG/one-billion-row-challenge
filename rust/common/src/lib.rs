pub struct Record {
    pub min: f32,
    pub max: f32,
    pub sum: f32,
    pub count: u64,
}

impl Record {
    pub fn add(&mut self, value: f32) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.sum += value;
        self.count += 1;
    }

    pub fn average(&self) -> f32 {
        self.sum / self.count as f32
    }
}

impl Default for Record {
    fn default() -> Self {
        Record {
            min: f32::MAX,
            max: f32::MIN,
            sum: 0.0,
            count: 0,
        }
    }
}