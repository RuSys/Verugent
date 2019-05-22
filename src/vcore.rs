
use std::ops::*;
use std::string::String;
use std::fs::File;
use std::io::Write;

use std::*;

/** 
  * 構文生成マクロ
  * オーバーロードで対応できない構文用のマクロ
  **/
#[macro_export]
macro_rules! F {
    ($lhs:ident == $rhs:expr) => {
        ($lhs.clone()).eq($rhs.clone())
    };

    ($lhs:ident != $rhs:expr) => {
        ($lhs.clone()).ne($rhs.clone())
    };

    ($lhs:ident <= $rhs:expr) => {
        ($lhs.clone()).ge($rhs.clone())
    };
    
    ($lhs:ident < $rhs:expr) => {
        ($lhs.clone()).gt($rhs.clone())
    };

    ($lhs:ident >= $rhs:expr) => {
        ($lhs.clone()).le($rhs.clone())
    };

    ($lhs:ident > $rhs:expr) => {
        ($lhs.clone()).lt($rhs.clone())
    };

    ($lhs:ident = $rhs:expr) => {
        ($lhs.clone()).sst($rhs.clone())
    };

    ( $lhs:ident || $rhs:expr ) => {
        $lhs.clone().lor($rhs.clone())
    };

    ( $lhs:ident && $rhs:expr ) => {
        $lhs.clone().land($rhs.clone())
    };
}

/**
  * Verilogモジュールクラス
  * すべてのASTの統合構造体
  **/
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone,Debug)]
pub struct VModule {
    Module_Name : String,
    IO_Port     : Vec<wrVar>,
    IO_Param    : Vec<wrVar>,
    Local       : Vec<wrVar>,
    Always_AST  : Vec<Always>,
    Assign_AST  : Vec<Assign>,
    Function_AST: Vec<Func_AST>,
    Fsm         : Vec<FsmModule>,
    Axi         : Vec<AXI>,

    // generate code
    code        : String,
}

/*
/// 入出力ポート、内部配線用Trait
#[allow(dead_code)]
#[allow(non_snake_case)]
impl VModule{
    /// input の追加
    pub fn Input(&mut self, name: &str) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Input(name, 1);
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// inout の追加
    pub fn Inout(&mut self, name: &str) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Inout(name, 1);
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// output の追加
    pub fn Output(&mut self, name: &str) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Output(name, 1);
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// output(register) の追加
    pub fn Reg_Output(&mut self, name: &str) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.OutputReg(name, 1);
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// wire の追加
    pub fn Wire(&mut self, name: &str) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Wire(name, 1);
        self.Local.push(tmp.clone());
        return _V(tmp);
    }

    /// reg の追加
    pub fn Reg(&mut self, name: &str) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Reg(name, 1);
        self.Local.push(tmp.clone());
        return _V(tmp);
    }
}
*/

/// 入出力ポート、内部配線用Trait
#[allow(non_snake_case)]
pub trait Vset<T> {
    fn Input(&mut self, name: &str, Width: T) -> Box<E>;
    fn Inout(&mut self, name: &str, Width: T) -> Box<E>;
    fn Output(&mut self, name: &str, Width: T) -> Box<E>;
    fn Reg_Output(&mut self, name: &str, Width: T) -> Box<E>;
    fn Wire(&mut self, name: &str, Width: T) -> Box<E>;
    fn Reg(&mut self, name: &str, Width: T) -> Box<E>;
}

