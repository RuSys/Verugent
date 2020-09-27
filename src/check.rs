// Check機能を実装
// ・wire/regの名前(camel or snakeの不一致指摘) -- ok
// ・FSMのモデル構成(gotoとfromの混在指摘) -- 
// ・二項演算の長さ規定(10項程度を超えるとwarning) -- ok
// ・Functionの構造

extern crate regex;
extern crate ansi_term;

use super::vcore::*;
use self::regex::Regex;
use self::ansi_term::Colour::*;

macro_rules! Warn {
	() => {
		Yellow.bold().paint("V-Warning")
	};
}
/*
macro_rules! Suggest {
	() => {
		Green.bold().paint("Suggest")
	};
}
*/
macro_rules! Note {
	() => {
		Green.bold().paint("*Note*")
	};
}

// module, function, port, wire, reg : snake
// parameter, localparam : uppercamel
// define : SCREAMING_SNAKE

/// Detect UpperCamelCase or lowerCamelCase in code
fn camel_detection(s: &str, mode: char) -> bool {
    let re;
    if mode == 'U' {
        // UpperCamel mode
        re = Regex::new(r"[A-Z][a-z]*([A-Z0-9][a-z0-9]+)*([A-Z])?").unwrap();
    }
    else if mode == 'L' {
        // lowerCamel mode
        re = Regex::new(r"[a-z]*([A-Z0-9][a-z0-9]+)*([A-Z])?").unwrap();
    }
    else {
        return false;
	}
	
    let reg = re.captures(s);
	let caps;
	match reg {
		Some(x) => {
			caps = x;
		},
		None => {
			return false;
		}
	}

    let st = caps.get(0).unwrap().as_str();
    if st == "" || st.len() != s.len() {
        false
    }
    else {
        true
    }
}


/// Detect snake_case in code
fn snake_detection(s: &str) -> bool {
	let re = Regex::new(r"[a-z][a-z0-9]*(_[a-z][a-z0-9]*)*").unwrap();
	let reg = re.captures(s);
	let caps;
	match reg {
		Some(x) => {
			caps = x;
		},
		None => {
			return false;
		}
	}

    let st = caps.get(0).unwrap().as_str();
    if st == "" || st.len() != s.len() {
        false
    }
    else {
        true
    }
}

/// change camel to snake
fn camel_to_snake(s: &str) -> String {
	let mut st = String::new();
	let re = Regex::new(r"[a-z]+|[A-Z][a-z0-9]*").unwrap();
	let mut caps_it = re.captures_iter(s);
	st += &(&(caps_it.nth(0).unwrap()[0]).to_string().to_lowercase());
	for caps in caps_it {
		st += "_";
		st += &((&caps[0]).to_string().to_lowercase());
	}
	st
}

/// change snake to camel
fn snake_to_camel(s: &str, mode: char) -> String {
	let mut st = String::new();
	let mut v: Vec<&str> = s.split('_').collect();
    if mode == 'U' {
        // UpperCamel mode
        for st_ in v {
			let mut tmpv: Vec<char> = st_.chars().collect();
			tmpv[0] = tmpv[0].to_uppercase().nth(0).unwrap();
			let s2: String = tmpv.into_iter().collect();
			st += &s2;
		}
    }
    else if mode == 'L' {
		// lowerCamel mode
		st += v[0];
		v.remove(0);
		for st_ in v {
			let mut tmpv: Vec<char> = st_.chars().collect();
			tmpv[0] = tmpv[0].to_uppercase().nth(0).unwrap();
			let s2: String = tmpv.into_iter().collect();
			st += &s2;
		}
    }
    else {
        return "".to_string();
    }
	st
}

