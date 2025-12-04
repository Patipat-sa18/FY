#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub timeout: u64,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub server: Server,
    pub database: Database,
    pub secret: String,
    // pub max_crew_per_mission: u32,
}
