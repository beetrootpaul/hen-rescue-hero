pub enum RobotDirection {
    LeftStaying,
    LeftMoving,
    RightStaying,
    RightMoving,
}

impl RobotDirection {
    pub fn is_right(&self) -> bool {
        match *self {
            RobotDirection::LeftStaying => false,
            RobotDirection::LeftMoving => false,
            RobotDirection::RightStaying => true,
            RobotDirection::RightMoving => true,
        }
    }
    pub fn to_staying(&self) -> RobotDirection {
        match *self {
            RobotDirection::LeftStaying => RobotDirection::LeftStaying,
            RobotDirection::LeftMoving => RobotDirection::LeftStaying,
            RobotDirection::RightStaying => RobotDirection::RightStaying,
            RobotDirection::RightMoving => RobotDirection::RightStaying,
        }
    }
}
