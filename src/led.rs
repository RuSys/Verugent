#![macro_use]
extern crate verugent;
use verugent::vcore::*;

pub fn led(){
    let mut m = VModule::new("LED");

    let clk  = m.Input("CLK", 1);
    let rst  = m.Input("RST", 1);
    let btn1 = m.Input("i_BTN1", 1);
    let btn2 = m.Input("i_BTN2", 1);
    let mut led = m.Output("o_LED", 8);

    let mut fsm = Clock_Reset(clk.clone(),rst.clone())
                .State("State")
                .AddState("IDLE").goto("RUN", F!(btn1 == 1).land(F!(rst != 1)))
                .AddState("RUN").goto("END", F!(btn2 == 1))
                .AddState("END").goto("IDLE", Blank!());
    let run = fsm.Param("RUN");
    let fstate = m.FSM(fsm);

    m.Assign(led._e(_Branch(F!(fstate == run), _Num(255), _Num(0))));
    m.endmodule();
}