/// 入力幅：Box<E>
#[allow(dead_code)]
#[allow(non_snake_case)]
impl<T> Vset<T> for VModule
where
    T: Into<Box<E>>,
{
    /// input の追加
    fn Input(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = wrVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.Input(name, len);
        if let E::Ldc(wr) = width { tmp.Width( &( wr.getWP() ) ); };
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// inout の追加
    fn Inout(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = wrVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.Inout(name, len);
        if let E::Ldc(wr) = width { tmp.Width( &( wr.getWP() ) ); };
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// output の追加
    fn Output(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = wrVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.Output(name, len);
        if let E::Ldc(wr) = width { tmp.Width( &( wr.getWP() ) ); };
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// output(register) の追加
    fn Reg_Output(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = wrVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.OutputReg(name, len);
        if let E::Ldc(wr) = width { tmp.Width( &( wr.getWP() ) ); };
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// wire の追加
    fn Wire(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = wrVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.Wire(name, len);
        if let E::Ldc(wr) = width { tmp.Width( &( wr.getWP() ) ); };
        self.Local.push(tmp.clone());
        return _V(tmp);
    }

    /// reg の追加
    fn Reg(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = wrVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.Reg(name, len);
        if let E::Ldc(wr) = width { tmp.Width( &( wr.getWP() ) ); };
        self.Local.push(tmp.clone());
        return _V(tmp);
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl VModule {
    /// モジュールの生成
    #[allow(non_snake_case)]
    pub fn new(Name: &str) -> VModule {
        VModule{Module_Name: Name.to_string(), 
            IO_Port: Vec::new(),
            IO_Param: Vec::new(),
            Local: Vec::new(),
            Always_AST: Vec::new(),
            Assign_AST: Vec::new(),
            Function_AST: Vec::new(),
            Fsm: Vec::new(),
            Axi: Vec::new(),
            
            code: " ".to_string()}
    }

    /// パラメータの追加
    pub fn Param(&mut self, name: &str, Value: i32) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Parameter(name, Value);
        self.IO_Param.push(tmp.clone());
        return _V(tmp);
    }

    /// ローカルパラメータの追加
    pub fn LParam(&mut self, name: &str, Value: i32) -> Box<E>{
        let mut tmp = wrVar::new();
        tmp.Parameter(name, Value);
        self.Local.push(tmp.clone());
        return _V(tmp);
    }

    /// Debug: モジュール名の取得
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    fn getName(&mut self) -> String {
        self.Module_Name.clone()
    }

    /// always 構文ブロックの追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Always(&mut self, AST_of_Always: Always) {
        let tmp = AST_of_Always;
        self.Always_AST.push(tmp.clone());
        return;
    }

    /// assign 構文 AST の追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Assign(&mut self, AST_of_Assign: Assign) {
        let tmp = AST_of_Assign;
        self.Assign_AST.push(tmp.clone());
        return;
    }

    /// function 構文ブロックの追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Function(&mut self, AST_of_Function: Func_AST) {
        let tmp = AST_of_Function;
        self.Function_AST.push(tmp.clone());
    }

    /// FSM AST 構文ブロック群を追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn FSM(&mut self, fsm: FsmModule) -> Box<E> {
        let tmp = fsm.clone();
        let mut stmt = fsm.StateOut();
        let state = *(tmp.clone().StateReg());
        let p;
        let mut np = wrVar::new();
        let mut n = 0;
        for ss in &mut stmt {
            self.Local.push(wrVar{name: ss.getStateName(), 
                            io_param: io_p::param_, 
                            width: 0, 
                            length: 0, 
                            reg_set: false, 
                            value: n, 
                            width_p: "_".to_string(), 
                            length_p: "_".to_string()});
            n+=1;
        }

        if let E::Ldc(x) = state {
            p = x.clone();
            let nam = p.getName() + "_Next";
            if let E::Ldc(wr) = *wrVar::new().Reg(&nam, 32) {np = wr;}
            }
        else {return Box::new(E::Null);}
        self.Local.push(p);
        self.Local.push(np);
        self.Fsm.push(tmp.clone());

        return tmp.StateReg();
    }

    /// モジュールの AST 解析と Verilog 構文の出力
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn endmodule(&mut self) -> String {
        let mut st = String::new();
        //print!("module {} ",self.getName());
        st += &format!("module {} ",self.getName());
        
        // 入出力パラメータ出力コード
        st += &PrintParam(self.IO_Param.clone());

        // 入出力ポート出力コード
        st += &PrintPort(self.IO_Port.clone());

        // 内部パラメータおよび内部配線出力コード
        st += &PrintLocal(self.Local.clone());

        // Assign構文出力コード
        st += &PrintAssign(self.Assign_AST.clone());

        // Always構文出力コード
        st += &PrintAlways(self.Always_AST.clone());

        // Function構文出力コード
        st += &PrintFunction(self.Function_AST.clone());

        st += "\n    // ----Extra Component set----\n\n";

        
        // FSMの出力コード
        if self.Fsm.clone().len() > 0 {
            for tmp in self.Fsm.clone() {
                st += &PrintFsm(tmp.clone());
            }
        }
        
        if self.Axi.clone().len() > 0 {
            let mut i = -1;
            for tmp in self.Axi.clone() {
                i += 1;
                st += &PrintAXI(tmp.clone(), i);
            }
        }

        st += "\nendmodule\n";
        self.code = st.clone();

        return st;
    }

    pub fn genPrint(&mut self) {
        println!("{}",self.code);
    }

    pub fn genFile(&mut self, path: &str) -> Result<(),Box<std::error::Error>> {
        let mut file = File::create(path)?;
        write!(file, "{}", self.code)?;
        file.flush()?;
        Ok(())
    }
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub trait AXI_trait<T> {
    fn AXI(&mut self, setAXI: T);
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
impl AXI_trait<AXISLite> for VModule {
    fn AXI(&mut self, setAXI: AXISLite) {
        let length = self.Axi.len() as i32;
        
        let reg_length = setAXI.reg_array.len() as i32;
        let mut reg_addr_width: i32 = 1;

        // address width calc
        loop {
            if 2i32.pow(reg_addr_width as u32) >= (reg_length * 4 - 1) {
                break;
            }
            reg_addr_width += 1;
        }

        // read address channel
        let o_arr = self.Output(&(format!("o_S_ARREADY{}", length.clone())), 0);
        let i_arv = self.Input(&(format!("i_S_ARVALID{}", length.clone())), 0);
        let i_ara = self.Input(&(format!("i_S_ARADDR{}", length.clone())), reg_addr_width);
        self.Input(&(format!("i_S_ARPROT{}", length.clone())), 3);

        // read data channel
        let o_rda = self.Output(&(format!("o_S_RDATA{}", length.clone())), 32);
        let o_rsp = self.Output(&(format!("o_S_RRESP{}", length.clone())), 2);
        let o_rva = self.Output(&(format!("o_S_RVALID{}", length.clone())), 0);
        let i_rre = self.Input(&(format!("i_S_RREADY{}", length.clone())), 0);

        // write address channel
        let o_awr = self.Output(&(format!("o_S_AWREADY{}", length.clone())), 0);
        let i_awv = self.Input(&(format!("i_S_AWVALID{}", length.clone())), 0);
                    self.Input(&(format!("i_S_AWADDR{}", length.clone())), reg_addr_width);
                    self.Input(&(format!("i_S_AWPROT{}", length.clone())), 3);

        // write data channel
        let i_wda = self.Input(&(format!("i_S_WDATA{}", length.clone())), 32);
        let i_wst = self.Input(&(format!("i_S_WSTRB{}", length.clone())), 4);
        let i_wva = self.Input(&(format!("i_S_WVALID{}", length.clone())), 0);
        let o_wre = self.Output(&(format!("o_S_WREADY{}", length.clone())), 0);

        // write response channel
        let o_bre = self.Output(&(format!("o_S_BRESP{}", length.clone())), 2);
        let o_bva = self.Output(&(format!("o_S_BVALID{}", length.clone())), 0);
        let i_bre = self.Input(&(format!("i_S_BREADY{}", length.clone())), 0);

        // inner wire and register
        let r_arr = self.Reg(&(format!("r_arready{}", length.clone())), 0);
        let w_arv = self.Wire(&(format!("w_arvalid{}", length.clone())), 0);
        let w_ara = self.Wire(&(format!("w_araddr{}", length.clone())), reg_addr_width);

        let r_rda = self.Reg(&(format!("r_rdata{}", length.clone())), 32);
        let r_rsp = self.Reg(&(format!("r_rresp{}", length.clone())), 2);
        let r_rva = self.Reg(&(format!("r_rvalid{}", length.clone())), 0);
        let w_rre = self.Wire(&(format!("w_rready{}", length.clone())), 0);

        let r_awr = self.Reg(&(format!("r_awready{}", length.clone())), 0);
        let w_awv = self.Wire(&(format!("w_awvalid{}", length.clone())), 0);
                    self.Reg(&(format!("r_awaddr{}", length.clone())), reg_addr_width);

        let w_wda = self.Wire(&(format!("w_wdata{}", length.clone())), 32);
        let w_wst = self.Wire(&(format!("r_wstrb{}", length.clone())), 4);
        let w_wva = self.Wire(&(format!("w_wvalid{}", length.clone())), 0);
        let r_wre = self.Reg(&(format!("r_wready{}", length.clone())), 0);

        let r_bre = self.Reg(&(format!("r_bresp{}", length.clone())), 2);
        let r_bva = self.Reg(&(format!("r_bvalid{}", length.clone())), 0);
        let w_bre = self.Wire(&(format!("w_bready{}", length.clone())), 0);

        // 接続の追加
        self.Assign(o_arr._e(r_arr));
        self.Assign(w_arv._e(i_arv));
        self.Assign(w_ara._e(i_ara));

        self.Assign(o_rda._e(r_rda));
        self.Assign(o_rsp._e(r_rsp));
        self.Assign(o_rva._e(r_rva));
        self.Assign(w_rre._e(i_rre));

        self.Assign(o_awr._e(r_awr));
        self.Assign(w_awv._e(i_awv));
        //self.Assign(w_awa._e(i_awa));

        self.Assign(w_wda._e(i_wda));
        self.Assign(w_wst._e(i_wst));
        self.Assign(w_wva._e(i_wva));
        self.Assign(o_wre._e(r_wre));

        self.Assign(o_bre._e(r_bre));
        self.Assign(o_bva._e(r_bva));
        self.Assign(w_bre._e(i_bre));

        self.Axi.push(AXI::Lite(setAXI));
    }
}


/// メモリレジスタ生成用のトレイト
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub trait Memset<T> {
    fn Mem(&mut self, name: &str, args: T) -> Box<E>;
}

/// 入力(Box<E>:Box<E>)生成するメモリ構文
impl<T, U> Memset<(T, U)> for VModule
where
    T: Into<Box<E>>,
    U: Into<Box<E>>,
{
    /// メモリ構文
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Mem(&mut self, name: &str, args: (T, U)) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Mem(name, 0, 0);
        if let E::Ldc(wr) = *args.0.into() { tmp.Width( &( wr.getName() ) ); };
        if let E::Ldc(wr) = *args.1.into() { tmp.Length( &( wr.getName() ) ); };
        self.Local.push(tmp.clone());
        return _V(tmp);
    }
}

/**
  * 入出力設定パラメータ
  * 特に大きな意味は無い
  **/
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum io_p {
    input_,
    output_,
    inout_,
    param_,
    none,
}

/// 入出力ポート、パラメータデータ格納構造体
/**
  * 入出力パラメータクラス
  * 
  **/
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct wrVar {
    name     : String,
    io_param : io_p,
    width    : i32,
    length   : i32,
    reg_set  : bool,
    value    : i32,
    width_p  : String,
    length_p : String,
}

/**
  * 入出力パラメータクラスメソッド
  * セット・ゲット・コピー関数
  **/
#[allow(non_camel_case_types)]
impl wrVar {
    /// コンストラクタ
    pub fn new() -> wrVar {
        wrVar{name: "None".to_string(), io_param: io_p::none, width: 0, length: 0, reg_set: false, value: 0, width_p: "_".to_string(), length_p: "_".to_string()}
    }

    /// データ取得メソッド:name
    #[allow(non_snake_case)]
    pub fn getName(&self) -> String {
        self.name.clone()
    }

    /// データ取得メソッド:io_param
    #[allow(non_snake_case)]
    pub fn getIO(&self) -> io_p {
        self.io_param.clone()
    }

    /// データ取得メソッド:width
    #[allow(non_snake_case)]
    pub fn getWidth(&self) -> i32 {
        self.width.clone()
    }

    /// データ取得メソッド:length
    #[allow(non_snake_case)]
    pub fn getLength(&self) -> i32 {
        self.length.clone()
    }

    /// データ取得メソッド:reg_set
    #[allow(non_snake_case)]
    pub fn getReg(&self) -> bool {
        self.reg_set.clone()
    }

    /// データ取得メソッド:value
    #[allow(non_snake_case)]
    pub fn getValue(&self) -> i32 {
        self.value.clone()
    }

    /// データ取得メソッド:width_p
    #[allow(non_snake_case)]
    pub fn getWP(&self) -> String {
        self.width_p.clone()
    }

    /// データ取得メソッド:length_p
    #[allow(non_snake_case)]
    pub fn getLP(&self) -> String {
        self.length_p.clone()
    }

    /// パラメータによる長さ設定メソッド
    #[allow(non_snake_case)]
    pub fn Length(&mut self, S: &str) -> wrVar {
        self.length_p = S.to_string();
        self.clone()
    }

    /// パラメータによる幅設定メソッド
    #[allow(non_snake_case)]
    pub fn Width(&mut self, S: &str) -> wrVar {
        self.width_p = S.to_string();
        self.clone()
    }

    /// パラメータ設定メソッド:input
    #[allow(non_snake_case)]
    pub fn Input(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        self.io_param = io_p::input_;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:output
    #[allow(non_snake_case)]
    pub fn Output(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        self.io_param = io_p::output_;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:inout
    #[allow(non_snake_case)]
    pub fn Inout(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        self.io_param = io_p::inout_;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:output(register)
    #[allow(non_snake_case)]
    pub fn OutputReg(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();

        self.io_param = io_p::output_;
        self.width = Width;
        self.reg_set = true;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:parameter
    #[allow(non_snake_case)]
    pub fn Parameter(&mut self, Name: &str, Value: i32) -> Box<E> {
        self.name = Name.to_string();
        self.value = Value;

        self.io_param = io_p::param_;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:wire
    #[allow(non_snake_case)]
    pub fn Wire(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        _V(self.clone())
    }

    /// パラメータ設定メソッド:reg
    #[allow(non_snake_case)]
    pub fn Reg(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        self.reg_set = true;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:reg[ length : 0 ]
    #[allow(non_snake_case)]
    pub fn Mem(&mut self, Name: &str, Width: i32, Lenght: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;
        self.length = Lenght;

        self.reg_set = true;
        _V(self.clone())
    }
}

/// Assign 構文代入用トレイト
#[allow(non_snake_case)]
pub trait SetEqual<T>
where
    T: Into<Box<E>>,
{
     fn _e(&self, RHS: T) -> Assign;

     fn _ve(&self, RHS: T) -> Assign;
}

/// Assign 構文代入用トレイト
impl<T> SetEqual<T> for Box<E>
where
    T: Into<Box<E>>,
{
    /// Box<E>からAssign生成を行うメソッド
    #[allow(non_snake_case)]
    fn _e(&self, RHS: T) -> Assign {
        let mut tmp = Assign::new();
        tmp.L(self).R(&RHS.into())
    }

    #[allow(non_snake_case)]
    fn _ve(&self, RHS: T) -> Assign {
        let mut tmp = Assign::new();
        tmp.L(self).R(&RHS.into())
    }
}

/**
  * assign構文用AST構造体
  * 
  **/
#[allow(dead_code)]
#[derive(Clone,Debug)]
pub struct Assign {
    lhs     : Box<E>,
    rhs     : Box<E>,
}

/**
  * assign構文用ASTメソッド
  * 
  **/
impl Assign {
    /// assign 構文生成
    pub fn new() -> Assign {
        Assign{lhs: Box::new(E::Ldc(wrVar::new())), rhs: Box::new(E::Ldc(wrVar::new()))}
    }

    /// 左辺設定メソッド
    #[allow(non_snake_case)]
    pub fn L<T: Into<Box<E>>>(&mut self, LHS: T) -> Assign {
        self.lhs = LHS.into();
        let tmp = self.clone();
        return tmp;
    }

    /// 右辺設定メソッド
    #[allow(non_snake_case)]
    pub fn R<T: Into<Box<E>>>(&mut self, RHS: T) -> Assign {
        self.rhs = RHS.into();
        let tmp = self.clone();
        return tmp;
    }

    /// 左辺出力メソッド
    #[allow(non_snake_case)]
    pub fn LOut(&mut self) -> Box<E> {
        self.lhs.clone()
    }

    /// 右辺出力メソッド
    #[allow(non_snake_case)]
    pub fn ROut(&mut self) -> Box<E> {
        self.rhs.clone()
    }
}

/**
  * Always構文用AST構造体
  * 
  **/
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone,Debug)]
pub struct Always{
    br      : String,
    stmt    : Vec<Box<E>>,
    P_edge  : Vec<wrVar>,
    N_edge  : Vec<wrVar>,
}

/// Always構文内使用の立ち上がり信号設定構文
#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn Posedge<T: Into<Box<E>>>(edge: T) -> Always {
    let e = *edge.into();
    let mut tmp = Always{br: "brock".to_string(), stmt: Vec::new(), P_edge: Vec::new(), N_edge: Vec::new()};
    match e {
        E::Ldc(wr) => tmp.P_edge.push(wr.clone()),
        _ => return tmp,
    }
    tmp.clone()
}

/// Always構文内使用の立ち下り信号設定構文
#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn Negedge<T: Into<Box<E>>>(edge: T) -> Always {
    let e = *edge.into();
    let mut tmp = Always{br: "brock".to_string(), stmt: Vec::new(), P_edge: Vec::new(), N_edge: Vec::new()};
    match e {
        E::Ldc(wr) => tmp.N_edge.push(wr.clone()),
        _ => return tmp,
    }
    tmp.clone()
}

/// Always構文内使用の信号未設定構文
#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn Nonedge() -> Always {
    Always{br: "brock".to_string(), stmt: Vec::new(), P_edge: Vec::new(), N_edge: Vec::new()}
}

/**
  * Always構文用ASTメソッド
  * 
  **/
impl Always {
    // Debug
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn new() -> Always {
        Always{br: "brock".to_string(), stmt: Vec::new(), P_edge: Vec::new(), N_edge: Vec::new()}
    }

    /// debug:外部出力
    fn blockout(&mut self) ->String {
        self.br.clone()
    }

    /// ブロッキング設定
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn block(&mut self)-> Always {
        self.br = "brock".to_string();
        self.clone()
    }

    /// ノンブロッキング設定
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn non(&mut self)-> Always {
        self.br = "Non".to_string();
        self.clone()
    }

    /// 立ち上がり信号設定
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Posedge<T: Into<Box<E>>>(&mut self, edge: T) -> Always {
        let e = *edge.into();
        match e {
            E::Ldc(wr) => self.P_edge.push(wr.clone()),
            _ => return self.clone(),
        }
        self.clone()
    }

    /// 立ち下がり信号設定
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Negedge<T: Into<Box<E>>>(&mut self, edge: T) -> Always {
        let e = *edge.into();
        match e {
            E::Ldc(wr) => self.N_edge.push(wr.clone()),
            _ => return self.clone(),
        }
        self.clone()
    }

    /// 分岐 if 構文追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn If<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Always {
        let i = If(C.into(), S);
        self.stmt.push(i);
        self.clone()
    }

    /// 分岐 else if 構文追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Else_If<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Always {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: true, IfE: C.into(), ST: S});
                self.stmt.push(Box::new(E::BL(p)));
            },
            _ => {return self.clone();},
        }
        self.clone()
    }

    /// 分岐 else 構文追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Else(&mut self, S: Vec<Box<E>>) -> Always {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: false, IfE: Box::new(E::Null), ST: S});
                self.stmt.push(Box::new(E::BL(p)));
            },
            _ => {},
        }
        self.clone()
    }

    /// Case文追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Case<T: Into<Box<E>>>(&mut self, Sel: T) -> Always {
        let c = Case(Sel.into());
        self.stmt.push(c);
        self.clone()
    }

    /// Case文内の分岐追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn S<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Always {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(C.into(), S);
                self.stmt.push(Box::new(E::CS(p)))
            },
            _ => {
                println!("abort");
                panic!("Not Case");
            },
        }
        self.clone()
    }
}

