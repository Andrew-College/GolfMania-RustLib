pub struct Physics {
    _angle: f32,
    _initial_power: f32,
    _drag: f32,
}

#[derive(Copy, Clone)]
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

impl PhysicsArgs {
    pub fn new() -> PhysicsArgs {
        PhysicsArgs{
            _angle: None,
            _drag: None,
            _initial_power: None,
        }
    }

    pub fn angle(&mut self, newAngle : f32) {
        self._angle = Some(newAngle)
    }
    
    pub fn drag(&mut self, newDrag : f32) {
        self._drag = Some(newDrag)
    }

    pub fn power(&mut self, newPower : f32) {
        self._initial_power = Some(newPower)
    }
}

#[cfg(test)]
mod tests {
    use super::{Physics, PhysicsArgs};

    #[test]
    fn create_physics() {
        let test = Physics::empty();
        
        assert_eq!(test._angle, 0f32);
        assert_eq!(test._drag, 0f32);
        assert_eq!(test._initial_power, 0f32);
    }

    #[test]
    fn create_physics_from_args() {
        let mut _args = PhysicsArgs::new();

        _args.angle(15f32);

        assert_eq!(_args._angle, Some(15f32));

        let test = Physics::fire(Some(_args));

        assert_eq!(test._angle, 15f32);
    }
}