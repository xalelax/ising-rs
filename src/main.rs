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
}

fn main() {
    let model = IsingModel::new(3, 4, (2f32.sqrt() + 1.).ln() / 2.);
    println!(
        "{}, {}, {}, {:?}",
        model.width, model.height, model.coupling_constant, model.spins
    );
}
