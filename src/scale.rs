pub trait Scale {
    fn offset(&self, i: usize) -> f32;
    fn segment(&self, i: usize) -> f32;
    fn spread_segments(&self) -> Vec<i32>;
}

pub struct LinearScale {
    pub n: usize,
    pub width: f32,
}

impl Scale for LinearScale {
    fn offset(&self, i: usize) -> f32 {
        self.segment(i) * (i as f32)
    }

    #[allow(unused_variables)]
    fn segment(&self, i: usize) -> f32 {
        self.width / (self.n as f32)
    }

    fn spread_segments(&self) -> Vec<i32> {
        // 1 tick per 25px
        let k = 25.0;
        let n = self.n as f32;
        let h = self.width;
        let ticks = h / k;
        let segment_width = {
            let precise = n / ticks;
            match precise {
                p @ _ if p <= 1.0 => 1.0,
                p @ _ if p <= 2.0 => 2.0,
                p @ _             => (p / 5.0).ceil() * 5.0,
            }
        } as i32;

        (1..(ticks as i32))
            .map(|t| (t - 1) * segment_width)
            .collect()
    }
}
