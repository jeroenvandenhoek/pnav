struct Input {
    arguments: Vec<&str>,
    flags_general: Vec<&str>,
    flags_targeting_project: Vec<&str>,
    flags_targeting_production: Vec<&str>,
    config_project_root: &str,
    config_production_roots: Vec<&stra>,
    config_active_project: &str,
    config_company_name: &str
}

impl Input {
    fn parse_args(&mut str) {}
}
