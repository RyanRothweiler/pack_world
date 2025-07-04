// todo this should go into the build_vars module
// !! really shouldn't be putting api keys here man

pub struct ServerEnvironment {
    pub display_name: &'static str,
    pub supabase_url: &'static str,
    pub supabase_api_key: &'static str,
}

pub fn server_env() -> &'static ServerEnvironment {
    if let Some(env) = option_env!("SERVER_ENV") {
        if env == "prod" {
            return &SERVER_ENV_PROD;
        }
    }

    // default to dev
    return &SERVER_ENV_DEV;
}

pub const SERVER_ENV_PROD: ServerEnvironment = ServerEnvironment {
    display_name: "prod",
    supabase_url: "https://xmqsqfvasqrvqnuqyrdr.supabase.co",
    supabase_api_key: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InhtcXNxZnZhc3FydnFudXF5cmRyIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NTE1MTU1MzgsImV4cCI6MjA2NzA5MTUzOH0.eVuWenl7HF9wd89v7lvk3vh0sgEXxkOK_DQUD9IN7ck",
};

pub const SERVER_ENV_DEV: ServerEnvironment = ServerEnvironment {
    display_name: "dev",
    supabase_url: "https://qqibqjlgvkhzyrjaabvg.supabase.co",
    supabase_api_key: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InFxaWJxamxndmtoenlyamFhYnZnIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDIzMTc1MTUsImV4cCI6MjA1Nzg5MzUxNX0.wYCDHY5jXVIex2E6ZmzU16DQC5GtqMiPV974N7TQKUM",
};
