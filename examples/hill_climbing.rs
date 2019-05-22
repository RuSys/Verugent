#[macro_use]
extern crate verugent;

use verugent::vcore::*;

fn main() {
    hill_climbing();
}

fn hill_climbing() {
    let mut m = VModule::new("Hill_climbing");

    let clk = m.Input("CLK", 1);
    let rst = m.Input("RST", 1);

    let start = m.Input("Start", 1);
    let data = m.Input("Data", 32);
    let ready = m.Output("Ready", 1);
    let busy = m.Output("Busy", 1);
    let done = m.Output("Done", 1);

    let evalfuncdata = m.Reg_Output("Eval",32);
    let evalstart = m.Reg_Output("EvalS", 1);
    let evalfuncout = m.Input("EvalOut", 32);
    let evalvalid = m.Input("Evalvalid", 1);

    let result = m.Output("Result",32);

    let bestdata = m.Reg("Bestdata", 32);
    let besteval = m.Reg("Besteval", 32);

    let nextdata = m.Reg("Nextdata", 32);
    let nexteval = m.Reg("Nexteval", 32);

    let neighborcount = m.Reg("Neighbor_Count", 32);
    let tmpeval = m.Reg("Tmpeval", 32);

    let currentnode = m.Reg("CurrentNode", 32);

    let neighborset = m.Wire("NBWire", 32);

    m.Assign(neighborset._e(&neighborcount + &currentnode));
    m.Assign(result._e(&bestdata));

    let mut fsm = Clock_Reset(&clk, &rst)
                .State("State")
                .AddState("IDLE").goto("INIT", F!(start == 1))
                .AddState("INIT").goto("NINIT", F!(evalvalid == 1))
                .AddState("NINIT").goto("UPDATE_NEXT1", Blank!())
                .AddState("UPDATE_NEXT1").goto("UPDATE_NEXT2",F!(evalvalid == 1))
                .AddState("UPDATE_NEXT2").goto("UPDATE_NEXT1",Blank!())
                                         .goto("POSTPROCESS", F!(neighborcount > 1))
                .AddState("POSTPROCESS").goto("NINIT", Blank!())
                                        .goto("END", F!(nexteval <= besteval))
                .AddState("END").goto("IDLE", Blank!());
    
    let s_idle = fsm.Param("IDLE");
    let s_init = fsm.Param("INIT");
    let s_ninit = fsm.Param("NINIT");
    let s_upd_n1 = fsm.Param("UPDATE_NEXT1");
    let s_upd_n2 = fsm.Param("UPDATE_NEXT2");
    let s_pproc = fsm.Param("POSTPROCESS");
    let s_end = fsm.Param("END");

    let fstate = m.FSM(fsm);

    m.Always(Posedge(clk).Posedge(&rst).block()
                .If(rst, Form(F!(evalfuncdata = 0))
                        .Form(F!(bestdata = 0))
                        .Form(F!(besteval = 0))
                        .Form(F!(evalstart = 0))
                        .Form(F!(nextdata = 0))
                        .Form(F!(nexteval = 0))
                        .Form(F!(neighborcount = 0))
                        .Form(F!(tmpeval = 0))
                        .Form(F!(currentnode = 0))
                    )
                .Else(Form(If(F!(fstate == s_init), Form(F!(bestdata = data))
                                                  .Form(F!(evalfuncdata = data))
                                                  .Form(F!(evalstart = 1))
                                                  .Form(F!(besteval = evalfuncout))
                                                  .Form(F!(currentnode = data))
                            )
                            .Else_If(F!(fstate == s_ninit), Form(F!(nextdata = 0))
                                                           .Form(F!(nexteval = 0))
                                                           .Form(F!(neighborcount = 0))
                                                           .Form(F!(evalstart = 0))
                            )
                            .Else_If(F!(fstate == s_upd_n1), Form(F!(evalfuncdata = neighborset))
                                                            .Form(F!(evalstart = 1))
                                                            .Form(F!(tmpeval = evalfuncout))
                                                            .Form(F!(currentnode = nextdata))
                            )
                            .Else_If(F!(fstate == s_upd_n2), Form(If(F!(nexteval < tmpeval), Form(F!(nexteval = tmpeval))
                                                                                            .Form(F!(nextdata = evalfuncdata))
                                                                                            .Form(neighborcount.sst(&neighborcount + 1))
                                    ))
                            )
                            .Else_If(F!(fstate == s_pproc), Form(If(F!(nextdata > bestdata), Form(F!(bestdata = nextdata))))
                            
                            )
                        )
                )
    );

    m.Assign(ready._e(F!(fstate == s_idle)));
    m.Assign(done._e(F!(fstate == s_end)));
    m.Assign(busy._e(F!(fstate != s_idle).land(F!(fstate != s_end))));

    m.endmodule();
	m.genPrint();
}
