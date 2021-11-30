pub fn run_intcode(mut commands: Vec<i32>, inputs: Vec<i32>) -> Vec<i32> {
    let mut index = 0;
    let mut input_index = 0;
    let mut outputs: Vec<i32> = Vec::new();
    while index < commands.len() {
        print!("\nnext opcode is: {}\n", commands[index]);
        let opcode = commands[index] % 100;

        // params start at index 1 (opcode is index 0)
        let get_param = |x: usize| commands[index + x];
        let get_mode_param = |x| {
            let base_param = get_param(x);
            if get_digit(commands[index], (x + 1).try_into().unwrap()) == 0 {
                return commands[base_param as usize]
            }
            base_param
        };
        match opcode {
            1 => {
                let assign_index = get_param(3);
                commands[assign_index as usize] = get_mode_param(1) + get_mode_param(2);
                index += 4;
            }
            2 => {
                let assign_index = get_param(3);
                commands[assign_index as usize] = get_mode_param(1) * get_mode_param(2);
                index += 4;
            }
            3 => {
                let assign_index = get_param(1);
                commands[assign_index as usize] = inputs[input_index];
                input_index += 1;
                index += 2;
            }
            4 => {
                let assign_index = get_param(1);
                outputs.push(commands[assign_index as usize]);
                print!("{:?}\n", outputs);
                index += 2;
            }
            5 => {
                if get_mode_param(1) != 0 {
                    println!("jumping to {}", get_mode_param(2));
                    index = get_mode_param(2) as usize;
                } else {
                    index += 3
                }
            }
            6 => {
                if get_mode_param(1) == 0 {
                    println!("jumping to {}", get_mode_param(2));
                    index = get_mode_param(2) as usize;
                } else {
                    index += 3
                }
            }
            7 => {
                let val = if get_mode_param(1) < get_mode_param(2) { 1 } else { 0 };
                let assign_index = get_param(3);
                commands[assign_index as usize] = val;
                index += 4;
            }
            8 => {
                let val = if get_mode_param(1) == get_mode_param(2) { 1 } else { 0 };
                let assign_index = get_param(3);
                commands[assign_index as usize] = val;
                index += 4;
            }
            99 => {
                index = commands.len();
            }
            _ => panic!("unexpected value! {}", commands[index]),
        }
    }
    outputs
}

fn get_digit(num: i32, digit: u32) -> i32 {
    num / (10_i32.pow(digit)) % 10_i32
}