impl VModule {
	pub fn all_check(&mut self) {
		let modn = self.module_name_check();
		let io = self.io_check();
		let prm = self.param_check();
		let lcl = self.local_check();
		let lprm = lcl.0;
		let wr = lcl.1;
		let fnc = self.function_check();
		let asn = self.assign_len_check();
		let alw = self.always_edge_check();
		let nedge = alw.0;
		let medge = alw.1;

		if modn | io | prm | lprm | wr | fnc {
			println!("{}: It is recommended that you write your code based on a specific naming convention.",Note!());
			//println!("It is recommended that you write your code based on a specific naming convention.");
			println!("The naming convention for the programming language Rust (RFC 430) is as follows:");
			println!("** Modules, functions, and variables must be written in {}.", White.bold().underline().paint("snake_case"));
			println!("** Types and traits must be written in {}.", White.bold().underline().paint("CamelCase"));

			println!("\nIn accordance with this, it is appropriate to describe the hardware based on the following rules.");
			println!("** Modules, functions, wires, regs and input / output must be written in {}.", White.bold().underline().paint("snake_case"));
			println!("** Parameters and local parameters must be written in {}.\n", White.bold().underline().paint("CamelCase"));
		}

		if asn {
			//println!("{}: An arithmetic description that is too long significantly reduces readability. So you should be multiple short descriptions.",Note!());
			println!("{}: Arithmetic operations that are too long significantly reduce readability.",Note!());
			println!("\t Therefore, the arithmetic operation must be divided into multiple short descriptions.");
			println!("For example:");
			println!("--Not recommended case--");
			println!("module.Assign(smooth._e( (img_data_0_0 + img_data_0_1 + img_data_0_2 + img_data_1_0 + img_data_1_1 + img_data_1_2 + img_data_2_0 + img_data_2_1 + img_data_2_2) / 8 );");
			println!("--Better case--");
			println!("module.Assign( line0._e(img_data_0_0 + img_data_0_1 + img_data_0_2) );");
			println!("module.Assign( line1._e(img_data_1_0 + img_data_1_1 + img_data_1_2) );");
			println!("module.Assign( line2._e(img_data_2_0 + img_data_2_1 + img_data_2_2) );");
			println!("module.Assign( smooth._e( (line0 + line1 + line2) / 8 );\n");
		}

		if nedge {
			println!("{}: It is recommended to use \"always\" as a sequential circuit.",Note!());
			println!("When generating a combinational circuit, readability is improved by using \"function\" or \"assign\".");
		}

		if medge {
			println!("{}: The driving edge positive and negative are mixed.",Note!());
			println!("It is recommended that the driving edge of \"always\" be either positive or negative.");
		}
	}

	pub fn module_name_check(&mut self) -> bool {
		if cfg!(target_os = "windows") {
			ansi_term::enable_ansi_support().unwrap();
		}
		if !snake_detection(&(self.get_mod_name())) {
			println!("{}: Module name is not snake_case. (Module name :{})", Warn!(), self.get_mod_name());
			if camel_detection(&(self.get_mod_name()), 'U') | camel_detection(&(self.get_mod_name()), 'L') {
				println!("This is better for coding. --> Port name: {}",camel_to_snake(&(self.get_mod_name())));
			}
			return true;
		}
		false
	}

	pub fn io_check(&mut self) -> bool {
		let mut h = false;
		if cfg!(target_os = "windows") {
			ansi_term::enable_ansi_support().unwrap();
		}
		let mut vm = self.clone();
		for n in vm.out_port() {
			if !snake_detection(&(n.clone().getName())) {
				println!("{}: Port is not snake_case. (Port name :{})", Warn!(), n.clone().getName());
				if camel_detection(&(n.getName()), 'U') | camel_detection(&(n.getName()), 'L') {
					println!("This is better for coding. --> Port name: {}",camel_to_snake(&(n.getName())));
				}
				h = true;
			}
		}
		h
	}

	pub fn param_check(&mut self) -> bool {
		let mut h = false;
		if cfg!(target_os = "windows") {
			ansi_term::enable_ansi_support().unwrap();
		}
		let mut vm = self.clone();
		for n in vm.out_param() {
			if !camel_detection(&(n.clone().getName()), 'U') {
				println!("{}: Parameter is not CamelCase. (Parameter name :{})", Warn!(), n.clone().getName());
				if snake_detection(&(n.getName())) {
					println!("This is better for coding. --> Parameter name: {}",snake_to_camel(&(n.getName()), 'U'));
				}
				h = true;
			}
		}
		h
	}

