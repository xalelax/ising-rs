use rand::seq::SliceRandom;

type Spin = i8;
type GridCoordinate = i8;

struct Spinor;
impl Spinor {
    const UP: Spin = 1;
    const DOWN: Spin = -1;
}

struct IsingModel {
    width: GridCoordinate,
    height: GridCoordinate,
    coupling_constant: f32,
    spins: Vec<Spin>,
}

impl IsingModel {
    fn new(width: GridCoordinate, height: GridCoordinate, coupling_constant: f32) -> IsingModel {
        if width < 1 || height < 1 {
            panic!("Invalid dimensions");
        }
        let n_spinors = width * height;
        let choices = [Spinor::UP, Spinor::DOWN];
        let mut rng = rand::thread_rng();

        let initial_state: Vec<Spin> = (0..n_spinors)
            .map(|_| choices.choose(&mut rng).unwrap().clone())
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
        println!("{},{}", x, y);
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
}

fn main() {
    let model = IsingModel::new(3, 4, (2f32.sqrt() + 1.).ln() / 2.);
    println!(
        "{}, {}, {}, {:?}",
        model.width, model.height, model.coupling_constant, model.spins
    );
}
