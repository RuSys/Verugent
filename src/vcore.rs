use std::ops::*;
use std::string::String;

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
}

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

/// 入力幅：i32
#[allow(dead_code)]
#[allow(non_snake_case)]
impl Vset<i32> for VModule{
    /// input の追加
    fn Input(&mut self, name: &str, Width: i32) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Input(name, Width);
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// inout の追加
    fn Inout(&mut self, name: &str, Width: i32) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Inout(name, Width);
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// output の追加
    fn Output(&mut self, name: &str, Width: i32) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Output(name, Width);
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// output(register) の追加
    fn Reg_Output(&mut self, name: &str, Width: i32) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.OutputReg(name, Width);
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// wire の追加
    fn Wire(&mut self, name: &str, Width: i32) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Wire(name, Width);
        self.Local.push(tmp.clone());
        return _V(tmp);
    }

    /// reg の追加
    fn Reg(&mut self, name: &str, Width: i32) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Reg(name, Width);
        self.Local.push(tmp.clone());
        return _V(tmp);
    }
}

/// 入力幅：Box<E>
#[allow(dead_code)]
#[allow(non_snake_case)]
impl Vset<Box<E>> for VModule{
    /// input の追加
    fn Input(&mut self, name: &str, Width: Box<E>) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Input(name, 0);
        if let E::Ldc(wr) = *Width { tmp.Width( &( wr.getWP() ) ); };
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// inout の追加
    fn Inout(&mut self, name: &str, Width: Box<E>) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Inout(name, 0);
        if let E::Ldc(wr) = *Width { tmp.Width( &( wr.getWP() ) ); };
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// output の追加
    fn Output(&mut self, name: &str, Width: Box<E>) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Output(name, 0);
        if let E::Ldc(wr) = *Width { tmp.Width( &( wr.getWP() ) ); };
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// output(register) の追加
    fn Reg_Output(&mut self, name: &str, Width: Box<E>) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.OutputReg(name, 0);
        if let E::Ldc(wr) = *Width { tmp.Width( &( wr.getWP() ) ); };
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// wire の追加
    fn Wire(&mut self, name: &str, Width: Box<E>) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Wire(name, 0);
        if let E::Ldc(wr) = *Width { tmp.Width( &( wr.getWP() ) ); };
        self.Local.push(tmp.clone());
        return _V(tmp);
    }

    /// reg の追加
    fn Reg(&mut self, name: &str, Width: Box<E>) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Reg(name, 0);
        if let E::Ldc(wr) = *Width { tmp.Width( &( wr.getWP() ) ); };
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
        VModule{Module_Name: Name.to_string(), IO_Port: Vec::new(), IO_Param: Vec::new(),Local: Vec::new(),Always_AST: Vec::new(),Assign_AST: Vec::new(), Function_AST: Vec::new(),Fsm: Vec::new()}
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
            self.Local.push(wrVar{name: ss.getStateName(), io_param: io_p::param_, width: 0, length: 0, reg_set: false, value: n, width_p: "_".to_string(), length_p: "_".to_string()});
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
    pub fn endmodule(&mut self) {
        print!("module {} ",self.getName());
        // 入出力パラメータ出力コード
        PrintParam(self.IO_Param.clone());

        // 入出力ポート出力コード
        PrintPort(self.IO_Port.clone());

        // 内部パラメータおよび内部配線出力コード
        PrintLocal(self.Local.clone());

        // Assign構文出力コード
        PrintAssign(self.Assign_AST.clone());

        // Always構文出力コード
        PrintAlways(self.Always_AST.clone());

        // Function構文出力コード
        PrintFunction(self.Function_AST.clone());

        // FSMの出力コード
        if self.Fsm.clone().len() > 0 {
            for tmp in self.Fsm.clone() {
                PrintFsm(tmp.clone());
            }
        }

        println!("\nendmodule");
    }
}

/// メモリレジスタ生成用のトレイト
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub trait Memset<T> {
    fn Mem(&mut self, name: &str, args: T) -> Box<E>;
}

/// 入力(i32:Box<E>)生成するメモリ構文
impl Memset<(i32, Box<E>)> for VModule{
    /// メモリ構文
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Mem(&mut self, name: &str, args: (i32, Box<E>)) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Mem(name, args.0, 0);
        if let E::Ldc(wr) = *args.1 { tmp.Length( &( wr.getName() ) ); };
        self.Local.push(tmp.clone());
        return _V(tmp);
    }
}

/// 入力(Box<E>:i32)生成するメモリ構文
impl Memset<(Box<E>, i32)> for VModule{
    /// メモリ構文
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Mem(&mut self, name: &str, args: (Box<E>, i32)) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Mem(name, 0, args.1);
        if let E::Ldc(wr) = *args.0 { tmp.Width( &( wr.getName() ) ); };
        self.Local.push(tmp.clone());
        return _V(tmp);
    }
}

