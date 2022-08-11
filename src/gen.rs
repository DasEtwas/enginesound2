use std::ops::Mul;

// m^3*Pa/(mol*K) = J/(mol*K)
pub const GAS_CONSTANT: f32 = 8.31446261815324;

/// converts bar to Pa
pub const BAR: f32 = 100000.0;
/// converts cubic centimeters to m^3
pub const CCM: f32 = 1e-6;
/// converts L to m^3
pub const LITER: f32 = 1e-3;
/// converts ml to m^3
pub const MILLILITER: f32 = CCM;
pub const CELSIUS: f32 = 273.15;

#[derive(Copy, Clone, Debug)]
pub struct Atmosphere {
    /// Pa
    pub ambient_pressure: f32,
    /// K
    pub ambient_temperature: f32,
    /// in mol/m^3
    pub gas: GasMix,
}

pub struct Fuel {}

pub struct Generator {
    pub engine: EngineState,
    /// Hz
    pub rate: f32,
    /// Hz
    pub sample_rate: u32,

    pub atmosphere: Atmosphere,
}

impl Generator {
    pub fn step(&mut self) {
        self.engine
            .step(&self.atmosphere, self.rate.recip(), self.rate);
    }
}

pub struct EngineState {
    pub cylinders: Vec<Cylinder>,
    /// rad/s
    pub speed: f32,
    /// rad
    pub position: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct GasMix {
    /// mol (nitrogen, argon, combustion products (water, carbon dioxide))
    pub neutral: f32,
    /// mol (atoms of oxygen)
    pub oxidizer: f32,
    /// mol (hexane, octane, ..)
    pub fuel: f32,
}

// https://www.ohio.edu/mechanical/thermo/property_tables/air/air_Cp_Cv.html
impl GasMix {
    /// J/kg*K
    pub fn specific_heat_capacity_constant_volume(&self) -> f32 {
        0.718
    }

    /// J/mol*K
    pub fn specific_heat_capacity_constant_pressure(&self) -> f32 {
        1.005
    }
}

impl GasMix {
    /// mol
    pub fn amount(&self) -> f32 {
        self.neutral + self.oxidizer + self.fuel
    }
}

impl Mul<f32> for GasMix {
    type Output = GasMix;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            fuel: rhs * self.fuel,
            neutral: rhs * self.neutral,
            oxidizer: rhs * self.oxidizer,
        }
    }
}

// PV = nRT
#[derive(Copy, Clone)]
pub struct Pipe {
    /// m^3
    pub pipe_volume: f32,

    /// K
    pub temperature: f32,
    pub gas: GasMix,
    /// Pa
    pub pressure: f32,
}

#[derive(Copy, Clone)]
pub struct Cylinder {
    pub cylinder: Pipe,
    /// rad, position offset from engine crankshaft
    pub phase: f32,
    /// m^2
    pub face_area: f32,
    /// m
    pub cylinder_height: f32,
    /// m
    pub crank_radius: f32,
    /// m
    pub connecting_rod_length: f32,

    /// 1 = open, 0 = closed
    pub intake_valve: f32,
    /// 1 = open, 0 = closed
    pub exhaust_valve: f32,
    /// instantaneous temperature of spark
    pub spark_temp: f32,
}

impl Cylinder {
    pub fn new(
        engine_position: f32,
        phase: f32,
        compression_ratio: f32,
        connecting_rod_length: f32,
        displacement: f32,
        radius: f32,
        atm: &Atmosphere,
    ) -> Self {
        assert!(compression_ratio > 1.0);

        let clearance_volume = displacement / (compression_ratio - 1.0);
        let chamber_volume = displacement + clearance_volume;

        let face_area = std::f32::consts::PI * radius.powi(2);

        let crank_radius = (displacement / face_area) * 0.5;

        let cylinder_height = crank_radius + connecting_rod_length + clearance_volume / face_area;

        let mut ret = Self {
            cylinder: Pipe {
                pipe_volume: displacement,
                temperature: atm.ambient_temperature,
                gas: atm.gas * chamber_volume,
                pressure: atm.ambient_pressure,
            },
            phase,
            face_area,
            cylinder_height,
            crank_radius,
            connecting_rod_length,
            intake_valve: 0.0,
            exhaust_valve: 0.0,
            spark_temp: 0.0,
        };

        ret.cylinder.pipe_volume = ret.volume(engine_position + ret.phase);
        ret
    }

    pub fn volume(&self, position: f32) -> f32 {
        (self.cylinder_height
            - (self.crank_radius * position.cos()
                + f32::sqrt(
                    self.connecting_rod_length.powi(2)
                        - (self.crank_radius * position.sin()).powi(2),
                )))
            * self.face_area
    }

    pub fn step(&mut self, position: f32, atm: &Atmosphere, dt: f32, inv_dt: f32) {
        let last_volume = self.cylinder.pipe_volume;
        let volume = self.volume(position + self.phase);

        let last_pressure = self.cylinder.pressure;

        let adiabatic_ratio = self.cylinder.gas.specific_heat_capacity_constant_pressure()
            / self.cylinder.gas.specific_heat_capacity_constant_volume();

        let j = (last_volume / volume).powf(adiabatic_ratio);
        //  dbg!(j);
        let pressure = last_pressure * j;

        let last_temperature = self.cylinder.temperature;
        let h = (pressure / last_pressure).powf((adiabatic_ratio - 1.0) / adiabatic_ratio);
        let temperature = last_temperature * h;

        // dbg!(temperature);

        self.cylinder.pipe_volume = volume;
        self.cylinder.pressure = pressure;
        self.cylinder.temperature = temperature;
    }
}

impl EngineState {
    pub fn step(&mut self, atm: &Atmosphere, dt: f32, inv_dt: f32) {
        self.position += self.speed * dt;

        // PV = nRT

        for cyl in &mut self.cylinders {
            cyl.step(self.position, atm, dt, inv_dt);
        }
    }
}