	pub fn local_check(&mut self) -> (bool, bool) {
		let mut hp = false;
		let mut hwr = false;
		if cfg!(target_os = "windows") {
			ansi_term::enable_ansi_support().unwrap();
		}
		let mut vm = self.clone();
		for n in vm.out_l_param() {
			let io = n.getIO();
			match io {
				io_p::param_ => {
					if !camel_detection(&(n.clone().getName()), 'U') {
						println!("{}: Local parameter is not CamelCase. (Parameter name :{})", Warn!(), n.clone().getName());
						if snake_detection(&(n.getName())) {
							println!("This is better for coding. --> Parameter name: {}",snake_to_camel(&(n.getName()), 'U'));
						}
						hp = true;
					}
				},
				io_p::none => {
					if !snake_detection(&(n.clone().getName())) {
						if n.getReg() {
							println!("{}: Register is not snake_case. (Register name :{})", Warn!(), n.clone().getName());
						}
						else {
							println!("{}: Wire is not snake_case. (Wire name :{})", Warn!(), n.clone().getName());
						}

						if camel_detection(&(n.clone().getName()), 'U') | camel_detection(&(n.clone().getName()), 'L') {
							println!("This is better for coding. --> Name: {}", &camel_to_snake(&(n.clone().getName())));
						}
						hwr = true;
					}
				}
				_ => {},
			}	
		}
		(hp, hwr)
	}

	pub fn function_check(&mut self) -> bool {
		let mut h = false;
		if cfg!(target_os = "windows") {
			ansi_term::enable_ansi_support().unwrap();
		}
		let mut vm = self.clone();
		for n in vm.out_func_name() {
			if !snake_detection(&n) {
				println!("{}: Function name is not snake_case. (Function name :{})", Warn!(), n.clone());
				if camel_detection(&n, 'U') | camel_detection(&n, 'L') {
					println!("This is better for coding. --> Function name: {}", &camel_to_snake(&n));
				}
				h = true;
			}
		}
		h
	}

	pub fn assign_len_check(&mut self) -> bool {
		let mut h = false;
		for st in self.out_assign() {
			let l = check_length(st.clone().ROut());
			if l >= 10 {
				println!("{}: Too long assignment. (length: {})", Warn!(), l);
				h = true;
			}
		}
		h
	}

	pub fn always_edge_check(&mut self) -> (bool, bool) {
		let mut ne = false;
		let mut me = false;
		for (i, al) in &mut self.out_always().iter().enumerate() {
			let mut tal = al.clone();
			let plen = tal.out_p_edge().len();
			let nlen = tal.out_n_edge().len();
			if plen == 0 || nlen == 0 {
				println!("{}: This always component is combination circuit. (always number: {})", Warn!(), i);
				ne = true;
			}
			if plen > 0 && nlen > 0 {
				println!("{}: A description that mixes\"positive\" and \"negative\" is not recommended.(always number: {})", Warn!(), i);
				me = true;
			}
		}
		(ne, me)
	}
}

fn check_length(ast: Box<E>) -> i32 {
	let mut c = 0;
	decomp_ast(ast, &mut c);
	c
}

/// decomposition
fn decomp_ast(ast: Box<E>, count: &mut i32){
    let e = *ast;
    match e {
        E::Bin(_ , ref l, ref r) => {
            decomp_ast(l.clone(),count);
            decomp_ast(r.clone(),count);
        }
        E::Ldc(_) => {
            *count += 1;
        }
        E::Num(_) => {
            *count += 1;
        }
        E::PL(_, ref t, ref f) => {
            //DeconpAST(d.clone(),&count);
            decomp_ast(t.clone(),count);
            decomp_ast(f.clone(),count);
        },
        E::MEM(ref m, _) => {
            let ma = &*m;
            decomp_ast(ma.clone(),count);
        }
		E::MBT(ref m, _, _) => {
			let mn = &*m;
			decomp_ast(mn.clone(),count);
		}
        E::No(ref b) => {
            let bb = &*b;
            decomp_ast(bb.clone(),count);
        }
        E::Red(_, ref a) => {
			decomp_ast(a.clone(),count);
        }
        _ => {}
    }
}