/// 入力(i32:i32)生成するメモリ構文
impl Memset<(i32, i32)> for VModule{
    /// メモリ構文
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Mem(&mut self, name: &str, args: (i32, i32)) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Mem(name, args.0, args.1);
        self.Local.push(tmp.clone());
        return _V(tmp);
    }
}

/// 入力(Box<E>:Box<E>)生成するメモリ構文
impl Memset<(Box<E>, Box<E>)> for VModule{
    /// メモリ構文
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Mem(&mut self, name: &str, args: (Box<E>, Box<E>)) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Mem(name, 0, 0);
        if let E::Ldc(wr) = *args.0 { tmp.Width( &( wr.getName() ) ); };
        if let E::Ldc(wr) = *args.1 { tmp.Length( &( wr.getName() ) ); };
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
pub trait SetEqual {
     fn _e(&mut self, RHS: Box<E>) -> Assign;

     fn _ve(&mut self, RHS: Box<E>) -> Assign;
}

/// Assign 構文代入用トレイト
impl SetEqual for Box<E> {
    /// Box<E>からAssign生成を行うメソッド
    #[allow(non_snake_case)]
    fn _e(&mut self, RHS: Box<E>) -> Assign {
        let mut tmp = Assign::new();
        tmp.L(self.clone()).R(RHS)
    }

    #[allow(non_snake_case)]
    fn _ve(&mut self, RHS: Box<E>) -> Assign {
        let mut tmp = Assign::new();
        tmp.L(self.clone()).R(RHS)
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
    pub fn L(&mut self, LHS :Box<E>) -> Assign {
        self.lhs = LHS;
        let tmp = self.clone();
        return tmp;
    }

    /// 右辺設定メソッド
    #[allow(non_snake_case)]
    pub fn R(&mut self, RHS : Box<E>) -> Assign {
        self.rhs = RHS;
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
pub fn Posedge(edge: Box<E>) -> Always {
    let e = *edge;
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
pub fn Negedge(edge: Box<E>) -> Always {
    let e = *edge;
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
    pub fn Posedge(&mut self, edge: Box<E>) -> Always {
        let e = *edge;
        match e {
            E::Ldc(wr) => self.P_edge.push(wr.clone()),
            _ => return self.clone(),
        }
        self.clone()
    }

    /// 立ち下がり信号設定
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Negedge(&mut self, edge: Box<E>) -> Always {
        let e = *edge;
        match e {
            E::Ldc(wr) => self.N_edge.push(wr.clone()),
            _ => return self.clone(),
        }
        self.clone()
    }

    /// 分岐 if 構文追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn If(&mut self, C: Box<E>, S: Vec<Box<E>>) -> Always {
        let i = If(C, S);
        self.stmt.push(i);
        self.clone()
    }

    /// 分岐 else if 構文追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Else_If(&mut self, C: Box<E>, S: Vec<Box<E>>) -> Always {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: true, IfE: C.clone(), ST: S});
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
    pub fn Case(&mut self, Sel: Box<E>) -> Always {
        let c = Case(Sel);
        self.stmt.push(c);
        self.clone()
    }

    /// Case文内の分岐追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn S(&mut self, C: Box<E>, S: Vec<Box<E>>) -> Always {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(C, S);
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
    pub fn If(&mut self, C: Box<E>, S: Vec<Box<E>>) -> Func_AST {
        let i = If(C, S);
        self.stmt.push(i);
        self.clone()
    }

    /// 分岐 else if 構文追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn Else_If(&mut self, C: Box<E>, S: Vec<Box<E>>) -> Func_AST {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: true, IfE: C.clone(), ST: S});
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
    pub fn Case(&mut self, Sel: Box<E>) -> Func_AST {
        let c = Case(Sel);
        self.stmt.push(c);
        self.clone()
    }

    /// Case 文内の分岐追加
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn S(&mut self, C: Box<E>, S: Vec<Box<E>>) -> Func_AST {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(C, S);
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
pub fn If(C: Box<E>, S: Vec<Box<E>>) -> Box<E> {
    let mut i = Vec::new();
    i.push(IfStmt_AST{If_: true, IfE: C.clone(), ST: S});
    Box::new(E::BL(i))
}

/// ステートメントブロック内のif分岐追加
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub trait Ifset {
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Else_If(self, C: Box<E>, S: Vec<Box<E>>) -> Box<E>;

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Else(self, S: Vec<Box<E>>) -> Box<E>;
}


impl Ifset for Box<E> {
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Else_If(self, C: Box<E>, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut p;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: true, IfE: C.clone(), ST: S});
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
    pub fn SetCaseS(&mut self, Cond: Box<E>, Stmt: Vec<Box<E>>) {
        self.Select.push(Case_{CaseT: Cond, CaseS: Stmt})
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
fn Case(Sel: Box<E>) -> Box<E> {
    let e = *Sel;
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
    fn S(self, C: Box<E>, S: Vec<Box<E>>) -> Box<E>;

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn Default(self, S: Vec<Box<E>>) -> Box<E>;
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
impl Caseset for Box<E> {
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    fn S(self, C: Box<E>, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut n;
        match e {
            E::CS(csast) => {
                n = csast;
            },
            _ => return Box::new(E::Null),
        }
        n.SetCaseS(C, S);
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
pub fn Form(formu: Box<E>) -> Vec<Box<E>>{
    let mut tmp = Vec::new();
    tmp.push(formu);
    return tmp
}

/// ステートメントブロック内の式追加
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub trait addForm {
     fn Form(self, formu: Box<E>) -> Vec<Box<E>>;
}

impl addForm for Vec<Box<E>> {
    fn Form(self, formu: Box<E>) -> Vec<Box<E>> {
        let mut tmp = self;
        tmp.push(formu);
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
    Bin(String, Box<E>, Box<E>),    // 二項演算
    PL(Box<E>, Box<E>, Box<E>),     // 分岐構文
    SB(Box<E>, Box<E>),             // 代入文
    CS(CaseStmt_AST),               // case文
    BL(Vec<IfStmt_AST>),            // if, else if, else文
    Func(Box<E>, Vec<Box<E>>),      // function文
    MEM(Box<E>,Box<E>),             // メモリ
    Node(String),                   // 内部検索用
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
pub fn _Veq(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::SB(L, R))
}

// 分岐構文関数
#[allow(non_snake_case)]
pub fn _Branch(Terms: Box<E>, TrueNode: Box<E>, FalseNode: Box<E>) -> Box<E> {
    Box::new(E::PL(Terms, TrueNode, FalseNode))
}


// 演算子関数
/// "+" addition
#[allow(non_snake_case)]
fn _Add(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "add".to_string(), L, R))
}

/// "-" substruction
#[allow(non_snake_case)]
fn _Sub(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "sub".to_string(), L, R))
}

/// "*" multipication
#[allow(non_snake_case)]
fn _Mul(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "mul".to_string(), L, R))
}

/// "/" division
#[allow(non_snake_case)]
fn _Div(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "div".to_string(), L, R))
}

/// "%" modulo
#[allow(non_snake_case)]
fn _Mod(L : Box<E>, R : Box<E>) -> Box<E> {
    Box::new(E::Bin( "mod".to_string(), L, R))
} 

/// "||" or
#[allow(non_snake_case)]
fn _LOr(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "lor".to_string(), L, R))
}

/// "&&" and
#[allow(non_snake_case)]
fn _LAnd(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "land".to_string(), L, R))
}

