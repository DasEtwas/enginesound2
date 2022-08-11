/// J/(mol*K)
pub const GAS_CONSTANT: f32 = 8.3145;

pub struct Atmosphere {
    /// Pa
    pub ambient_pressure: f32,
    /// K
    pub ambient_temperature: f32,
}

pub struct Generator {
    pub engine: EngineState,
    /// Hz
    pub rate: f32,
    /// Hz
    pub sample_rate: u32,
}

pub struct EngineState {
    pub cylinders: Vec<Cylinder>,
    /// rad/s
    pub speed: f32,
    /// rad
    pub position: f32,
}

// PV = nRT
pub struct Pipe {
    /// m^3
    pub pipe_volume: f32,

    /// K
    pub temperature: f32,
    /// mol (nitrogen, argon, combustion products, ..)
    pub neutral_amount: f32,
    /// mol (oxygen)
    pub oxidizer_amount: f32,
    /// mol (hexane, octane, ..)
    pub fuel_amount: f32,
}

pub struct Cylinder {
    pub cylinder: Pipe,
    /// rad
    pub phase: f32,
    /// m^3
    pub min_volume: f32,
    /// m^3
    pub max_volume: f32,
    /// 1 = open, 0 = closed
    pub intake_valve: f32,
    /// 1 = open, 0 = closed
    pub exhaust_valve: f32,
    /// instantaneous temperature of spark
    pub spark_temp: f32,
}

impl Cylinder {}

impl EngineState {
    pub fn step(&mut self, atm: &Atmosphere, dt: f32, inv_dt: f32) {
        self.position += self.speed * dt;
    }
}
