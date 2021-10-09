use rand::{seq::SliceRandom, Rng};
use wasm_bindgen::prelude::*;

type Spin = i8;
type GridCoordinate = i8;

struct Spinor;
impl Spinor {
    const UP: Spin = 1;
    const DOWN: Spin = -1;
}

struct Offset2D {
    x: GridCoordinate,
    y: GridCoordinate,
}

const NN_OFFSETS: [Offset2D; 4] = [
    Offset2D { x: 0, y: -1 },
    Offset2D { x: 0, y: 1 },
    Offset2D { x: 1, y: 0 },
    Offset2D { x: -1, y: 0 },
];

#[wasm_bindgen]
struct IsingModel {
    width: GridCoordinate,
    height: GridCoordinate,
    coupling_constant: f32,
    spins: Vec<Spin>,
}

#[wasm_bindgen]
impl IsingModel {
    pub fn new(width: GridCoordinate, height: GridCoordinate, coupling_constant: f32) -> IsingModel {
        if width < 1 || height < 1 {
            panic!("Invalid dimensions");
        }
        let n_spinors = width * height;
        let choices = [Spinor::UP, Spinor::DOWN];
        let mut rng = rand::thread_rng();

        let initial_state: Vec<Spin> = (0..n_spinors)
            .map(|_| *choices.choose(&mut rng).unwrap())
            .collect();
        IsingModel {
            width,
            height,
            coupling_constant,
            spins: initial_state,
        }
    }

    fn get_spin(&self, i: GridCoordinate, j: GridCoordinate) -> &Spin {
        let x = if i>=0 {i % self.width}  else { self.width  + i % self.width };
        let y = if j>=0 {j % self.height} else { self.height + j % self.height };
        &self.spins[(x + self.width * y) as usize]
    }

    fn flip_spin(&mut self, i: GridCoordinate, j: GridCoordinate) {
        let x = if i>=0 {i % self.width}  else { self.width + i % self.width };
        let y = if j>=0 {j % self.height} else { self.height + j % self.height };
        self.spins[(x + self.width * y) as usize] *= -1;
    }

    fn select_random_node(&self) -> (GridCoordinate, GridCoordinate) {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.width);
        let j = rng.gen_range(0..self.height);
        (i, j)
    }

    fn calculate_energy_contribution(&self, i: GridCoordinate, j: GridCoordinate) -> f32 {
        let mut neighbors_spin_sum: Spin = 0;
        for offset in NN_OFFSETS {
            neighbors_spin_sum += self.get_spin(i + offset.x, j + offset.y)
        }
        self.coupling_constant * (self.get_spin(i, j) * neighbors_spin_sum) as f32
    }

    fn total_energy(&self) -> f32 {
        // Room for improvements, naive loop
        let mut total: f32 = 0.;
        for i in 0..self.width {
            for j in 0..self.height {
                total += self.calculate_energy_contribution(i, j);
            }
        }
        total
    }

    fn calculate_boltzmann_factor(energy_contribution: f32, temperature: f32) -> f32 {
        (2.0 * energy_contribution / temperature).exp()
    }

    pub fn step(&mut self, temperature: f32) -> Option<(GridCoordinate, GridCoordinate)> {
        let (i, j) = self.select_random_node();

        let energy_contribution = self.calculate_energy_contribution(i, j);

        let spin_should_flip = energy_contribution >= 0.0  
            || rand::thread_rng().gen::<f32>()
                < Self::calculate_boltzmann_factor(energy_contribution, temperature);

        if spin_should_flip {
            self.flip_spin(i, j);
            Some((i, j))
        } else {
            println!("Rejected");
            None
        }
    }
}
