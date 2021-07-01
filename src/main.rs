mod sim;

use sim::State;

fn main() {
    let mut state = State::new();
    state.main.0[0] = 0x00;
    state.main.0[1] = 0x00;
    state.main.0[2] = 0x10;
    state.main.0[3] = 0x00;
    state.main.0[4] = 0x00;
    state.main.0[5] = 0x20;
    state.clk = 0;
    state.prop();
    println!("0x{:x}", state.fsm_ir_output);
    state.clk = 1;
    state.prop();
    println!("0x{:x}", state.fsm_ir_output);
    state.clk = 0;
    state.prop();
    state.clk = 1;
    state.prop();
    println!("0x{:x}", state.fsm_ir_output);
    state.clk = 0;
    state.prop();
    state.clk = 1;
    state.prop();
    println!("0x{:x}", state.fsm_ir_output);
    state.clk = 0;
    state.prop();
    state.clk = 1;
    state.prop();
    println!("0x{:x}", state.fsm_ir_output);
    state.reset = 1;
    state.prop();
    println!("0x{:x}", state.fsm_ir_output);
}
