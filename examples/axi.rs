//#[macro_use]
extern crate verugent;

use verugent::vcore::*;

fn main() {
	axilite();
}

fn axilite(){
	let mut al = VModule::new("axi_interface");
	let clk = al.Input("clk", 0);
	let rst = al.Input("rst", 0);

	let mut axi = AXIS_Lite_new(clk, rst);
	axi.NamedRegSet("Calc_A");
	axi.NamedRegSet("Calc_B");
	axi.NamedRegSet("Output_calc");

	let a = al.Output("o_A", 32);
	let b = al.Output("o_B", 32);
	al.Assign(a._e(axi.NamedReg("Calc_A")));
	al.Assign(b._e(axi.NamedReg("Calc_B")));

	let w = al.Wire("write_en_cdata",0);
	al.Assign(w._e(_Num(1)));

	let calc = al.Input("i_Calc", 32);
	axi.RegWrite(w, calc);
	
	al.AXI(axi);

	al.endmodule();
	al.genPrint();
	//al.genFile("axi.v");
}