/**
  * function生成用関数
  *
  **/ 
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub fn func(name: &str, width: i32) -> Func_AST {
    Func_AST{top: wrVar::new().Wire(name, width), input: Vec::new(), stmt: Vec::new()}
}

/**
  * function構文用AST構造体
  * 
  **/
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Clone,Debug)]
pub struct Func_AST {
    top   : Box<E>,
    input : Vec<Box<E>>,
    stmt  : Vec<Box<E>>,
}

/**
  * function引数設定マクロ
  * 
  **/
#[macro_export]
macro_rules! func_args {
    ($($x: expr),*) => (
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x.clone());
        )*
        temp_vec
    )
}

/**
  * function構文用ASTメソッド
  * 
  **/
#[allow(non_snake_case)]
impl Func_AST {
    /// Functionのトップ文字列を格納したAST取得
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn own(&mut self) -> Box<E> {
        self.top.clone()
    }

    /// debug:構文生成
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    fn func(&mut self, args: Vec<Box<E>>) -> Box<E> {
        let tmp = Box::new(E::Func(self.top.clone(), args));
        tmp.clone()
    }

    /// 入力の追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Input(&mut self, Name: &str, Width: i32) -> Box<E> {
        let mut tmp = wrVar::new();
        let port = tmp.Input(Name, Width);
        self.input.push(port.clone());
        port
    }

    /// 分岐 if 構文追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn If<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Func_AST {
        let i = If(C.into(), S);
        self.stmt.push(i);
        self.clone()
    }

    /// 分岐 else if 構文追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Else_If<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Func_AST {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: true, IfE: C.into(), ST: S});
                self.stmt.push(Box::new(E::BL(p)));
            },
            _ => {return self.clone();},
        }
        self.clone()
    }

    /// 分岐 else 構文追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Else(&mut self, S: Vec<Box<E>>) -> Func_AST {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: false, IfE: Box::new(E::Null), ST: S});
                self.stmt.push(Box::new(E::BL(p)));
            },
            _ => {},
        }
        self.clone()
    }

    /// Case 文追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Case<T: Into<Box<E>>>(&mut self, Sel: T) -> Func_AST {
        let c = Case(Sel.into());
        self.stmt.push(c);
        self.clone()
    }

    /// Case 文内の分岐追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn S<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Func_AST {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(C.into(), S);
                self.stmt.push(Box::new(E::CS(p)))
            },
            _ => {
                println!("abort");
            },
        }
        self.clone()
    }

    /// Case 文のデフォルト追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Default(&mut self, S: Vec<Box<E>>) -> Func_AST {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(Box::new(E::Null), S);
                self.stmt.push(Box::new(E::CS(p)))
            },
            _ => {
                println!("abort");
            },
        }
        self.clone()
    }
}

/**
  * if,else if,else構造体
  * 
  **/
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Clone,Debug)]
pub struct IfStmt_AST {
    If_     : bool,         // if文フラグ
    IfE     : Box<E>,       // if文条件式
    ST      : Vec<Box<E>>,  // 実行式
}

#[allow(non_snake_case)]
impl IfStmt_AST {
    fn getIfFlag(&mut self) -> bool {
        self.If_.clone()
    }

    fn getTerms(&mut self) -> Box<E> {
        self.IfE.clone()
    }

    fn getStatement(&mut self) -> Vec<Box<E>> {
        self.ST.clone()
    }
}

