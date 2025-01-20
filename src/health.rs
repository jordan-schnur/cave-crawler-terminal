#[derive(Clone, Copy, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Health {
            current: max,
            max,
        }
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

    pub fn is_alive(&self) -> bool {
        self.current > 0
    }
}