extern crate verugent;

use verugent::vcore::*;

fn main() {
	axifull();
}

fn axifull() {
	let mut al = VModule::new("axi_full_interface");
	let clk = al.Input("clk", 0);
	let rst = al.Input("rst", 0);

	let mut axi = AXIS_new(clk, rst);
	//axi.mem_if();
	axi.OrderRegSet(64);

	al.AXI(axi);

	al.endmodule();
	al.genPrint();
	//al.genFile("axifull.v");
}