/// "|" or
#[allow(non_snake_case)]
fn _Or(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "or".to_string(), L, R))
}

/// "&" and
#[allow(non_snake_case)]
fn _And(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "and".to_string(), L, R))
}

/// "^" exclusive or
#[allow(non_snake_case)]
fn _Xor(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "xor".to_string(), L, R))
}

/// "==" equal
#[allow(non_snake_case)]
pub fn _Eq(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "equal".to_string(), L, R))
}

/// "!=" not equal
#[allow(non_snake_case)]
pub fn _Neq(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "Not equal".to_string(), L, R))
}

/// "<<" left shift
#[allow(non_snake_case)]
fn _LSH(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "lshift".to_string(), L, R))
}

/// ">>" right shift
#[allow(non_snake_case)]
fn _RSH(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "rshift".to_string(), L, R))
}

/// "<" more than
#[allow(non_snake_case)]
fn _MTH(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "more_than".to_string(), L, R))
}

/// ">" less than
#[allow(non_snake_case)]
fn _LTH(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "less_than".to_string(), L, R))
}

/// "<=" or more
#[allow(non_snake_case)]
fn _OMR(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "or_more".to_string(), L, R))
}

/// ">=" or less
#[allow(non_snake_case)]
fn _OLS(L : Box<E>, R : Box<E>) -> Box<E>{
    Box::new(E::Bin( "or_less".to_string(), L, R))
}

/**
  * 演算子実装メソッド
  *
  **/
pub trait Notc {
    fn not(&mut self) -> Box<E>;
}

impl Notc for Box<E> {
    fn not(&mut self) -> Box<E> {
        Box::new(E::No(self.clone()))
    }
}

impl Not for Box<E> {
    type Output = Box<E>;

    fn not(self) -> Box<E> {
        Box::new(E::No(self.clone()))
    }
}

impl Add<Box<E>> for Box<E> {
    type Output = Box<E>;

    fn add(self, other: Box<E>) -> Box<E> {
        _Add(self, other)
    }
}

impl Add<i32> for Box<E> {
    type Output = Box<E>;

