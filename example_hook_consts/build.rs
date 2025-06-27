use build_timing::BuildTimingBuilder;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct MyConst(&'static str);

const OKK: MyConst = MyConst("OKK");
const NONONO: MyConst = MyConst("NONONO");

impl build_timing::BuildConstVal for MyConst {
    fn build_val(&self) -> build_timing::ConstVal {
        match *self {
            OKK => build_timing::ConstVal {
                desc: "OKK".to_string(),
                v: "OKK123".to_string(),
                t: build_timing::ConstType::Str,
            },
            NONONO => build_timing::ConstVal {
                desc: "NONONO".to_string(),
                v: "NONONO456".to_string(),
                t: build_timing::ConstType::Str,
            },
            _ => panic!("Unknown build constant: {}", self.to_string()),
        }
    }
}

impl ToString for MyConst {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

fn main() {
    BuildTimingBuilder::builder()
        .add_const_hook(Box::new(OKK))
        .add_const_hook(Box::new(NONONO))
        .add_const_hook(Box::new(build_timing::BUILD_OS))
        .build()
        .unwrap();
}
