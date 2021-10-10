import { IsingModel } from "ising-rs";

const ising_model = IsingModel.new(3,3, 3.);

console.log(ising_model.total_energy());