    fn add(self, other: i32) -> Box<E> {
        _Add(self, _Num(other))
    }
}

impl Add<Box<E>> for i32 {
    type Output = Box<E>;

    fn add(self, other: Box<E>) -> Box<E> {
        _Add(_Num(self), other)
    }
}

impl Sub<Box<E>> for Box<E> {
    type Output = Box<E>;

    fn sub(self, other: Box<E>) -> Box<E> {
        _Sub(self, other)
    }
}

impl Sub<i32> for Box<E> {
    type Output = Box<E>;

    fn sub(self, other: i32) -> Box<E> {
        _Sub(self, _Num(other))
    }
}

impl Sub<Box<E>> for i32 {
    type Output = Box<E>;

    fn sub(self, other: Box<E>) -> Box<E> {
        _Sub(_Num(self), other)
    }
}

impl Mul<Box<E>> for Box<E> {
    type Output = Box<E>;

    fn mul(self, other: Box<E>) -> Box<E> {
        _Mul(self, other)
    }
}

impl Mul<i32> for Box<E> {
    type Output = Box<E>;

    fn mul(self, other: i32) -> Box<E> {
        _Mul(self, _Num(other))
    }
}

impl Mul<Box<E>> for i32 {
    type Output = Box<E>;

    fn mul(self, other: Box<E>) -> Box<E> {
        _Mul(_Num(self), other)
    }
}

impl Div<Box<E>> for Box<E> {
    type Output = Box<E>;

    fn div(self, other: Box<E>) -> Box<E> {
        _Div(self, other)
    }
}

impl Div<i32> for Box<E> {
    type Output = Box<E>;

    fn div(self, other: i32) -> Box<E> {
        _Div(self, _Num(other))
    }
}

impl Div<Box<E>> for i32 {
    type Output = Box<E>;

    fn div(self, other: Box<E>) -> Box<E> {
        _Div(_Num(self), other)
    }
}

impl Rem<Box<E>> for Box<E> {
    type Output = Box<E>;

    fn rem(self, other: Box<E>) ->Box<E> {
        _Mod(self, other)
    }
}

impl Rem<i32> for Box<E> {
    type Output = Box<E>;

    fn rem(self, other: i32) -> Box<E> {
        _Mod(self, _Num(other))
    }
}

impl Rem<Box<E>> for i32 {
    type Output = Box<E>;

    fn rem(self, other: Box<E>) -> Box<E> {
        _Mod(_Num(self), other)
    }
}

impl BitOr<Box<E>> for Box<E> {
    type Output = Box<E>;

    fn bitor(self, other: Box<E>) -> Box<E> {
        _Or(self, other)
    }
}

impl BitOr<i32> for Box<E> {
    type Output = Box<E>;

    fn bitor(self, other: i32) -> Box<E> {
        _Or(self, _Num(other))
    }
}

impl BitOr<Box<E>> for i32 {
    type Output = Box<E>;

    fn bitor(self, other: Box<E>) -> Box<E> {
        _Or(_Num(self), other)
    }
}

impl BitAnd<Box<E>> for Box<E> {
    type Output = Box<E>;

    fn bitand(self, other: Box<E>) -> Box<E> {
        _And(self, other)
    }
}

impl BitAnd<i32> for Box<E> {
    type Output = Box<E>;

    fn bitand(self, other: i32) -> Box<E> {
        _And(self, _Num(other))
    }
}

impl BitAnd<Box<E>> for i32 {
    type Output = Box<E>;

    fn bitand(self, other: Box<E>) -> Box<E> {
        _And(_Num(self), other)
    }
}

impl BitXor<Box<E>> for Box<E> {
    type Output = Box<E>;

    fn bitxor(self, other: Box<E>) -> Box<E> {
        _Xor(self, other)
    }
}

impl BitXor<i32> for Box<E> {
    type Output = Box<E>;

    fn bitxor(self, other: i32) -> Box<E> {
        _Xor(self, _Num(other))
    }
}

impl BitXor<Box<E>> for i32 {
    type Output = Box<E>;

    fn bitxor(self, other: Box<E>) -> Box<E> {
        _Xor(_Num(self), other)
    }
}

impl Shl<Box<E>> for Box<E> {
    type Output = Box<E>;

    fn shl(self, other: Box<E>) -> Box<E> {
        _LSH(self, other)
    }
}

impl Shl<i32> for Box<E> {
    type Output = Box<E>;

    fn shl(self, other: i32) -> Box<E> {
        _LSH(self, _Num(other))
    }
}

impl Shl<Box<E>> for i32 {
    type Output = Box<E>;

    fn shl(self, other: Box<E>) -> Box<E> {
        _LSH(_Num(self), other)
    }
}

impl Shr<Box<E>> for Box<E> {
    type Output = Box<E>;

    fn shr(self, other: Box<E>) -> Box<E> {
        _RSH(self, other)
    }
}

