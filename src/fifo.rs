
use verugent::vcore::*;

pub fn fifo() {
    let mut m = VModule::new("FIFO");
    let width = m.Param("width", 8);
    let widthad = m.Param("widthad", 9);
    let words = m.Param("numwords",512);

    let clk = m.Input("CLK", 1);
    let rst = m.Input("RST", 1);
    let d = m.Input("D", width.clone());
    let q = m.Output("Q", width.clone());
    let wr = m.Input("WR", 1);
    let rd = m.Input("RD", 1);
    let full = m.Output("FULL", 1);
    let empty = m.Output("EMPTY", 1);

    let cnt = m.Wire("CNT", 10);
    let wp = m.Wire("WP",widthad.clone());
    let rp = m.Wire("RP",widthad.clone());

    let wcnt = m.Reg("WCNT", 9);
    let rcnt = m.Reg("RCNT", 9);
    let data = m.Mem("DATA", (width.clone(), words.clone()));

    m.Assign(q.clone()._e(data.clone().addr(rp.clone())));
    m.Assign(cnt.clone()._e(wcnt.clone() - rcnt.clone()));
    m.Assign(full.clone()._e(cnt.clone().addr(widthad.clone())));
    m.Assign(wp.clone()._e(wcnt.clone().addr(widthad.clone()-1)));
    m.Assign(rp.clone()._e(rcnt.clone().addr(widthad.clone()-1)));

    m.Always(Posedge(clk.clone()).Posedge(rst.clone()).non()
                .If(rst, Form(wcnt.clone().sst(0))
                        .Form(rcnt.clone().sst(0)))
                .Else(vec![
                    If(wr & full.clone().not(), Form(data.clone().addr(wp).sst(d))
                                         .Form(wcnt.clone().sst(wcnt.clone() + 1))),
                    If(rd & empty.clone().not(), Form(rcnt.clone().sst(rcnt.clone() + 1)))]
            ));
    m.endmodule();
    m.genPrint();
}