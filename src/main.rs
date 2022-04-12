use rand::{Rng, thread_rng};

struct SortingAlg {
    input:Vec<i32>,
}

struct AlgSteps {
    current_alg:&'static str,
    steps:Vec<String>
}

impl AlgSteps {
    pub fn clear(&mut self) {
        self.steps.clear();
    }

    pub fn add_step(&mut self, step:&'static str) {
        self.steps.push(String::from(step));
    }

    pub fn add_step_string(&mut self, step:String) {
        self.steps.push(step);
    }

    pub fn get_steps(&self, input:&Vec<i32>) -> String {
        let mut output:String = format!("Input values: {:?}\nCurrent sorting algorithm: {}\n", input, self.current_alg);
        for step in &self.steps {
            output.push_str(step);
            output.push_str("\n");
        }
        output
    }
}

impl SortingAlg {
    fn selection_sort(&mut self, steps: &mut AlgSteps) {
        steps.steps.clear();
        steps.current_alg = "Selection Sort";
        let mut output = self.input.to_owned();
        steps.add_step("Start a loop that breaks when `swapped` equals true.");
        loop {
            let mut swapped:bool = false;
            steps.add_step("Iterate over the input values");
            for (ind, ..) in self.input.iter().enumerate() {
                steps.add_step("Check if there's a value after the current one to be sorted");
                if ind + 1 < self.input.len() {
                    steps.add_step_string(format!("Check if current value `{}` is greater than next value `{}`", output[ind], output[ind+1]));
                    if &output[ind] > &output[ind+1] {
                        steps.add_step_string(format!("`{cur}` is greater than `{next}`, so swap them. [{cur}, {next}] becomes [{next}, {cur}])", cur = output[ind], next = output[ind+1]));
                        let val = output[ind];
                        output.remove(ind);
                        output.insert(ind+1, val);
                        swapped = true;
                    }
                    else {

                        steps.add_step_string(format!("`{cur}` is NOT greater than `{next}`, do nothing.", cur = output[ind], next = output[ind+1]));
                    }
                }
                else {
                    steps.add_step("There is not a value after the current one, so there's nothing to sort from here. Do nothing.");
                }
            }
            if !swapped {
                steps.add_step("No values were swapped, so the list must be sorted! Break loop. The End.");
                break;
            }
            else {
                steps.add_step("Some values were swapped, so let's iterate over it again to make sure the list is fully sorted\n--New Iteration--");
            }
        }
        steps.add_step_string(format!("Outut: {:?}", output));
        println!("{}", steps.get_steps(&self.input));
    }
}


const INPUT_SIZE:usize = 5;
fn main() {
    let mut input : Vec<i32> = vec![0; INPUT_SIZE];

    // Rand input
    let mut rng = thread_rng();
    for i in 0..INPUT_SIZE {
        input[i as usize] = rng.gen_range(0, 100);
    }
    let mut sort = SortingAlg {
        input,
    };
    let mut steps = AlgSteps {
        current_alg: "",
        steps: Vec::new()
    };
    sort.selection_sort(&mut steps);
}
