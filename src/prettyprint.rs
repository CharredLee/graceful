use polynomial::Polynomial;

pub trait PrettyPrint {
    fn ascii(&self) -> String;

    fn pretty_print(&self) {
        println!("{}", self.ascii());
    }
}

impl PrettyPrint for u64 {
    fn ascii(&self) -> String {
        format!("{}", self)
    }
}

impl PrettyPrint for Polynomial<i64> {
    fn ascii(&self) -> String{
        let mut out = String::new();
        if self.data().len() == 0 {
            out.push_str("0");
            return out;
        } else if self.data().len() == 1 {
            out.push_str(&format!("{}", self.data()[0]));
            return out;
        } else {
            let constant = self.data()[0];
            let linear = self.data()[1];
            if linear != 1 {
                out.push_str(&format!("{}n", linear));
            } else {
                out.push_str("n");
            }
            if constant != 0 {
                if constant > 0 {
                    out.push_str(&format!("+{}", constant));
                } else {
                    out.push_str(&format!("{}", constant));
                }
            }
            return out;
        }
    }
}