use noise::{Billow, Fbm, NoiseFn, Perlin};

pub struct NoiseMapBuilder<N>
where
    N: NoiseFn<f64, 2>,
{
    pub noise: N,
    size: (usize, usize),
    x_bounds: (f64, f64),
    y_bounds: (f64, f64),
}

impl<N> NoiseMapBuilder<N>
where
    N: NoiseFn<f64, 2>,
{
    pub fn new(noise: N) -> Self {
        NoiseMapBuilder {
            noise,
            size: (100, 100),
            x_bounds: (1., 1.),
            y_bounds: (1., 1.),
        }
    }

    pub fn size(self, width: usize, height: usize) -> Self {
        NoiseMapBuilder {
            size: (width, height),
            ..self
        }
    }

    pub fn x_bounds(self, lower: f64, upper: f64) -> Self {
        NoiseMapBuilder {
            x_bounds: (lower, upper),
            ..self
        }
    }

    pub fn y_bounds(self, lower: f64, upper: f64) -> Self {
        NoiseMapBuilder {
            y_bounds: (lower, upper),
            ..self
        }
    }

    pub fn bounds(self, lower: f64, upper: f64) -> Self {
        NoiseMapBuilder {
            x_bounds: (lower, upper),
            y_bounds: (lower, upper),
            ..self
        }
    }

    pub fn scale(self, scale: f64) -> Self {
        NoiseMapBuilder {
            x_bounds: (-scale, scale),
            y_bounds: (-scale, scale),
            ..self
        }
    }

    pub fn build(self) -> NoiseMap<N> {
        NoiseMap {
            noise: self.noise,
            size: self.size,
            x_bounds: self.x_bounds,
            y_bounds: self.y_bounds,
        }
    }
}

pub struct NoiseMap<N>
where
    N: NoiseFn<f64, 2>,
{
    pub noise: N,
    size: (usize, usize),
    x_bounds: (f64, f64),
    y_bounds: (f64, f64),
}

impl NoiseMap<Perlin> {
    pub fn perlin(seed: u32) -> NoiseMapBuilder<Perlin> {
        NoiseMapBuilder::new(Perlin::new(seed))
    }
}

impl NoiseMap<Billow<Perlin>> {
    pub fn billow(seed: u32) -> NoiseMapBuilder<Billow<Perlin>> {
        NoiseMapBuilder::new(Billow::<Perlin>::new(seed))
    }
}

impl NoiseMap<Fbm<Perlin>> {
    pub fn fbm(seed: u32) -> NoiseMapBuilder<Fbm<Perlin>> {
        NoiseMapBuilder::new(Fbm::<Perlin>::new(seed))
    }
}

impl<N> NoiseMap<N>
where
    N: NoiseFn<f64, 2>,
{
    pub fn size(&mut self, width: usize, height: usize) {
        self.size = (width, height);
    }

    pub fn x_bounds(&mut self, lower: f64, upper: f64) {
        self.x_bounds = (lower, upper)
    }

    pub fn y_bounds(&mut self, lower: f64, upper: f64) {
        self.y_bounds = (lower, upper)
    }

    pub fn bounds(&mut self, lower: f64, upper: f64) {
        self.x_bounds(lower, upper);
        self.y_bounds(lower, upper);
    }

    pub fn scale(&mut self, scale: f64) {
        self.x_bounds(-scale, scale);
        self.y_bounds(-scale, scale);
    }
}

fn pad_array<const SIZE: usize>(values: &[f64]) -> [f64; SIZE] {
    let mut result = [0.0; SIZE];

    result[..values.len().min(SIZE)].clone_from_slice(&values[..values.len().min(SIZE)]);

    result
}

impl<N> NoiseMap<N>
where
    N: NoiseFn<f64, 2>,
{
    pub fn get(&self, x: usize, y: usize) -> f64 {
        let x_extent = self.x_bounds.1 - self.x_bounds.0;
        let y_extent = self.y_bounds.1 - self.y_bounds.0;

        let x_step = x_extent / self.size.0 as f64;
        let y_step = y_extent / self.size.1 as f64;

        let current_y = self.y_bounds.0 + y_step * y as f64;
        let current_x = self.x_bounds.0 + x_step * x as f64;
        self.noise.get(pad_array(&[current_x, current_y]))
    }
}