impl Shr<i32> for Box<E> {
    type Output = Box<E>;

    fn shr(self, other: i32) -> Box<E> {
        _RSH(self, _Num(other))
    }
}

impl Shr<Box<E>> for i32 {
    type Output = Box<E>;

    fn shr(self, other: Box<E>) -> Box<E> {
        _RSH(_Num(self), other)
    }
}

// Equal,Not Equal構文生成
pub trait PartialEq<Rhs = Self> {

    fn eq(self, other: Rhs) -> Box<E>;

    fn ne(self, other: Rhs) -> Box<E>;
}

impl PartialEq<Box<E>> for Box<E> {

    fn eq(self, other: Box<E>) -> Box<E> {
        _Eq(self, other)
    }

    fn ne(self, other: Box<E>) -> Box<E> {
        _Neq(self, other)
    }
}

impl PartialEq<i32> for Box<E> {

    fn eq(self, other: i32) -> Box<E> {
        _Eq(self, _Num(other))
    }

    fn ne(self, other: i32) -> Box<E> {
        _Neq(self, _Num(other))
    }
}

// compare構文生成
pub trait PartialOrd<Rhs = Self>{
    fn lt(self, other: Rhs) -> Box<E>;

    fn le(self, other: Rhs) -> Box<E>;

    fn gt(self, other: Rhs) -> Box<E>;

    fn ge(self, other: Rhs) -> Box<E>;
}

impl PartialOrd<Box<E>> for Box<E> {
    fn lt(self, other: Box<E>) -> Box<E>{
        _LTH(self, other)
    }

    fn le(self, other: Box<E>) -> Box<E>{
        _OLS(self, other)
    }

    fn gt(self, other: Box<E>) -> Box<E>{
        _MTH(self, other)
    }

    fn ge(self, other: Box<E>) -> Box<E>{
        _OMR(self, other)
    }
}

impl PartialOrd<i32> for Box<E> {
    fn lt(self, other: i32) -> Box<E>{
        _LTH(self, _Num(other))
    }

    fn le(self, other: i32) -> Box<E>{
        _OLS(self, _Num(other))
    }

    fn gt(self, other: i32) -> Box<E>{
        _MTH(self, _Num(other))
    }

    fn ge(self, other: i32) -> Box<E>{
        _OMR(self, _Num(other))
    }
}

// 代入文生成
pub trait Subs<Rhs = Self> {
    fn sst(self, other: Rhs) -> Box<E>;
}

impl Subs<i32> for Box<E> {
    fn sst(self, other: i32) -> Box<E> {
        _Veq(self.clone(), _Num(other))
    }
}

impl Subs<Box<E>> for Box<E> {
    fn sst(self, other: Box<E>) -> Box<E> {
        _Veq(self.clone(), other)
    }
}

// 論理演算子生成
pub trait Logi<Rhs = Self> {
    fn land(self, other: Rhs) -> Box<E>;

    fn lor(self, other: Rhs) -> Box<E>;
}

impl Logi<Box<E>> for Box<E> {
    fn land(self, other: Box<E>) -> Box<E> {
        _LAnd(self.clone(), other)
    }

    fn lor(self, other: Box<E>) -> Box<E> {
        _LOr(self.clone(), other)
    }
}

// メモリ、レジスタ用アドレス指定
pub trait Addr<Rs = Self> {
    fn addr(&mut self, address: Rs) ->Box<E>;
}

impl Addr<i32> for Box<E>{
    fn addr(&mut self, address: i32) -> Box<E> {
        Box::new(E::MEM(self.clone(), _Num(address)))
    }
}

impl Addr<Box<E>> for Box<E>{
    fn addr(&mut self, address: Box<E>) -> Box<E> {
        Box::new(E::MEM(self.clone(), address))
    }
}

/**
  * 出力、分解、デバッグ関数
  * 出力関数以外はデバッグ用関数のため削除しても問題はない
  **/

