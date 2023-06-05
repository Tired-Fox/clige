use noise::{Billow, Fbm, NoiseFn, Perlin, Simplex, Worley};

pub struct NoiseMapBuilder<N>
where
    N: NoiseFn<f64, 2>,
{
    pub noise: N,
    step: (f64, f64),
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
            step: (0.15, 0.15),
            x_bounds: (1., 1.),
            y_bounds: (1., 1.),
        }
    }

    // pub fn size(self, width: usize, height: usize) -> Self {
    //     NoiseMapBuilder {
    //         size: (width, height),
    //         ..self
    //     }
    // }

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

    pub fn step(self, x: f64, y: f64) -> Self {
        NoiseMapBuilder {
            step: (x, y),
            ..self
        }
    }

    pub fn build(self) -> NoiseMap<N> {
        NoiseMap {
            noise: self.noise,
            step: self.step,
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
    step: (f64, f64),
    x_bounds: (f64, f64),
    y_bounds: (f64, f64),
}

impl NoiseMap<Perlin> {
    pub fn perlin(seed: i64) -> NoiseMapBuilder<Perlin> {
        NoiseMapBuilder::new(Perlin::new(seed as u32))
    }
}

impl NoiseMap<Billow<Perlin>> {
    pub fn billow(seed: i64) -> NoiseMapBuilder<Billow<Perlin>> {
        NoiseMapBuilder::new(Billow::<Perlin>::new(seed as u32))
    }
}

impl NoiseMap<Fbm<Perlin>> {
    pub fn fbm(seed: i64) -> NoiseMapBuilder<Fbm<Perlin>> {
        NoiseMapBuilder::new(Fbm::<Perlin>::new(seed as u32))
    }
}

impl NoiseMap<Worley> {
    pub fn worley(seed: i64) -> NoiseMapBuilder<Worley> {
        NoiseMapBuilder::new(Worley::new(seed as u32))
    }
}

impl NoiseMap<Simplex> {
    pub fn simplex(seed: i64) -> NoiseMapBuilder<Simplex> {
        NoiseMapBuilder::new(Simplex::new(seed as u32))
    }
}

impl<N> NoiseMap<N>
where
    N: NoiseFn<f64, 2>,
{
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

    pub fn step(&mut self, x: f64, y: f64) {
        self.step= (x, y);
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
        let current_y = self.y_bounds.0 + self.step.1 * y as f64;
        let current_x = self.x_bounds.0 + self.step.0 * x as f64;
        self.noise.get(pad_array(&[current_x, current_y]))
    }
}
