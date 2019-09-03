#[macro_use]
extern crate verugent;

use verugent::vcore::*;

fn main() {
	func_test();
}

fn func_test() {
	let mut m = VModule::new("functest");

	let ina = m.Input("ina", 32);
	let inb = m.Input("inb", 32);

	let out = m.Output("outs", 0);

	let mut f = func("testf", 32);
	
	let ia = f.Input("ia", 32);
	let ib = f.Input("ib", 32);
	
	f.If(F!(ia == ib), 
		Form(f.clone().own().sst(1))
	);
	f.Else(
		Form(f.clone().own().sst(0))
	);
	m.Function(f.clone());

	m.Assign(out._e(f.using(func_args!(ina, inb))));

	m.endmodule();
	m.genPrint();
	//m.genFile("fucn.v")
}