/// ステートメントブロック内のif構文作成
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub fn If<T: Into<Box<E>>>(C: T, S: Vec<Box<E>>) -> Box<E> {
    let mut i = Vec::new();
    i.push(IfStmt_AST{If_: true, IfE: C.into(), ST: S});
    Box::new(E::BL(i))
}

/// ステートメントブロック内のif分岐追加
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub trait Ifset {
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Else_If<T: Into<Box<E>>>(self, C: T, S: Vec<Box<E>>) -> Box<E>;

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Else(self, S: Vec<Box<E>>) -> Box<E>;
}


impl Ifset for Box<E> {
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Else_If<T: Into<Box<E>>>(self, C: T, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut p;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: true, IfE: C.into(), ST: S});
            },
            _ => return Box::new(E::Null),
        }
        return Box::new(E::BL(p));
    }

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Else(self, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut p;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: false, IfE: Box::new(E::Null), ST: S});
            },
            _ => return Box::new(E::Null),
        }
        return Box::new(E::BL(p));
    }
}

/**
  * Case構造体
  * 
  **/
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Clone,Debug)]
pub struct CaseStmt_AST {
    CaseVar : wrVar,
    Select  : Vec<Case_>,
}

impl CaseStmt_AST {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn SetCaseV(&mut self, V: wrVar) {
        self.CaseVar = V.clone()
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn SetCaseS<T: Into<Box<E>>>(&mut self, Cond: T, Stmt: Vec<Box<E>>) {
        self.Select.push(Case_{CaseT: Cond.into(), CaseS: Stmt})
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn getCaseV(&mut self) -> wrVar {
        self.CaseVar.clone()
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn getSelect(&mut self) -> Vec<Case_> {
        self.Select.clone()
    }
}

// ステートメントブロック内のcase文作成
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
fn Case<T: Into<Box<E>>>(Sel: T) -> Box<E> {
    let e = *Sel.into();
    let mut C = CaseStmt_AST{CaseVar: wrVar::new(), Select: Vec::new()};
    match e {
        E::Ldc(wr) => {
            C.SetCaseV(wr);
        },
        _ => {
            Box::new(E::Null);
        },
    }

    Box::new(E::CS(C))
}

// ステートメントブロック内のcase分岐追加
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub trait Caseset {
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn S<T: Into<Box<E>>>(self, C: T, S: Vec<Box<E>>) -> Box<E>;

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Default(self, S: Vec<Box<E>>) -> Box<E>;
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
impl Caseset for Box<E> {
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn S<T: Into<Box<E>>>(self, C: T, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut n;
        match e {
            E::CS(csast) => {
                n = csast;
            },
            _ => return Box::new(E::Null),
        }
        n.SetCaseS(C.into(), S);
        Box::new(E::CS(n))
    }

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Default(self, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut n;
        match e {
            E::CS(csast) => {
                n = csast;
            },
            _ => return Box::new(E::Null),
        }
        n.SetCaseS(Box::new(E::Null), S);
        Box::new(E::CS(n))
    }
}


/**
  *　Caseの各条件における内部構造体
  * 
  **/
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone,Debug)]
pub struct Case_ {
    pub CaseT   : Box<E>,
    pub CaseS   : Vec<Box<E>>,
}

/// ステートメントブロック用ベクタ_ブロック作成 & 式追加
#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn Form<T: Into<Box<E>>>(formu: T) -> Vec<Box<E>> {
    let mut tmp = Vec::new();
    tmp.push(formu.into());
    return tmp
}

/// ステートメントブロック内の式追加
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub trait addForm<T>
where
    T: Into<Box<E>>,
{
     fn Form(self, formu: T) -> Vec<Box<E>>;
}

impl<T> addForm<T> for Vec<Box<E>>
where
    T: Into<Box<E>>,
{
    fn Form(self, formu: T) -> Vec<Box<E>> {
        let mut tmp = self;
        tmp.push(formu.into());
        return tmp
    }
}

/**
  * 各構文用列挙型構造体
  * 
  **/
#[allow(non_snake_case)]
#[derive(Clone,Debug)]
pub enum E {
    Null,
    Ldc(wrVar),                     // 変数
    Num(i32),                       // 数値
    No(Box<E>),                     // Not構文
    Red(String, Box<E>),            // リダクション構文
    Bin(String, Box<E>, Box<E>),    // 二項演算
    PL(Box<E>, Box<E>, Box<E>),     // 分岐構文
    SB(Box<E>, Box<E>),             // 代入文
    CS(CaseStmt_AST),               // case文
    BL(Vec<IfStmt_AST>),            // if, else if, else文
    Func(Box<E>, Vec<Box<E>>),      // function文
    MEM(Box<E>,Box<E>),             // メモリ
    Node(String),                   // 内部検索用
}

impl<'a> From<&'a Box<E>> for Box<E> {
    fn from(x: &'a Box<E>) -> Self {
        x.clone()
    }
}

impl<'a> From<&'a mut Box<E>> for Box<E> {
    fn from(x: &'a mut Box<E>) -> Self {
        x.clone()
    }
}

impl From<i32> for Box<E> {
    fn from(i: i32) -> Self {
        _Num(i)
    }
}

impl From<&i32> for Box<E> {
    fn from(i: &i32) -> Self {
        _Num(*i)
    }
}

// 変数出力関数
#[allow(non_snake_case)]
fn _V(V: wrVar) -> Box<E>{
    Box::new(E::Ldc(V))
}

// 数値出力関数
#[allow(non_snake_case)]
pub fn _Num(num: i32) -> Box<E>{
    Box::new(E::Num(num))
}

// 代入演算関数
#[allow(non_snake_case)]
pub fn _Veq<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::SB(L.into(), R.into()))
}

// 分岐構文関数
#[allow(non_snake_case)]
pub fn _Branch<T: Into<Box<E>>, U: Into<Box<E>>, V: Into<Box<E>>>(
    Terms: T,
    TrueNode: U,
    FalseNode: V,
) -> Box<E> {
    Box::new(E::PL(Terms.into(), TrueNode.into(), FalseNode.into()))
}

// 演算子関数
/// "+" addition
#[allow(non_snake_case)]
fn _Add<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("add".to_string(), L.into(), R.into()))
}

/// "-" substruction
#[allow(non_snake_case)]
fn _Sub<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("sub".to_string(), L.into(), R.into()))
}

/// "*" multipication
#[allow(non_snake_case)]
fn _Mul<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("mul".to_string(), L.into(), R.into()))
}

/// "/" division
#[allow(non_snake_case)]
fn _Div<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("div".to_string(), L.into(), R.into()))
}

/// "%" modulo
#[allow(non_snake_case)]
fn _Mod<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("mod".to_string(), L.into(), R.into()))
}

/// "||" or
#[allow(non_snake_case)]
fn _LOr<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("lor".to_string(), L.into(), R.into()))
}

/// "&&" and
#[allow(non_snake_case)]
fn _LAnd<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("land".to_string(), L.into(), R.into()))
}

/// "|" or
#[allow(non_snake_case)]
fn _Or<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("or".to_string(), L.into(), R.into()))
}

/// "&" and
#[allow(non_snake_case)]
fn _And<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("and".to_string(), L.into(), R.into()))
}

/// "^" exclusive or
#[allow(non_snake_case)]
fn _Xor<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("xor".to_string(), L.into(), R.into()))
}

/// "==" equal
#[allow(non_snake_case)]
pub fn _Eq<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("equal".to_string(), L.into(), R.into()))
}

/// "!=" not equal
#[allow(non_snake_case)]
pub fn _Neq<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("Not equal".to_string(), L.into(), R.into()))
}

/// "<<" left shift
#[allow(non_snake_case)]
fn _LSH<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("lshift".to_string(), L.into(), R.into()))
}

/// ">>" right shift
#[allow(non_snake_case)]
fn _RSH<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("rshift".to_string(), L.into(), R.into()))
}

/// "<" more than
#[allow(non_snake_case)]
fn _MTH<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("more_than".to_string(), L.into(), R.into()))
}

/// ">" less than
#[allow(non_snake_case)]
fn _LTH<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("less_than".to_string(), L.into(), R.into()))
}

/// "<=" or more
#[allow(non_snake_case)]
fn _OMR<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("or_more".to_string(), L.into(), R.into()))
}

/// ">=" or less
#[allow(non_snake_case)]
fn _OLS<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("or_less".to_string(), L.into(), R.into()))
}

/**
  * 演算子実装メソッド
  *
  **/
pub trait Notc {
    fn not(&self) -> Box<E>;
}

impl Notc for Box<E> {
    fn not(&self) -> Box<E> {
        Box::new(E::No(self.clone()))
    }
}

impl Not for Box<E> {
    type Output = Box<E>;

    fn not(self) -> Box<E> {
        Box::new(E::No(self.clone()))
    }
}

impl<T> Add<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn add(self, other: T) -> Box<E> {
        _Add(self, other.into())
    }
}

impl<T> Add<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn add(self, other: T) -> Box<E> {
        _Add(self, &other.into())
    }
}

impl<T> Sub<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn sub(self, other: T) -> Box<E> {
        _Sub(self, other.into())
    }
}

