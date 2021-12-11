use good_lp::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    variables! {
        vars:
            in0  (integer);
            in1  (integer);
            in11 (integer);
            in2  (integer);
            in3  (integer);
            in4  (integer);

            out0 (integer);
            out1 (integer);
            out2 (integer);
            out3 (integer);
            out4 (integer);
    }

    let mut model = vars
        .minimise(in0 + in1 + in11 + in2 + in3 + in4 + out0 + out1 + out2 + out3 + out4)
        .using(default_solver)
        .with(constraint!(in0        == 0))
        .with(constraint!(out4       == 0))
        .with(constraint!(in1 - out0 == 0))
        .with(constraint!(in1 + 8 == in11))
        .with(constraint!(in2 - out1 == 0))
        .with(constraint!(in3 - out1 == 0))
        .with(constraint!(in3 - out2 == 0))
        .with(constraint!(in4 - out0 == 0))
        .with(constraint!(out0 - in0 == 0))
        .with(constraint!(out1 - in1 <= 8))
        .with(constraint!(out2 - in2 == 0))
        .with(constraint!(out3 - in3 == 0))
        .with(constraint!(out4 - in4 == 0))
        .with(constraint!(in1 >= 0))
        .with(constraint!(in11 >= 0))
        .with(constraint!(in2 >= 0))
        .with(constraint!(in3 >= 0))
        .with(constraint!(in4 >= 0))
        .with(constraint!(out0 >= 0))
        .with(constraint!(out1 >= 0))
        .with(constraint!(out2 >= 0))
        .with(constraint!(out3 >= 0));

    model.set_parameter("log", "0");

    let solution = model
        .solve()
        .unwrap();

    println!("in0: {}", solution.value(in0));
    println!("in1: {}", solution.value(in1));
    println!("in11: {}", solution.value(in11));
    println!("in2: {}", solution.value(in2));
    println!("in3: {}", solution.value(in3));
    println!("in4: {}", solution.value(in4));

    println!("out0: {}", solution.value(out0));
    println!("out1: {}", solution.value(out1));
    println!("out2: {}", solution.value(out2));
    println!("out3: {}", solution.value(out3));
    println!("out4: {}", solution.value(out4));

    Ok(())
}
