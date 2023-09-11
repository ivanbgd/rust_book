use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(_) => {
                self.update_average();
                result
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn sort(&mut self) {
        self.list.sort_unstable();
    }

    fn update_average(&mut self) {
        let total = self.list.iter().sum::<i32>();
        self.average = total as f64 / self.list.len() as f64;
    }
}

impl Display for AveragedCollection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AveragedCollection {{\n    list: {:?},\n    average: {},\n}}", self.list, self.average)
    }
}