impl<T> Sub<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn sub(self, other: T) -> Box<E> {
        _Sub(self, &other.into())
    }
}

impl<T> Mul<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn mul(self, other: T) -> Box<E> {
        _Mul(self, other.into())
    }
}

impl<T> Mul<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn mul(self, other: T) -> Box<E> {
        _Mul(self, &other.into())
    }
}

impl<T> Div<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn div(self, other: T) -> Box<E> {
        _Div(self, other.into())
    }
}

impl<T> Div<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn div(self, other: T) -> Box<E> {
        _Div(self, &other.into())
    }
}

impl<T> Rem<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn rem(self, other: T) -> Box<E> {
        _Mod(self, other.into())
    }
}

impl<T> Rem<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn rem(self, other: T) -> Box<E> {
        _Mod(self, &other.into())
    }
}

impl<T> BitOr<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitor(self, other: T) -> Box<E> {
        _Or(self, other.into())
    }
}

impl<T> BitOr<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitor(self, other: T) -> Box<E> {
        _Or(self, &other.into())
    }
}

impl<T> BitAnd<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitand(self, other: T) -> Box<E> {
        _And(self, other.into())
    }
}

impl<T> BitAnd<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitand(self, other: T) -> Box<E> {
        _And(self, &other.into())
    }
}

impl<T> BitXor<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitxor(self, other: T) -> Box<E> {
        _Xor(self, other.into())
    }
}

impl<T> BitXor<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitxor(self, other: T) -> Box<E> {
        _Xor(self, &other.into())
    }
}

impl<T> Shl<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn shl(self, other: T) -> Box<E> {
        _LSH(self, other.into())
    }
}

impl<T> Shl<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn shl(self, other: T) -> Box<E> {
        _LSH(self, &other.into())
    }
}

impl<T> Shr<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn shr(self, other: T) -> Box<E> {
        _RSH(self, other.into())
    }
}

impl<T> Shr<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn shr(self, other: T) -> Box<E> {
        _RSH(self, &other.into())
    }
}

// Equal,Not Equal構文生成
pub trait PartialEq<Rhs = Self> {

    fn eq(self, other: Rhs) -> Box<E>;

    fn ne(self, other: Rhs) -> Box<E>;
}

impl<T> PartialEq<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn eq(self, other: T) -> Box<E> {
        _Eq(self, other.into())
    }

    fn ne(self, other: T) -> Box<E> {
        _Neq(self, other.into())
    }
}

impl<T> PartialEq<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    fn eq(self, other: T) -> Box<E> {
        _Eq(self, &other.into())
    }

    fn ne(self, other: T) -> Box<E> {
        _Neq(self, &other.into())
    }
}

// compare構文生成
pub trait PartialOrd<Rhs = Self>{
    fn lt(self, other: Rhs) -> Box<E>;

    fn le(self, other: Rhs) -> Box<E>;

    fn gt(self, other: Rhs) -> Box<E>;

    fn ge(self, other: Rhs) -> Box<E>;
}

impl<T> PartialOrd<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn lt(self, other: T) -> Box<E> {
        _LTH(self, other.into())
    }

    fn le(self, other: T) -> Box<E> {
        _OLS(self, other.into())
    }

    fn gt(self, other: T) -> Box<E> {
        _MTH(self, other.into())
    }

    fn ge(self, other: T) -> Box<E> {
        _OMR(self, other.into())
    }
}

impl<T> PartialOrd<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    fn lt(self, other: T) -> Box<E> {
        _LTH(self, &other.into())
    }

    fn le(self, other: T) -> Box<E> {
        _OLS(self, &other.into())
    }

    fn gt(self, other: T) -> Box<E> {
        _MTH(self, &other.into())
    }

    fn ge(self, other: T) -> Box<E> {
        _OMR(self, &other.into())
    }
}

// 代入文生成
pub trait Subs<Rhs = Self> {
    fn sst(&self, other: Rhs) -> Box<E>;
}

impl<T> Subs<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn sst(&self, other: T) -> Box<E> {
        _Veq(self.clone(), other.into())
    }
}

// 論理演算子生成
pub trait Logi<Rhs = Self> {
    fn land(&self, other: Rhs) -> Box<E>;

    fn lor(&self, other: Rhs) -> Box<E>;
}

impl<T> Logi<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn land(&self, other: T) -> Box<E> {
        _LAnd(self, &other.into())
    }

    fn lor(&self, other: T) -> Box<E> {
        _LOr(self, &other.into())
    }
}

// メモリ、レジスタ用アドレス指定
pub trait Addr<Rs = Self> {
    fn addr(&self, address: Rs) ->Box<E>;
}

impl<T> Addr<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn addr(&self, address: T) -> Box<E> {
        Box::new(E::MEM(self.clone(), address.into()))
    }
}

/**
  * 出力、分解、デバッグ関数
  * 出力関数以外はデバッグ用関数のため削除しても問題はない
  **/

/// 分解出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn DeconpAST(Parenthesis: bool, ast: Box<E>, cnfg: &str, indent: i32) -> String{
    let e = *ast;
    let mut st = String::new();

    match e {
        E::Bin(ref bin, ref l, ref r) => {
            let tmp = bin.as_str();
            for _ in 0..indent {
                st += "    ";
            }
            if Parenthesis {
                match tmp.clone() {
                    "add" => {st += "("},
                    "sub" => {st += "("},
                    "or" => {st += "("},
                    "lor" => {st += "("},
                    _ => {st += ""},
                }
            }
            let mut pareset = false;
            st += &DeconpAST(false ,l.clone(),cnfg, 0);
            match tmp.clone() {
                "add" => {st += "+"},
                "sub" => {st += "-"},
                "mul" => {st += "*"; pareset = true},
                "div" => {st += "/"; pareset = true},
                "mod" => {st += "%"; pareset = true},
                "or"  => {st += "|"},
                "and" => {st += "&"; pareset = true},
                "lor"  => {st += "||"},
                "land" => {st += "&&"; pareset = true},
                "lshift" => {st += "<<"},
                "rshift" => {st += ">>"},
                "equal" => {st += "=="},
                "Not equal" => {st += "!="},
                "more_than" => {st += "<"},
                "less_than" => {st += ">"},
                "or_more" => {st += "<="},
                "or_less" => {st += ">="},
                _ => panic!("No correspond syntax"),
            }
            st += &DeconpAST(pareset, r.clone(),cnfg, 0);
            if Parenthesis {
                match tmp {
                    "add" => {st += ")"},
                    "sub" => {st += ")"},
                    "or" => {st += ")"},
                    "lor" => {st += ")"},
                    _ => {st += "";},
                }
            }
        }
        E::Ldc(ref wr) => {
            st += &format!("{}",wr.getName());
        }
        E::Num(ref i) => {
            st += &format!("{}",i);
        }
        E::PL(ref d, ref t, ref f) => {
            st += "(";
            st += &DeconpAST(false,d.clone(),cnfg, 0);
            st += ")? ";
            st += &DeconpAST(false, t.clone(),cnfg, 0);
            st += ": ";

            st += &DeconpAST(false, f.clone(),cnfg, 0);
        },
        E::SB(ref l, ref r) => {
            for _ in 0..indent {
                st += "    ";
            }
            st += &DeconpAST(false, l.clone(),cnfg, indent);
            if cnfg.to_string() == "brock".to_string() {
                st += " = ";
            }
            else {
                st += " <= ";
            }
            st += &DeconpAST(false, r.clone(),cnfg, 0);
            st += ";\n";
        }
        E::CS(ref c) => {
            let cn = &*c;
            st += &PrintCase(cn.clone(),cnfg, indent);
        }
        E::BL(ref i) => {
            let iels = &*i;
            st += &PrintIf(iels.clone(),cnfg, indent);
        }
        E::MEM(ref m, ref a) => {
            let ma = &*m;
            let aa = &*a;
            st += &DeconpAST(false, ma.clone(),cnfg, indent);
            st += &format!("[");
            st += &DeconpAST(false, aa.clone(),cnfg, 0);
            st += &format!("]");
        }
        E::No(ref b) => {
            let bb = &*b;
            st += "~";
            st += &DeconpAST(false, bb.clone(),cnfg, 0);
        }
        E::Red(ref r, ref a) => {
            let tmp = r.as_str();
            match tmp.clone() {
                "and" => {st += "&";},
                "or"  => {st += "|"},
                "xor" => {st += "^"},
                "nand"=> {st += "~&"},
                "nor" => {st += "~|"},
                "xnor"=> {st += "~^"},
				_ => {return st;},
            }
			st += &DeconpAST(false, a.clone(), cnfg, 0);
        }
        _ => {
            st += "";
        }
    }
    return st;
}

/// GlobalParameter出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintParam(Param: Vec<wrVar>) -> String {
    let tmp = Param;
    let n = tmp.len();
    let mut num = 0;
    let mut st = String::new();
    if n != 0 {
        st += "#(\n";
    }

    for x in tmp {
        num += 1;
        st += &format!("    parameter {} = {}",x.getName(), x.getValue());
        if n > num {
            st += ",\n";
        }
        else {
            st += "\n";
        }
    }
    if n != 0 {
        st += ")\n";
    }
    return st;
}

