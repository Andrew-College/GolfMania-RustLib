pub struct Physics {
    _angle: f32,
    _initial_power: f32,
    _drag: f32,
}

pub struct PhysicsArgs {
    _angle: Option<f32>,
    _initial_power: Option<f32>,
    _drag: Option<f32>,
}

impl Physics {
    fn empty() -> Physics {
        Physics{
            _angle: 0f32,
            _drag: 0f32,
            _initial_power: 0f32,
        }
    }

    pub fn fire(args: Option<PhysicsArgs>) -> Physics {
        match args {
            Some(args) => Physics{
                _angle: match args._angle {
                    Some(angle) => angle,
                    None => 0f32,
                },
                _drag: match args._drag {
                    Some(drag) => drag,
                    None => 0f32,
                },
                _initial_power: match args._initial_power {
                    Some(power) => power,
                    None => 0f32,
                }
            },
            None => Physics::empty(),
        }
    }
}