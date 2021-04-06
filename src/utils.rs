#[allow(dead_code)]

pub struct Coffee {
    pub name: &'static str,
    pub price: &'static str,
}
impl Coffee {
    pub fn from(name: &'static str, price: &'static str) -> Self {
        Coffee { name, price }
    }
}
// list the coffee menu from

pub fn coffee() -> Vec<Coffee> {
    vec![
        Coffee::from("Expresso", "£3.99"),
        Coffee::from("Cappuchino", "£3.10"),
        Coffee::from("Latte", "£2.75"),
        Coffee::from("Americano", "£2.00"),
        Coffee::from("Mocha", "£2.50"),
    ]
}
// Sugar level enum
// Sugar Levels
#[allow(dead_code)]
pub struct Sugar {
    pub name: &'static str,
    pub spoons: u8,
}

#[allow(dead_code)]
pub fn sugar() -> Vec<Sugar> {
    vec![
        Sugar {
            name: "low",
            spoons: 1,
        },
        Sugar {
            name: "Medium",
            spoons: 2,
        },
        Sugar {
            name: "High",
            spoons: 3,
        },
    ]
}