/// 入出力ポート出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintPort(Port: Vec<wrVar>) -> String {
    let tmp = Port;
    let n = tmp.len();
    let mut num = 0;

    let mut st = String::new();

    st += "(\n";
    //println!("(");
        for x in tmp {
            let port_set = x.getIO();
            num += 1;
            match port_set {
                io_p::input_ => {
                    st += "    input ";
                }
                io_p::output_ => {
                    if x.getReg() {
                        st += "    output reg ";
                    }
                    else {
                        st += "    output ";
                    }
                }
                io_p::inout_ => {
                    st += "    inout ";
                }
                _ => return st
            }

            if x.getWidth() == 0 && x.getWP() != "_" {
                st += &format!("[{}-1:0] ",x.getWP());
            }
            else if x.getWidth() > 1 {
                st += &format!("[{}:0] ",x.getWidth()-1);
            }
            else {
                st += " ";
            }

            st += &format!("{}",x.getName());

            if x.getLength() == 0 && x.getLP() != "_" {
                st += &format!(" [{}-1:0]",x.getLP());
            }
            else if x.getLength() != 0 {
                st += &format!(" [{}:0]",x.getLength()-1);
            }
            else {
                st += "";
            }

            if n > num {
                st += ",\n";
            }
            else {
                st += "\n";
            }
        }
        st += ");\n";
        st
}

/// LocalParameter + Wire + Reg出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintLocal(PWR: Vec<wrVar>) -> String {
    let mut st = String::new();
    st += "    // ----Generate local parts----\n\n";
    let tmp = PWR;
    for x in tmp {
        let port_set = x.getIO();
        match port_set {
            io_p::param_ => {
                st += &format!("    localparam {} = {};\n",x.getName(), x.getValue());
            }
            io_p::none => {
                if x.getReg() {
                    st += "    reg ";
                }
                else {
                    st += "    wire ";
                }
                if x.getWidth() == 0 && x.getWP() != "_".to_string() {
                    st += &format!("[{}-1:0] ",x.getWP());
                }
                else if x.getWidth() > 1 {
                    st += &format!("[{}:0] ",x.getWidth()-1);
                }
                else {
                    st += " ";
                }

                st += &format!("{}",x.getName());

                if x.getLength() == 0 && x.getLP() != "_".to_string() {
                    st += &format!(" [{}-1:0]",x.getLP());
                }
                else if x.getLength() != 0 {
                    st += &format!(" [{}:0]",x.getLength()-1);
                }
                else {
                    st += "";
                }
                st += ";\n";
            }
            _ => return st
        }
    }
    st
}

/// assign出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintAssign(Assign: Vec<Assign>) -> String {
    let mut st = String::new();
    st += "\n    // ----Generate assign compornent----\n\n";
    let tmp = Assign;
    for mut x in tmp {
        let LO = x.LOut();
        st += "    assign ";
        st += &DeconpAST(false, LO, "", 0);
        st += " = ";
        let port_set = x.ROut();
        st += &DeconpAST(false, port_set, "", 0);
        st += ";\n";
    }
    st += "\n";
    st
}

/// always出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintAlways(Always: Vec<Always>) -> String {
    let mut st = String::new();
    st += "\n    // ----Generate Always block----\n\n";
    let tmp = Always.clone();
    for x in tmp {
        st += "    always@(";
        let mut n = x.P_edge.clone();
        let mut tmp_num = 1;
        let mut len = n.len();
        for y in n{
            st += &format!("posedge {}",y.getName());
            if len > tmp_num {
                st += " or ";
            }
            tmp_num += 1;
        }

        n = x.N_edge.clone();
        len = n.len();
        if tmp_num > 1 && len > 0 {st += " or "}
        tmp_num = 1;
        for y in n {
            st += &format!("negedge {}",y.getName());
            if len > tmp_num {
                st += " or ";
            }
            tmp_num += 1;
        }
        st += ") begin\n";
        for s in x.stmt.clone() {
            st += &DeconpAST(false, s,&x.clone().blockout(), 2);
        }
        
        st += "    end\n";
    }
    return st;
}

/// function出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintFunction(Function: Vec<Func_AST>) -> String {
    let mut st = String::new();
    st += "\n    // ----Generate Function block----\n\n";

    let tmp = Function.clone();
    for x in tmp {
        let e = x.top;
        if let E::Ldc(wrtop) = (*e).clone() {
            st += &format!("\n    function [{}:0] ", wrtop.getWidth()-1);
            st += &DeconpAST(false, e, "", 1);
            st += ";\n";
        }
        for inpt in x.input {
            let i = inpt.clone();
            if let E::Ldc(wr) = (*i).clone() {
                if wr.getWidth() > 0 {
                    st += &format!("        input [{}:0]", wr.getWidth()-1);
                    st += &DeconpAST(false, i, "",2);
                    st += ";\n";
                }
                else {
                    st += "        input \n";
                    st += &DeconpAST(false, i, "", 2);
                    st += ";\n";
                }
            }
        }
        for s in x.stmt {
            st += &DeconpAST(false, s, "", 2);
        }
        st += "    endfunction\n\n";
    }
    return st;
}

/// if_else構文出力関数--ブロック出力関数より呼び出し
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintIf(If_Stmt: Vec<IfStmt_AST>, cnfg: &str, indent: i32) -> String {
    let tmp = If_Stmt;
    let mut num = 0;
    let mut st = String::new();

	let mut nonBranch  = false;
    
    for mut x in tmp {
        let n = x.getStatement();
        if num == 0 {
            let e = *x.clone().getTerms();
            match e {
                E::Null => {
                    num = 0;
					nonBranch = true;
                }
                _ => {
                    for _ in 0..indent {
                        st += "    ";
                    }
                    st += "if(";
                    num += 1;
                    st += &DeconpAST(false, x.getTerms(), "", 0);
                    st += ") begin\n";
                }
            }
        }
        else if x.getIfFlag() {
            for _ in 0..indent {
                st += "    ";
            }
            st += "else if(";
            st += &DeconpAST(false, x.getTerms(), "",0);
            st += ") begin\n";
        }
        else {
            for _ in 0..indent {
                st += "    ";
            }
            st += "else begin\n";
        }

		if nonBranch {
			for y in n.clone() {
            	st += &DeconpAST(false, y,cnfg, indent);
        	}
			return st
		}
        for y in n.clone() {
            st += &DeconpAST(false, y,cnfg, indent + 1);
        }

        for _ in 0..indent {
            st += "    ";
        }
        st += "end\n";
    }
    return st;
}

/// Case構文出力関数--ブロック出力関数より呼び出し
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintCase(case_stmt: CaseStmt_AST, cnfg: &str, indent: i32) -> String {
    let mut tmp = case_stmt;
    let ctmp = tmp.clone().Select;
    let mut st = String::new();
    for _ in 0..indent {
        st += "    ";
    }
    st += &format!("case ({})\n",tmp.getCaseV().getName());
    for x in ctmp {
        let e = x.CaseT.clone();
        let ef = x.CaseS.clone();
        let mut tm = *e.clone();
        for _ in 0..indent+1 {
            st += "    ";
        }
        match tm {
            E::Null => {
                st += "default ";
            },
            _ => {
                st += &DeconpAST(false, e,cnfg, indent + 1);
            },
        }
        st += " :";
        let n = ef.len();
        if n > 1 {st += "begin \n";}
        for y in ef {
            if n > 1 {
                st += &DeconpAST(false, y,cnfg, indent + 2);
            }
            else {
                st += &DeconpAST(false, y,cnfg, 0);
            }
        }
        if n > 1 {
            for _ in 0..indent+1 {
                st += "    ";
            }
            st += "end \n";
        }
    }
    for _ in 0..indent {
        st += "    ";
    }
    st += "endcase\n";
    return st;
}

/// Fsm構文出力関数--always文を生成する
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintFsm(Fsm: FsmModule) -> String {
    let mut st = String::new(); 
    let tmp = Fsm.clone();
    let CLK = tmp.clone().StateClk();
    let RST = tmp.clone().StateRst();
    let Reg = tmp.clone().StateReg();
    let p = tmp.clone().StateOut(); 
    st += &format!("    always@(posedge {} or posedge {}) begin\n", _StrOut(CLK.clone()), _StrOut(RST.clone()));
    st += &format!("        if ({} == 1) begin \n            {} <= {}; \n        end\n",_StrOut(RST.clone()), _StrOut(Reg.clone()), _StrOut(tmp.clone().FirstState()));
    st += &format!("        else begin \n            {} <= {}_Next; \n        end \n    end \n\n",_StrOut(Reg.clone()),_StrOut(Reg.clone()));
    st += &format!("    always@(posedge {}) begin\n",_StrOut(CLK.clone()));
    st += &format!("        if ({}) {}_Next <= {};\n", _StrOut(RST.clone()), _StrOut(Reg.clone()), _StrOut(tmp.clone().FirstState()));
    st += "        else begin\n";
    st += &format!("            case({})\n",_StrOut(Reg.clone()));
    for s in p {
        st += &PrintState(s.clone());
    }
    st += "            endcase \n        end\n    end\n\n";

    return st;
}

