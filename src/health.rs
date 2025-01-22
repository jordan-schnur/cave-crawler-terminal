#[derive(Clone, Copy, Debug)]
pub struct Health {
    current: i32,
    max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Health { current: max, max }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.current -= damage;
        if self.current < 0 {
            self.current = 0;
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }

    pub fn get_current(&self) -> i32 {
        self.current
    }

    pub fn get_max(&self) -> i32 {
        self.max
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0
    }
}

impl std::fmt::Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.current, self.max)
    }
}