/// 分解出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn DeconpAST(Parenthesis: bool, ast: Box<E>, cnfg: &str, indent: i32) {
    let e = *ast;
    match e {
        E::Bin(ref bin, ref l, ref r) => {
            let tmp = bin.as_str();
            for _ in 0..indent {
                print!("    ");
            }
            if Parenthesis {
                match tmp.clone() {
                    "add" => print!("("),
                    "sub" => print!("("),
                    "or" => print!("("),
                    "lor" =>print!("("),
                    _ => print!(""),
                }
            }
            let mut pareset = false;
            DeconpAST(false ,l.clone(),cnfg, 0);
            match tmp.clone() {
                "add" => print!("+"),
                "sub" => print!("-"),
                "mul" => {print!("*"); pareset = true},
                "div" => {print!("/"); pareset = true},
                "mod" => {print!("%"); pareset = true},
                "or"  => print!("|"),
                "and" => {print!("&"); pareset = true},
                "lor"  => print!("||"),
                "land" => {print!("&&"); pareset = true},
                "lshift" => print!("<<"),
                "rshift" => print!(">>"),
                "equal" => print!("=="),
                "Not equal" => print!("!="),
                "more_than" => print!("<"),
                "less_than" => print!(">"),
                "or_more" => print!("<="),
                "or_less" => print!(">="),
                _ => panic!("No correspond syntax"),
            }
            DeconpAST(pareset, r.clone(),cnfg, 0);
            if Parenthesis {
                match tmp {
                    "add" => print!(")"),
                    "sub" => print!(")"),
                    "or" => print!(")"),
                    "lor" =>print!(")"),
                    _ => print!(""),
                }
            }
        }
        E::Ldc(ref wr) => {
            print!("{}",wr.getName());
        },
        E::Num(ref i) => {
            print!("{}",i);
        },
        E::PL(ref d, ref t, ref f) => {
            print!("(");
            DeconpAST(false,d.clone(),cnfg, 0);
            print!(")? ");
            DeconpAST(false, t.clone(),cnfg, 0);
            print!(": ");
            DeconpAST(false, f.clone(),cnfg, 0);
            print!("");
        },
        E::SB(ref l, ref r) => {
            for _ in 0..indent {
                print!("    ");
            }
            DeconpAST(false, l.clone(),cnfg, indent);
            if cnfg.to_string() == "brock".to_string() {
                print!(" = ");
            }
            else {
                print!(" <= ");
            }
            DeconpAST(false, r.clone(),cnfg, 0);
            println!(";");
        },
        E::CS(ref c) => {
            let cn = &*c;
            PrintCase(cn.clone(),cnfg, indent);
        }
        E::BL(ref i) => {
            let iels = &*i;
            PrintIf(iels.clone(),cnfg, indent);
        }
        E::MEM(ref m, ref a) => {
            let ma = &*m;
            let aa = &*a;
            DeconpAST(false, ma.clone(),cnfg, indent);
            print!("[");
            DeconpAST(false, aa.clone(),cnfg, 0);
            print!("]");
        }
        E::No(ref b) => {
            let bb = &*b;
            print!("~");
            DeconpAST(false, bb.clone(),cnfg, 0);
        }
        _ => {
            return
        }
    }
}

/// GlobalParameter出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintParam(Param: Vec<wrVar>) -> bool {
    let tmp = Param;
    let n = tmp.len();
    let mut num = 0;
    if n != 0 {
        println!("#(");
    }

    for x in tmp {
        num += 1;
        print!("    parameter {} = {}",x.getName(), x.getValue());
        if n > num {
            println!(",");
        }
        else {
            println!("");
        }
    }
    if n != 0 {
        println!(")");
    }
    return true;
}

/// 入出力ポート出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintPort(Port: Vec<wrVar>) -> bool {
    let tmp = Port;
    let n = tmp.len();
    let mut num = 0;
    println!("(");
        for x in tmp {
            let port_set = x.getIO();
            num += 1;
            match port_set {
                io_p::input_ => {
                    print!("    input ");
                }
                io_p::output_ => {
                    if x.getReg() {
                        print!("    output reg ");
                    }
                    else {
                        print!("    output ");
                    }
                }
                io_p::inout_ => {
                    print!("    inout ")
                }
                _ => return false
            }

            if x.getWidth() == 0 && x.getWP() != "_" {
                print!("[{}-1:0] ",x.getWP());
            }
            else if x.getWidth() > 1 {
                print!("[{}:0] ",x.getWidth()-1);
            }
            else {
                print!(" ");
            }

            print!("{}",x.getName());

            if x.getLength() == 0 && x.getLP() != "_" {
                print!(" [{}-1:0]",x.getLP());
            }
            else if x.getLength() != 0 {
                print!(" [{}:0]",x.getLength()-1);
            }
            else {
                print!("");
            }

            if n > num {
                println!(",");
            }
            else {
                println!("");
            }
        }
        println!(");");
        return true;
}

/// LocalParameter + Wire + Reg出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintLocal(PWR: Vec<wrVar>) -> bool {
    let tmp = PWR;
    for x in tmp {
        let port_set = x.getIO();
        match port_set {
            io_p::param_ => {
                println!("    localparam {} = {};",x.getName(), x.getValue());
            }
            io_p::none => {
                if x.getReg() {
                    print!("    reg ");
                }
                else {
                    print!("    wire ");
                }
                if x.getWidth() == 0 && x.getWP() != "_".to_string() {
                    print!("[{}-1:0] ",x.getWP());
                }
                else if x.getWidth() > 1 {
                    print!("[{}:0] ",x.getWidth()-1);
                }
                else {
                    print!(" ");
                }

                print!("{}",x.getName());

                if x.getLength() == 0 && x.getLP() != "_".to_string() {
                    print!(" [{}-1:0]",x.getLP());
                }
                else if x.getLength() != 0 {
                    print!(" [{}:0]",x.getLength()-1);
                }
                else {
                    print!("");
                }
                print!(";\n");
            }
            _ => return true
        }
    }
    return true;
}