/// 1Stateモデル出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintState(STMT: StateModule) -> String {
    let mut s = STMT;
    let stname = s.getStateName();
    let tmp = s.getBranch();

    let mut st = String::new();

    st += &format!("                {} : begin\n",stname);
    st += &PrintIf(tmp.clone(), "Non", 5);
    st += "                end\n";

    return st;
}

/// AXIインタフェース出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintAXI(AXI_Sugar: AXI, num: i32) -> String {
    let tmp = AXI_Sugar.clone();
	let mut st = String::new();
    match tmp {
        AXI::Lite(x) => { st += &PrintAXISL(x, num);}
        AXI::Slave(_) => {unimplemented!();}
        AXI::Master(_) => {unimplemented!();}
        AXI::Stream(_) => {unimplemented!();}
    }
    return st;
}

/// AXISLite構文出力関数--ほぼテンプレ
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintAXISL(AXISL: AXISLite, count: i32) -> String {
	let tmp = AXISL.clone();
    let mut st = String::new();

    // register
	let reg_tmp = tmp.reg_array.clone();

    // address space
    let mut addr_width = 0;

    // address width
    let reg_length = tmp.reg_array.len() as i32;
    let mut reg_addr_width: i32 = 1;
    loop {
        if 2i32.pow(reg_addr_width as u32) >= (reg_length * 4 - 1) {
            break;
        }
        reg_addr_width += 1;
    }

	st += &format!("    // AXI Lite Slave port : Number {}\n", count);
    st += &format!("    reg r_en{};\n", count);
    st += &format!("    wire w_wdata_en{};\n", count);
    st += &format!("    wire w_rdata_en{};\n\n", count);

    // wready - waddress generating
    st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
    st += &format!("        if( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_wready{} <= 1'b0;\n            r_awready{0} <= 1'b0;\n            r_en{0} <= 1'b1;\n            r_awaddr{0} <= 0;\n",count);
    st += &format!("        end else begin\n");
    st += &format!("            if( ~r_wready{} && w_awvalid{0} && w_wvalid{0} && r_en{0} ) begin\n", count);
    st += &format!("                r_wready{0} <= 1'b1;\n            end else begin\n                r_wready{0} <= 1'b0;\n            end\n\n",count);
    st += &format!("            if( ~r_awready{} && w_awvalid{0} && w_wvalid{0} && r_en{0} ) begin\n", count);
    st += &format!("                r_awready{0} <= 1'b1;\n                r_en{0} <= 1'b0;\n                r_awaddr{0} <= i_S_AWADDR{0};\n", count);
    st += &format!("            end else begin\n");
    st += &format!("                if( w_bready{} && r_bvalid{0} ) begin\n", count);
    st += &format!("                    r_en{} <= 1'b1;\n                end\n", count);
    st += &format!("                r_awready{0} <= 1'b0;\n", count);
    st += &format!("            end\n        end\n    end\n\n");

    st += &format!("    assign w_wdata_en{} = r_awready{0} && r_wready{0} && w_awvalid{0} && w_wvalid{0};\n\n", count);
    
    // wdata generating
    st += &format!("    always@( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
    st += &format!("        if( {} ) begin\n", _StrOut(tmp.clone().rst));

	for x in tmp.reg_array.clone() {
		//println!("            r_{} <= 32'd0;", _StrOut(x));
        st += &format!("            r_{} <= 32'd0;\n", _StrOut(x));
	}
    st += &format!("        end\n        else begin\n            if( w_wdata_en{} == 1'd1 ) begin\n", count);
    st += &format!("                case ( w_waddr{}[{}:2] )\n", count, reg_addr_width-1);
    
    // generate write register
    for x in reg_tmp.clone() {
        // Unpack
        let mut reg = x;
        st += &format!("                    {}'h{:02X} : begin\n", reg_addr_width-2, addr_width);
        for addr_count in 0..4 {
            st += &format!("                        if ( w_wstrb{}[{}] == 1'b1 ) r_{} <= w_wdata{0}[{}:{}]\n",
			    count, addr_count, _StrOut(reg.clone()), 8*(addr_count+1)-1, 8*addr_count);
        }

        addr_width += 1;
        st += "                    end\n";
    }
    st += "                    default: begin\n";
	for x in reg_tmp.clone() {
        st += &format!("                        r_{} <= r_{}\n", 
            _StrOut(x.clone()), _StrOut(x.clone()));
	}
    st += "                    end\n                endcase\n            end\n";

	// Local write en
	let write_tmp = tmp.wLocal_write.clone();
	let mut i = -1;
	for x in write_tmp.clone() {
		i += 1;
		if let E::Null = *(x.0.clone()) {continue;}
        st += &format!("\n            if( {} ) begin \n", _StrOut(x.0));
        st += &format!("                    r_{} <= {};\n",
            _StrOut(reg_tmp[i as usize].clone()), _StrOut(x.1));
        st += "            end\n";
	}
    st += "        end\n    end\n\n";

    // wready - waddress generating
    st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
    st += &format!("        if( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_bvalid{} <= 1'b0;\n            r_bresp{0} <= 2'b0;\n",count);
    st += &format!("            r_arready{} <= 1'b0;\n            r_araddr{0} <= 0;\n",count);
    st += &format!("            r_rvalid{} <= 1'b0;\n            r_rresp{0} <= 2'b0;\n",count);
    st += "        end else begin\n";
    
    st += &format!("            if( r_awready{} && w_awvalid{0} && ~r_bvalid{0} && r_wready{0} && w_wvalid{0} ) begin\n", count);
    st += &format!("                r_bvalid{0} <= 1'b1;\n                r_bresp{0} <= 2'b1;\n            end else if( w_bready{0} && r_bvalid{0} ) begin\n                r_bvalid{0} <= 1'b;\n            end\n\n",count);

    st += &format!("            if( ~r_arready{} && w_arvalid{0} ) begin\n", count);
    st += &format!("                r_arready{0} <= 1'b1;\n                r_araddr{0} <= i_S_ARADDR{0};\n            end else begin\n                r_arready{0} <= 1'b0;\n            end\n", count);

    st += &format!("            if( r_arready{} && w_arvalid{0} && r_rvarid{0} ) begin\n", count);
    st += &format!("                r_rvarid{} <= 1'b1;\n                r_rresp{0} <= 2'b0;\n            end else if( r_rvalid{0} && w_rready{0} ) begin\n                r_rvalid{0} <= 1'b0;\n            end\n", count);
    st += "        end\n    end\n\n";

    // rdata generation
    st += &format!("    assign w_rdata_en{} = r_arready{0} && w_arvalid{0} && r_rvalid{0};\n\n", count);
    st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
    st += &format!("        if( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_rdata{} <= 32'd0; \n        end\n", count);
    st += "        else begin\n";
    st += &format!("            if( w_rdata_en{} ) begin\n", count);
    st += &format!("                case( w_wraddr[{}:2] )\n", reg_addr_width-1);

	// 配列の生成
	i = -1;
	for x in reg_tmp.clone() {
		i += 1;
        st += &format!("                    {}'h{:02X} : r_rdata{} <= {};\n", reg_addr_width-2, i, count, _StrOut(x.clone()));
	}

    st += &format!("                    default: r_rdata{} <= 32'hDEAD_DEAD;\n                endcase\n", count);
    st += "            end\n        end\n    end\n\n";

	return st;
}

/// NONAST
#[macro_export]
macro_rules! Blank {
    () => (Box::new(E::Null))
}


/// FSM生成関数
#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn Clock_Reset<T: Into<Box<E>>, U: Into<Box<E>>>(in_clk: T, in_rst: U) -> FsmModule {
    let p = wrVar::new().Reg("state", 32);
    FsmModule{clk: in_clk.into(), rst: in_rst.into(), fsm: p, State: Vec::new(), Current_state: 0}
}

/// FSMモジュール
#[derive(Debug,Clone)]
#[allow(non_snake_case)]
pub struct FsmModule {
    clk: Box<E>,
    rst: Box<E>,
    fsm: Box<E>,
    State: Vec<StateModule>,
    Current_state: i32,
}

#[allow(non_snake_case)]
impl FsmModule {
    fn FirstState(&mut self) -> Box<E> {
        self.State[0].getState()
    }
    // ステートレジスタの変更
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn State(&mut self, set_state: &str) -> FsmModule {
        self.fsm = wrVar::new().Reg(set_state, 32);
        self.clone()
    }

    // ステートの追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn AddState(&mut self, State_name: &str) -> FsmModule{
        let mut p = wrVar::new();
        self.Current_state = self.State.len() as i32;
        p.Parameter(State_name, self.Current_state);
        let tmp = StateModule{State: Box::new(E::Ldc(p)), Branch: Vec::new()};
        self.State.push(tmp);

