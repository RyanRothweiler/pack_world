// !! really shouldn't be putting api keys here man

pub struct ServerEnvironment {
    pub supabase_url: &'static str,
    pub supabase_api_key: &'static str,
}

// prod env
#[cfg(feature = "server_prod")]
pub const SERVER_ENV: ServerEnvironment = ServerEnvironment {
    supabase_url: "todo",
    supabase_api_key: "todo",
};

// default dev env
#[cfg(not(feature = "server_prod"))]
pub const SERVER_ENV: ServerEnvironment = ServerEnvironment {
    supabase_url: "https://qqibqjlgvkhzyrjaabvg.supabase.co",
    supabase_api_key: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InFxaWJxamxndmtoenlyamFhYnZnIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDIzMTc1MTUsImV4cCI6MjA1Nzg5MzUxNX0.wYCDHY5jXVIex2E6ZmzU16DQC5GtqMiPV974N7TQKUM",
};
