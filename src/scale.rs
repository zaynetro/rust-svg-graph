pub trait Scale {
    fn offset(&self, i: f32) -> f32;
    fn segment(&self) -> f32;
    fn ticks(&self) -> Vec<i32>;
    fn with_range(&self, start: f32, stop: f32) -> Box<Scale>;
}

pub struct LinearScale {
    domain: (f32, f32),
    range: (f32, f32),
}

impl LinearScale {
    pub fn new(start: f32, stop: f32) -> LinearScale {
        LinearScale {
            domain: (start, stop),
            range: (0.0, 0.0),
        }
    }
}

impl Scale for LinearScale {
    fn offset(&self, i: f32) -> f32 {
        let d = i - self.domain.0;
        let ratio = d / (self.domain.1 - self.domain.0);
        self.range.0 + ratio * (self.range.1 - self.range.0)
    }

    fn segment(&self) -> f32 {
        (self.range.1 - self.range.0) / (self.domain.1 - self.domain.0)
    }

    fn ticks(&self) -> Vec<i32> {
        (0..((self.domain.1 - self.domain.0 + 1.0) as i32)).collect()
    }

    fn with_range(&self, start: f32, stop: f32) -> Box<Scale> {
        Box::new(LinearScale { range: (start, stop), .. *self })
    }
}

pub struct LinearRoundedScale {
    domain: (f32, f32),
    range: (f32, f32),
}

impl LinearRoundedScale {
    pub fn new(start: f32, stop: f32) -> LinearRoundedScale {
        LinearRoundedScale {
            domain: (start, stop),
            range: (0.0, 0.0),
        }
    }

    fn begin_end(&self) -> (f32, f32) {
        let segment = self.segment();
        let begin = Bound::Begin.bound(self.domain.0, segment);
        let end = Bound::End.bound(self.domain.1, segment);
        (begin, end)
    }
}

impl Scale for LinearRoundedScale {
    fn offset(&self, i: f32) -> f32 {
        let (begin, end) = self.begin_end();
        let d = i - begin;
        let ratio = d / (end - begin);
        let res = self.range.0 + ratio * (self.range.1 - self.range.0);
        res
    }

    fn segment(&self) -> f32 {
        // 1 tick per 25px
        let k = 25.0;
        let n = self.domain.1 - self.domain.0;
        let h = self.range.1 - self.range.0;
        let segment = {
            let ticks = h / k;
            let precise = n / ticks;
            match precise {
                p @ _ if p <= 1.0 => 1.0,
                p @ _ if p <= 2.0 => 2.0,
                p @ _             => (p / 5.0).ceil() * 5.0,
            }
        };
        segment
    }

    fn ticks(&self) -> Vec<i32> {
        let segment = self.segment();
        let (begin, end) = self.begin_end();
        let ticks = (end - begin) as f32 / (segment as f32) + 2.0;

        (1..(ticks.ceil() as i32))
            .map(|t| ((t - 1) as f32 * segment + begin) as i32)
            .collect()
    }

    fn with_range(&self, start: f32, stop: f32) -> Box<Scale> {
        Box::new(LinearRoundedScale { range: (start, stop), .. *self })
    }
}

#[derive(Debug)]
enum Bound {
    Begin,
    End
}

impl Bound {
    fn sign(self) -> f32 {
        match self {
            Bound::Begin => -1.0,
            Bound::End   => 1.0,
        }
    }

    fn bound(self, v: f32, segment: f32) -> f32 {
        if v == 0.0 {
            v
        } else if v % segment == 0.0 {
            v + segment * self.sign()
        } else {
            let k = v / segment;
            let rounded = match self {
                Bound::Begin => k.floor(),
                Bound::End   => k.ceil(),
            };
            rounded * segment
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_scale() {
        {
            let scale = LinearScale::new(0.0, 10.0).with_range(0.0, 100.0);
            assert_eq!(scale.segment(), 10.0);
            assert_eq!(scale.offset(0.0), 0.0);
            assert_eq!(scale.offset(5.0), 50.0);
            assert_eq!(scale.ticks(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        }

        {
            let scale = LinearScale::new(-5.0, 5.0).with_range(0.0, 100.0);
            assert_eq!(scale.segment(), 10.0);
            assert_eq!(scale.offset(-5.0), 0.0);
            assert_eq!(scale.offset(5.0), 100.0);
            assert_eq!(scale.ticks(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        }
    }

    #[test]
    fn linear_rounded_scale() {
        {
            let scale = LinearRoundedScale::new(0.0, 10.0).with_range(0.0, 100.0);
            assert_eq!(scale.segment(), 5.0);
            assert_eq!(scale.offset(0.0), 0.0);
            assert_eq!(scale.offset(5.0), 33.333336);
            assert_eq!(scale.ticks(), vec![0, 5, 10, 15]);
        }

        {
            let scale = LinearRoundedScale::new(-5.0, 5.0).with_range(0.0, 100.0);
            assert_eq!(scale.segment(), 5.0);
            assert_eq!(scale.offset(-5.0), 25.0);
            assert_eq!(scale.offset(5.0), 75.0);
            assert_eq!(scale.ticks(), vec![-10, -5, 0, 5, 10]);
        }
    }

    #[test]
    fn bound() {
        assert_eq!(Bound::Begin.bound(0.0, 20.0), 0.0);
        assert_eq!(Bound::Begin.bound(-19.0, 20.0), -20.0);
        assert_eq!(Bound::Begin.bound(-21.0, 10.0), -30.0);
        assert_eq!(Bound::Begin.bound(150.3, 5.0), 150.0);
        assert_eq!(Bound::Begin.bound(3.0, 2.0), 2.0);
        assert_eq!(Bound::Begin.bound(10.0, 10.0), 0.0);

        assert_eq!(Bound::End.bound(10.0, 5.0), 15.0);
        assert_eq!(Bound::End.bound(-19.0, 20.0), 0.0);
        assert_eq!(Bound::End.bound(152.7, 5.0), 155.0);
        assert_eq!(Bound::End.bound(3.0, 2.0), 4.0);

        assert_eq!(Bound::Begin.sign(), -1.0);
        assert_eq!(Bound::End.sign(), 1.0);
    }
}
