VeRuGent
========================================

The tool is a DSL library for building Verilog's AST using the Rust programming language.  
Rustプログラミング言語によるVerilogHDLコード構築DSLライブラリです  

[![Build Status](https://travis-ci.org/RuSys/Verugent.svg?branch=master)](https://travis-ci.org/RuSys/Verugent)
[![Crates.io](https://img.shields.io/crates/v/verugent.svg)](https://crates.io/crates/verugent)

[![Docs.rs](https://docs.rs/verugent/badge.svg)](https://docs.rs/verugent)  

Copyright (C) 2019, K.Takano    

License
========================================

Apache License (Version 2.0)  
http://www.apache.org/licenses/LICENSE-2.0  

What's VeRuGent? (VeRuGentとは？)  
========================================

VeRuGent for "Verilog from Rust : Generation Toolkit"  
and it is open-source library for constructing a Verilog HDL source code in Rust.  
VeRuGentはVerilog from Rust : Generation Toolkitsの略称です。  
RustによるVerilogHDLコード構築用ライブラリです。  

VeRuGent does not synthesis of the source code.(Of course it does not do high-level synthesis)  
This generates only Verilog code.    
本ツールは論理合成は行いません。(高位合成も対象外です)  
生成されるのは構築したVerilogのコードのみです。  

VeRuGent is provides a light-weight AST construction method of Verilog HDL.  
You can build up design written in Verilog HDL with less failure by using the Rust's high safety inspection function.  
(At the moment, This functions and safety is insufficient. So ,it is future task to update this tools.)  
簡易化したVerilogHDLのAST構築技術を提供します。Rustの持つ高い安全検査機能を利用することで、  
破綻の少ないハードウェアコードの設計が可能です。  
(現時点で、機能や安全性にやや不安が残るので今後の課題ということで…)    

Requirements (推奨環境)
========================================

* Rust 1.27.0 stable (or later)

* rustc 1.27.0 (3eda71b00 2018-06-19) (or later)

* cargo 1.27.0 (1e95190e5 2018-05-27) (or later)

Installation（導入）
========================================

## Using raw code  

**1.Put the core file in the same directory as the source code.**  
　コアファイルをソースコードと同じディレクトリに入れてください。  

**2.Add modules:**  
モジュールの追加  
```text
mod Verugent;  
```

**3.Declare the use of the core and macro within the block to be used.**  
　ライブラリとマクロ使用の宣言をしてください。  
```text
#[macro_use]  
use Verugent::core::*;  
```

## Using Library  

**1. Use Cargo's function to generate the project folder.  
　And add this code in cargo.toml file.**  
　Cargoのプロジェクト生成機能を使ってプロジェクトフォルダを作成してください。  
　その後、cargo.tomlファイルに以下の記述を付け加えてください。  

```text
[dependencies]  
Verugent = "0.1.3"
```

**2. Declare the use of the core and macro within the block to be used.**  
　下記のコードをmainファイルに記述してライブラリおよびマクロの使用宣言をしてください。  
```text
#[macro_use]  
extern crate Verugent;  
use Verugent::core::*;  
```


**It's only.**  
たったこれだけ！  

Getting Started
========================================
You can find some examples in "example" directory in this repository.  
リポジトリ内のexampleフォルダに作例が置かれていると思います。    

Let's use Verugent as an example of LED lighting circuit.  
この中のLED点灯回路の作例でVerugentを使用してみましょう。    

If you want to know the contents of the LED circuit code written in Rust, Read led.rs file.  
LED回路のコードが読みたい場合はled.rsを参照してください。  

実行コマンドは、cargo run --example ledです。  
Execution command is "cargo run --example led".
```
    #[macro_use]
    extern crate verugent;

    use verugent::vcore::*;

    fn main() {
        led();
    }
	
	・
	・
	・

```
If you build and run this code, you can see the output verilog code as a result.  
コードが実行されたらVerilogコードが出力されます。  
```
    module LED (
        input  CLK,
        input  RST,
        input  BTN1,
        input  BTN2,
        output [7:0] LED
    );
        localparam IDLE = 0;
        localparam RUN = 1;
        localparam END = 2;
        reg [31:0] State;
        reg [31:0] State_Next;
        assign LED = (State==RUN)? 8: 0;

        always@(posedge CLK or posedge RST) begin
            if (RST == 1) begin
                State <= IDLE;
            end
            else begin
                State <= State_Next
            end
        end

        always@(posedge CLK) begin
            case(State)
                IDLE : begin
                    if(BTN1==1&&RST!=1)
                        State_Next <= RUN;
                end
                RUN : begin
                    if(BTN2==1)
                        State_Next <= END;
                end
                END : begin
                    State_Next <= IDLE;
                end
            endcase
        end

    endmodule
```
  
Method cheat sheet
======================================
wire, register, input and output port setting method  
```
Input(&str, Box<E> or i32)				Input port  
Output(&str, Box<E> or i32)				Output port  
Reg_Output(&str, Box<E> or i32)			        Output port(register)  
Inout(&str, Box<E> or i32)				Inout port  
Param(&str, i32)					Global parameter  
LParam(&str, i32)					Local parameter  
Wire(&str, Box<E> or i32)				Wire setting  
Reg(&str, Box<E> or i32)				Register setting  

Mem(&str, Box<E> or i32, Box<E> or i32)	Array register setting  
```
  
Control block method  
```
Always(Always-AST struct)				Always block setting  
Function(Function-AST struct)			        Function block setting  
Assign(Assign-AST struct)				Assign block setting  
```
  
In-block AST  
```
Always:  
	--Drive edge setting--  
	Posedge(Box<E>)					Drive always block(positive edge)  
	Negedge(Box<E>)					Drive always block(negative edge)  
	Nonedge()					Non drive signal  
	
	--Substitution setting--  
	non()						Nonblocking substitution  
	block()						Blocking substitution  
	
Function:  
	func(&str, i32)					Generate function  
	Input(&str, i32)				Function input setting  
        own()						Function name AST generate(using build function AST)  
	
Assign:  
	_e(Box<E>)					Assign AST setting  
	
FSM(Finite State machine):  
	Clock_Reset(Box<E>, Box<E>)			Clock and Reset signal setting for FSM to drive  
	State(&str)					State register name setting  
	AddState(&str)					Add new state  
        goto(&str)					Designation of state transition destination  
	from(&str)					Transition to current state  
	Current(&str)					Change current state  
	Param(&str)					Get parameter AST in fsm  
Branch Method:  
	--if - else--  
	If(Box<E>, Vec<Box<E>>)				If AST setting(if)  
	Else_If(Box<E>, Vec<Box<E>>)			If AST setting(else if)  
	Else(Vec<Box<E>>)				If AST setting(else)  
	
	--case--  
	Case(Box<E>)					Case block setting  
        S(Box<E>, Vec<Box<E>>)				Process for each "case label"  

Common Items:  
	--method--  
	Form(Box<E>)					In block formula  
	
	--Macro--  
	F!( Formula )					Operator syntax that  
    							could not be implemented with overloading.  
				Formula -->		= : Substitution  
        						==: Equal  
							!=: Not equal  
							<=: More and equal  
						        >=: Less and equal  
							< : More than  
							> : Less than  
	
	func_args!( (Box<E>)* )		        	Pass the coated argment in Box.  
	
	
```

Update history
======================================
2017/09/20:  Development started  
2017/11/18:  Add formula macro  
2018/01/20:  Add FSM making method  
2018/06/30:  Add Syntax decomposition and output function  
2018/07/27:  Release first product on Github  
2018/09/15:  FSM function's bugfix  
2019/04/30:  Add AXI Slave Lite interface generator and code generator  
2019/07/07:  Minor bugfix  
2019/08/31:  Bugfix at AXI-Lite port generator  
2019/09/03:  Adjustment of "Function" generator  
2020/01/22:  Add AXI Slave Full interface generator  