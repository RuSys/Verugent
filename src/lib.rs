#[cfg(test)]
mod tests {
    use vcore::*;
    #[test]
    fn it_works() {
        let mut m = VModule::new("LED");
        m.Input("CLK", 1);
        m.Input("RST", 1);
        assert!(!m.endmodule().is_empty(), "Code not generated successfully...");
    }
}

pub mod vcore;
