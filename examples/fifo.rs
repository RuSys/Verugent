#[macro_use]
extern crate verugent;

use verugent::vcore::*;

fn main() {
    fifo();
}

fn fifo() {
    let mut m = VModule::new("FIFO");
    let width = m.Param("width", 8);
    let widthad = m.Param("widthad", 9);
    let words = m.Param("numwords",512);

    let clk = m.Input("CLK", 1);
    let rst = m.Input("RST", 1);
    let d = m.Input("D", &width);
    let q = m.Output("Q", &width);
    let wr = m.Input("WR", 1);
    let rd = m.Input("RD", 1);
    let full = m.Output("FULL", 1);
    let empty = m.Output("EMPTY", 1);

    let cnt = m.Wire("CNT", 10);
    let wp = m.Wire("WP",&widthad);
    let rp = m.Wire("RP",&widthad);

    let wcnt = m.Reg("WCNT", 9);
    let rcnt = m.Reg("RCNT", 9);
    let data = m.Mem("DATA", (width, words));

    m.Assign(q._e(data.addr(&rp)));
    m.Assign(cnt._e(&wcnt - &rcnt));
    m.Assign(full._e(cnt.addr(&widthad)));
    m.Assign(wp._e(wcnt.addr(&widthad-1)));
    m.Assign(rp._e(rcnt.addr(&widthad-1)));

    m.Always(Posedge(clk).Posedge(&rst).non()
                .If(rst, Form(F!(wcnt = 0))
                        .Form(F!(rcnt = 0)))
                .Else(vec![
                    If(wr & !full, Form(data.addr(wp).sst(d))
                                         .Form(F!(wcnt = (&wcnt + 1)))),
                    If(rd & !empty, Form(F!(rcnt = (&rcnt + 1))))]
            ));
    m.endmodule();
    m.genPrint();
}
