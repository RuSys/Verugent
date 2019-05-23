#[macro_use]
extern crate verugent;

use verugent::vcore::*;

fn main() {
    uart();
}

fn uart() {
    let mut m = VModule::new("UART");

    let rst = m.Input("RST", 1);

    let txclk = m.Input("txclk", 1);
    let ldtxdata = m.Input("ld_tx_data", 1);
    let txdata = m.Input("tx_data", 8);
    let txen = m.Input("tx_enable", 1);
    let txo = m.Reg_Output("tx_out", 1);
    let txemp = m.Reg_Output("tx_empty", 1);

    let rxclk = m.Input("rxclk",1);
    let ulrxdata = m.Input("uld_rx_data", 1);
    let rxdata = m.Reg_Output("rx_data", 8);
    let rxen = m.Input("rx_enable", 1);
    let rxin = m.Input("rx_in", 1);
    let rxemp = m.Reg_Output("rx_empty", 0);

    let txreg = m.Reg("tx_reg", 8);
    let txovrn = m.Reg("rx_over_run", 0);
    let txcnt = m.Reg("tx_cnt", 4);
    
    let rxreg = m.Reg("rx_reg", 8);
    let rxsmpl = m.Reg("rx_sample_cnt", 4);
    let rxcnt = m.Reg("rx_cnt", 4);
    let rxfrerr = m.Reg("rx_frame_err", 0);
    let rxovrn = m.Reg("rx_over_run", 0);

    let rxd1 = m.Reg("rx_d1", 1);
    let rxd2 = m.Reg("rx_d2", 1);

    let rxbsy = m.Reg("rx_busy", 1);

    m.Always(Posedge(rxclk).Posedge(&rst).non()
            .If(&rst, Form(F!(rxreg = 0))
                     .Form(F!(rxdata = 0))
                     .Form(F!(rxsmpl = 0))
                     .Form(F!(rxcnt = 0))
                     .Form(F!(rxfrerr = 0))
                     .Form(F!(rxovrn = 0))
                     .Form(F!(rxemp = 1))
                     .Form(F!(rxd1 = 1))
                     .Form(F!(rxd2 = 1))
                     .Form(F!(rxbsy = 0))
                )
            .Else(Form(F!(rxd1 = rxin))
                 .Form(F!(rxd2 = rxd1))
                 .Form(If(ulrxdata, 
                    Form(F!(rxdata = rxreg))
                   .Form(F!(rxemp = 1))))
                 .Form(If(&rxen, 
                    Form(If(!rxbsy.land(rxd2.not()), 
                        Form(F!(rxbsy = 1))
                       .Form(F!(rxsmpl = 1))
                       .Form(F!(rxcnt = 0))
                    ))
                   .Form(If(&rxbsy,
                        Form(rxsmpl.sst(&rxsmpl + 1))
                       .Form(If(F!(rxsmpl == 7),
                            Form(If(F!(rxd2 == 1).land(F!(rxcnt == 0)),
                                    Form(F!(rxbsy = 0)))
                                .Else(Form(rxcnt.sst(&rxcnt + 1))
                                     .Form(If(F!(rxcnt > 0).land(F!(rxcnt < 9)),
                                        Form(rxreg.addr(&rxcnt - 1).sst(&rxd2))
                                     ))
                                     .Form(If(F!(rxcnt == 9), Form(F!(rxbsy = 0))
                                        .Form(If(F!(rxd2 == 0), Form(F!(rxfrerr = 1)))
                                                .Else(Form(F!(rxemp = 0))
                                                     .Form(F!(rxfrerr = 0))
                                                     .Form(F!(rxovrn = !rxemp))))
                                     ))
                                )
                            ))
                        ))
                    )
                    
                ))
            )
            .If(rxen.not(), Form(F!(rxbsy = 0)))
    );


    m.Always(Posedge(txclk).Posedge(&rst).non()
        .If(rst, 
            Form(F!(txreg = 0))
           .Form(F!(txemp = 1))
           .Form(F!(txovrn = 0))
           .Form(F!(txo = 1))
           .Form(F!(txcnt = 0)))
        .Else(
            Form(If(ldtxdata,
                Form(If(txemp.not(), Form(F!(txovrn = 0)))
                    .Else(Form(F!(txreg = txdata))
                         .Form(F!(txemp = 0)))
                )
            ))
            .Form(If(txen.land(txemp.not()),Form(txcnt.sst(&txcnt + 1))
                .Form(If(F!(txcnt == 0), Form(F!(txo = 0))))
                .Form(If(F!(txcnt > 0).land(F!(txcnt < 9)), Form(txo.sst(txreg.addr(&txcnt + 1)))))
                .Form(If(F!(txcnt == 9), Form(F!(txo = 1))
                                        .Form(F!(txcnt = 0))
                                        .Form(F!(txemp = 1))))
            ))
            .Form(If(txen.not(), Form(F!(txcnt = 0))))
        )
    );
    m.endmodule();

	m.genPrint();
}