/// assign出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintAssign(Assign: Vec<Assign>) -> bool {
    let tmp = Assign;
    for mut x in tmp {
        let LO = x.LOut();
        print!("    assign ");
        DeconpAST(false, LO, "", 0);
        print!(" = ");
        let port_set = x.ROut();
        DeconpAST(false, port_set, "", 0);
        println!(";");
    }
    println!("");
    return true
}

/// always出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintAlways(Always: Vec<Always>) -> bool {
    let tmp = Always.clone();
    for x in tmp {
        print!("    always@(");
        let mut n = x.P_edge.clone();
        let mut tmp_num = 1;
        let mut len = n.len();
        for y in n{
            print!("posedge {}",y.getName());
            if len > tmp_num {
                print!(" or ");
            }
            tmp_num += 1;
        }

        n = x.N_edge.clone();
        len = n.len();
        if tmp_num > 1 && len > 0 {print!(" or ")}
        tmp_num = 1;
        for y in n {
            print!("negedge {}",y.getName());
            if len > tmp_num {
                print!(" or ");
            }
            tmp_num += 1;
        }
        print!(") begin\n");
        for st in x.stmt.clone() {
            DeconpAST(false, st,&x.clone().blockout(), 2);
        }
        
        println!("    end")
    }
    return true
}

/// function出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintFunction(Function: Vec<Func_AST>) -> bool {
    let tmp = Function.clone();
    for x in tmp {
        let e = x.top;
        if let E::Ldc(wrtop) = (*e).clone() {
            print!("\n    function [{}:0] ", wrtop.getWidth()-1);
            DeconpAST(false, e, "", 1);
            println!(";");
        }
        for inpt in x.input {
            let i = inpt.clone();
            if let E::Ldc(wr) = (*i).clone() {
                if wr.getWidth() > 0 {
                    print!("        input [{}:0]", wr.getWidth()-1);
                    DeconpAST(false, i, "",2);
                    println!(";");
                }
                else {
                    println!("        input ");
                    DeconpAST(false, i, "", 2);
                    println!(";");
                }
            }
        }
        for st in x.stmt {
            DeconpAST(false, st, "", 2);
        }
        println!("    endfunction\n");
    }
    return true;
}

/// if_else構文出力関数--ブロック出力関数より呼び出し
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintIf(If_Stmt: Vec<IfStmt_AST>, cnfg: &str, indent: i32) -> bool {
    let tmp = If_Stmt;
    let mut num = 0;
    
    for mut x in tmp {
        let n = x.getStatement();
        if num == 0 {
            let e = *x.clone().getTerms();
            match e {
                E::Null => {
                    num = 0;
                }
                _ => {
                    for _ in 0..indent {
                        print!("    ");
                    }
                    print!("if(");
                    num += 1;
                    DeconpAST(false, x.getTerms(), "", 0);
                    print!(")\n");
                }
            }
        }
        else if x.getIfFlag() {
            for _ in 0..indent {
                print!("    ");
            }
            print!("else if(");
            DeconpAST(false, x.getTerms(), "",0);
            print!(")\n");
        }
        else {
            for _ in 0..indent {
                print!("    ");
            }
            print!("else\n");
        }

        if n.len() > 1 {
            for _ in 0..indent {
                print!("    ")
            }
            print!("begin\n")
            }
        for y in n.clone() {
            DeconpAST(false, y,cnfg, indent + 1);
        }
        if n.len() > 1 {
            for _ in 0..indent {
                print!("    ")
            }
            print!("end\n")
            }
    }
    return true;
}

/// Case構文出力関数--ブロック出力関数より呼び出し
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintCase(case_stmt: CaseStmt_AST, cnfg: &str, indent: i32) -> bool {
    let mut tmp = case_stmt;
    let ctmp = tmp.clone().Select;
    for _ in 0..indent {
        print!("    ");
    }
    println!("case ({})",tmp.getCaseV().getName());
    for x in ctmp {
        let e = x.CaseT.clone();
        let ef = x.CaseS.clone();
        let mut tm = *e.clone();
        for _ in 0..indent+1 {
            print!("    ");
        }
        match tm {
            E::Null => {
                print!("default ");
            },
            _ => {
                DeconpAST(false, e,cnfg, indent + 1);
            },
        }
        print!(" :");
        let n = ef.len();
        if n > 1 {print!("begin \n")}
        for y in ef {
            if n > 1 {
                DeconpAST(false, y,cnfg, indent + 2);
            }
            else {
                DeconpAST(false, y,cnfg, 0);
            }
        }
        if n > 1 {
            for _ in 0..indent+1 {
                print!("    ");
            }
            print!("end \n")
        }
    }
    for _ in 0..indent {
        print!("    ");
    }
    println!("endcase");
    return true;
}

