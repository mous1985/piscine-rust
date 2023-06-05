pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}
pub fn fetch_data(server: Result<String, String>, security: Security) -> String {
    match security{
        Security::Unknown => server.unwrap(),  
        Security::High => server.expect("ERROR: program stops"),  
        Security::Medium => server.unwrap_or_else(|_| String::from("WARNING: check the server")), 
        Security::Low => server.unwrap_or_else(|err| format!("Not found: {}", err)),  
        Security::BlockServer => server.expect_err(""),  
    }
}