        self.clone()
    }

    // カレントの移動
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Current(&mut self, State_name: &str) -> FsmModule {
        let mut count = 0;
        for x in &mut self.State {
            let Nx = x.getStateName();
            count+=1;
            if Nx == State_name.to_string() {
                self.Current_state = count;
            }
        }

        self.clone()
    }

    // カレントステートから次のステートへの定義
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn goto<T: Into<Box<E>>>(mut self, State_name: &str, Branch: T) -> FsmModule {
        
        let SelfS = self.fsm.clone();
        let mut st = "".to_string();
        if let E::Ldc(wr) = *SelfS.clone() { st = wr.getName().clone() };
        st = st + "_Next";
        let NState = wrVar::new().Reg(&st,0);
        let Goto_ = wrVar::new().Parameter(State_name,0);
        self.State[(self.Current_state as usize)].SetBranch(Branch.into(), F!(NState = Goto_));

        self.clone()
    }

    // 指定ステートからカレントステートへの定義(指定ステートの作成後以降に使用可)
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn from<T: Into<Box<E>>>(mut self, State_name: &str, Branch: T) -> FsmModule {
        let SelfS = self.fsm.clone();
        let mut st = "".to_string();
        if let E::Ldc(wr) = *SelfS.clone() { st = wr.getName().clone() };
        st = st + "_Next";
        let NState = wrVar::new().Reg(&st,0);
        let NameCurrentState = self.State[((self.Current_state-1) as usize)].getStateName();
        let branch = Branch.into();
        for x in &mut self.State {
            let Nx = x.getStateName();
            if Nx == State_name.to_string() {
                let Goto_ = wrVar::new().Parameter(&NameCurrentState,0);
                x.SetBranch(branch.clone(), F!(NState = Goto_));
            }
        }

        self.clone()
    }

    // セットパラメータの取得
    #[allow(dead_code)]
    pub fn Param(&mut self, name: &str) -> Box<E> {
        let SelfS = self.State.clone();
        for mut x in SelfS {
            let Nx = x.getStateName();
            if Nx == name.to_string() {
                return x.getState();
            }
        }
        return Box::new(E::Null);
    }

    // 内部メソッド(ステート格納レジスタを外部に出力)
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    fn StateReg(self) -> Box<E> {
        let tmp = self.clone();
        tmp.fsm
    }

    // 内部メソッド(クロックを外部に出力)
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    fn StateClk(self) -> Box<E> {
        let tmp = self.clone();
        tmp.clk
    }

    // 内部メソッド(リセットを外部に出力)
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    fn StateRst(self) -> Box<E> {
        let tmp = self.clone();
        tmp.rst
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    fn StateOut(self) -> Vec<StateModule>
    {
        let tmp = self.clone();
        tmp.State
    }
}

/// 1ステートモデル
#[derive(Debug,Clone)]
#[allow(non_snake_case)]
struct StateModule {
    State: Box<E>,
    Branch: Vec<IfStmt_AST>,
}

#[allow(non_snake_case)]
impl StateModule {
    // ステート設定
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    fn SetState(&mut self, stmt: Box<E>){
        self.State = stmt
    }

    // ステート分岐先設定
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    fn SetBranch<T: Into<Box<E>>, U: Into<Box<E>>>(&mut self, Terms: T, Form: U) -> bool {
        let e = *(Terms.into());
        let mut tmp = Vec::new();
        tmp.push(Form.into());
        
        match e {
            E::Null => self.Branch.push(IfStmt_AST{If_: true, IfE: Box::new(e), ST: tmp}),
            _ => self.Branch.push(IfStmt_AST{If_: true, IfE: Box::new(e), ST: tmp}),
        }
        return true;
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    fn getState(&mut self) -> Box<E> {
        let tmp = self.clone();
        tmp.State
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    fn getStateName(&mut self) -> String {
        let tmp = *(self.clone().State);
        match tmp {
            E::Ldc(b) => b.getName(),
            _ => "Nothing".to_string(),
        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    fn getBranch(&mut self) -> Vec<IfStmt_AST> {
        self.clone().Branch
    }
}

/// AXI wrapping enum
#[derive(Debug,Clone)]
#[allow(dead_code)]
enum AXI {
    Lite(AXISLite),
    Slave(AXIS),
    Master(AXIM),
    Stream(AXIST),
}

/// AXI Stream インタフェースの作成 - 未実装
#[derive(Debug,Clone)]
#[allow(non_snake_case)]
pub struct AXIST;

/// AXI Master インタフェースの作成 - 未実装
#[derive(Debug,Clone)]
#[allow(non_snake_case)]
pub struct AXIM;

/// AXI Slave インタフェースの作成 - 未実装
#[derive(Debug,Clone)]
#[allow(non_snake_case)]
pub struct AXIS;

/// AXI Slave Lite インタフェースの作成
#[derive(Debug,Clone)]
#[allow(non_snake_case)]
pub struct AXISLite {
	clk: Box<E>,
	rst: Box<E>,
	reg_array: Vec<Box<E>>,
    wLocal_write: Vec<(Box<E>, Box<E>)>,
	current_reg: i32,
}

/// AXI Slave Lite インターフェース生成
#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn AXIS_Lite_new<T: Into<Box<E>>, U: Into<Box<E>>>(clock: T, reset: U) -> AXISLite {
	AXISLite{
		clk: clock.into(),
		rst: reset.into(),
		reg_array: Vec::new(),
		wLocal_write: Vec::new(),
		current_reg: 0
	}
}

/// AXI IFのレジスタ設定トレイト
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub trait AXI_S_IF_Set<T> {
	// 数だけ指定してレジスタを生成
	fn OrderRegSet(&mut self, num: i32) -> T;

	// レジスタ名を指定してスタック式にレジスタを追加
	fn NamedRegSet(&mut self, name: &str) -> T;

	// レジスタ番号によるアクセスメソッド
	fn OrderReg(&mut self, num: i32) -> Box<E>;

	// 対応レジスタへのアクセスメソッド
	fn NamedReg(&mut self, name: &str) -> Box<E>;
}

/// ローカルからのレジスタ制御設定トレイト
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub trait AXI_S_IF_LocalWrite<T, U>
where
    U: Into<Box<E>>,
{
    fn RegWrite(&mut self, write_en: U, write_data: T);
}

impl AXI_S_IF_Set<AXISLite> for AXISLite {
	#[allow(non_snake_case)]
	fn OrderRegSet(&mut self, num: i32) -> AXISLite {
		for x in 0..num {
			let Regname = format!("{}{}", "slv_reg".to_string(), x.to_string());
			let reg = wrVar::new().Reg(&Regname, 32);
			self.reg_array.push(reg);
			self.wLocal_write.push((Box::new(E::Null), Box::new(E::Null)));
		}
		self.current_reg = num-1;
		self.clone()
	}

	#[allow(non_snake_case)]
	fn NamedRegSet(&mut self, name: &str) -> AXISLite {
		let reg = wrVar::new().Reg(name, 32);
		self.reg_array.push(reg);
		self.wLocal_write.push((Box::new(E::Null), Box::new(E::Null)));
		self.current_reg = self.reg_array.len() as i32 - 1;
		self.clone()
	}

	#[allow(non_snake_case)]
	fn OrderReg(&mut self, num: i32) -> Box<E> {
		let SelfReg = self.reg_array.clone();
		return SelfReg[num as usize].clone();
	}

	#[allow(non_snake_case)]
	fn NamedReg(&mut self, name: &str) -> Box<E> {
		let SelfReg = self.reg_array.clone();
		for mut x in SelfReg {
			let Nx = *x.clone();
			if let E::Ldc(i) = Nx {
				if i.getName() == name.to_string() {
					return x
				}
			}
		}
		return Box::new(E::Null)
	}
}

/// AXIS Lite ローカル側データ書き込み処理設定
impl<T, U> AXI_S_IF_LocalWrite<T, U> for AXISLite
where
    T: Into<Box<E>>,
    U: Into<Box<E>>,
{
    fn RegWrite(&mut self, write_en: U, write_data: T) {
		// localwrite AXI Register
		self.wLocal_write[self.current_reg.clone() as usize] = (write_en.into(), write_data.into());
		return;
	}
}


// 基本Box<E>の分解に使用

/// AST分解メソッド
#[allow(non_snake_case)]
pub fn _Decomp<T: Into<Box<E>>>(e: T, Sel: &str) -> Box<E> {
    let m = *e.into();
    match m {
        E::Bin(_, ref L, ref R) => {
            if Sel == "L" {Box::new(*L.clone())}
            else if Sel == "R" {Box::new(*R.clone())}
            else {Box::new(E::Null)}
        },
        E::PL(ref D, ref T, ref F) => {
            if Sel == "D" {Box::new(*D.clone())}
            else if Sel == "T" {Box::new(*T.clone())}
            else if Sel == "F" {Box::new(*F.clone())}
            else {Box::new(E::Null)}
        },
        E::SB(ref L, ref R) => {
            if Sel == "L" {Box::new(*L.clone())}
            else if Sel == "R" {Box::new(*R.clone())}
            else {Box::new(E::Null)}
        }
        _ => Box::new(E::Null),
    }
}

/// AST文字列抽出メソッド
#[allow(non_snake_case)]
pub fn _StrOut<T: Into<Box<E>>>(e: T) -> String {
    let m = *e.into();
    match m {
        E::Ldc(WR) => WR.getName(),
        E::Bin(ref Op, _, _) => Op.clone(),
        _ => "Null".to_string(),
    }
}

/// AST数値抽出メソッド
#[allow(non_snake_case)]
pub fn _NumOut<T: Into<Box<E>>>(e: T) -> i32 {
    let m = *e.into();
    match m {
        E::Ldc(WR) => WR.getWidth(),
        E::Num(i) => i,
        _ => 0,
    }
}