/// Fsm構文出力関数--always文を生成する
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintFsm(Fsm: FsmModule) -> bool {
    let tmp = Fsm.clone();
    let CLK = tmp.clone().StateClk();
    let RST = tmp.clone().StateRst();
    let Reg = tmp.clone().StateReg();
    let p = tmp.clone().StateOut();
    println!("    always@(posedge {} or posedge {}) begin", _StrOut(CLK.clone()), _StrOut(RST.clone()));
    println!("        if ({} == 1) begin \n            {} <= {}; \n        end",_StrOut(RST.clone()), _StrOut(Reg.clone()), _StrOut(tmp.clone().FirstState()));
    println!("        else begin \n            {} <= {}_Next; \n        end \n    end \n",_StrOut(Reg.clone()),_StrOut(Reg.clone()));
    println!("    always@(posedge {}) begin",_StrOut(CLK.clone()));
    println!("        if ({}) {}_Next <= {};\n", _StrOut(RST.clone()), _StrOut(Reg.clone()), _StrOut(tmp.clone().FirstState()));
    println!("        else begin\n");
    println!("            case({})",_StrOut(Reg.clone()));
    for st in p {
        PrintState(st.clone());
    }
    println!("            endcase \n        end\n    end\n");



    return true;
}

/// 1Stateモデル出力関数
#[allow(dead_code)]
#[allow(non_snake_case)]
fn PrintState(STMT: StateModule) -> bool {
    let mut st = STMT;
    let stname = st.getStateName();
    let tmp = st.getBranch();

    println!("                {} : begin",stname);
    PrintIf(tmp.clone(), "Non", 4);
    println!("                end");
    return true;
}

/// NONAST
#[macro_export]
macro_rules! Blank {
    () => (Box::new(E::Null))
}

/// FSM生成関数
#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn Clock_Reset(in_clk: Box<E>, in_rst: Box<E>) -> FsmModule {
    let p = wrVar::new().Reg("state", 32);
    FsmModule{clk: in_clk, rst: in_rst, fsm: p, State: Vec::new(), Current_state: 0}
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
    pub fn goto(mut self, State_name: &str, Branch: Box<E>) -> FsmModule{
        
        let SelfS = self.fsm.clone();
        let mut st = "".to_string();
        if let E::Ldc(wr) = *SelfS.clone() { st = wr.getName().clone() };
        st = st + "_Next";
        let NState = wrVar::new().Reg(&st,0);
        let Goto_ = wrVar::new().Parameter(State_name,0);
        self.State[(self.Current_state as usize)].SetBranch(Branch.clone(), F!(NState = Goto_));

        self.clone()
    }

    // 指定ステートからカレントステートへの定義(指定ステートの作成後以降に使用可)
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn from(mut self, State_name: &str, Branch: Box<E>) -> FsmModule {
        let SelfS = self.fsm.clone();
        let mut st = "".to_string();
        if let E::Ldc(wr) = *SelfS.clone() { st = wr.getName().clone() };
        st = st + "_Next";
        let NState = wrVar::new().Reg(&st,0);
        let NameCurrentState = self.State[((self.Current_state-1) as usize)].getStateName();
        for x in &mut self.State {
            let Nx = x.getStateName();
            if Nx == State_name.to_string() {
                let Goto_ = wrVar::new().Parameter(&NameCurrentState,0);
                x.SetBranch(Branch.clone(), F!(NState = Goto_));
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
    fn SetBranch(&mut self, Terms: Box<E>, Form: Box<E>) -> bool {
        let e = *(Terms.clone());
        let mut tmp = Vec::new();
        tmp.push(Form);
        
        match e {
            E::Null => self.Branch.push(IfStmt_AST{If_: true, IfE: Terms, ST: tmp}),
            _ => self.Branch.push(IfStmt_AST{If_: true, IfE: Terms, ST: tmp}),
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

// 基本Box<E>の分解に使用

/// AST分解メソッド
#[allow(non_snake_case)]
pub fn _Decomp(e: Box<E>, Sel : &str) -> Box<E>{
    let m = *e;
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
pub fn _StrOut(e: Box<E>) -> String {
    let m = *e;
    match m {
        E::Ldc(WR) => WR.getName(),
        E::Bin(ref Op, _, _) => Op.clone(),
        _ => "Null".to_string(),
    }
}

/// AST数値抽出メソッド
#[allow(non_snake_case)]
pub fn _NumOut(e: Box<E>) -> i32 {
    let m = *e;
    match m {
        E::Ldc(WR) => WR.getWidth(),
        E::Num(i) => i,
        _ => 0,
    }
}
