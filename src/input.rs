use std::error::Error;

pub struct Input <'a> {
    arguments: Option<Vec<&'a str>>,
    flags_general: Option<Vec<&'a str>>,
    flags_targeting_project: Option<Vec<&'a str>>,
    flags_targeting_production: Option<Vec<&'a str>>,
    config_project_root: Option<&'a str>,
    config_production_roots: Option<Vec<&'a str>>,
    config_active_project: Option<&'a str>,
    config_company_name: Option<&'a str>
}

impl<'a> Input <'a>{
    pub fn get() -> Input<'a>{
        let mut Input = Input{
            arguments: None,
            flags_general: None,
            flags_targeting_project: None,
            flags_targeting_production: None,
            config_project_root: None,
            config_production_roots: None,
            config_active_project: None,
            config_company_name: None
        }; 

        Input.parse_args();

        Input
    }
}
impl<'a> Input<'a> {
    fn parse_args(&mut self) -> Option<Vec<&str>> {

        None
    }
    fn parse_flags(&mut self) {}
    fn iparse_config(&mut self) {}
    fn ask_config() {}
    fn write_config() {}

}


