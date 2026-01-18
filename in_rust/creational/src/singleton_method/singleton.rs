#![allow(dead_code)]

use std::sync::OnceLock; 

pub struct UniqueInformation{ 
    program_name: String,
    version: String,
}
impl UniqueInformation{
    fn new(program_name: &str, version: &str) -> Self{
        UniqueInformation{
            program_name: program_name.to_string(),
            version: version.to_string(),
        }
    }

    pub fn get_program_name(&self) -> &str{
        &self.program_name
    }

    pub fn get_version(&self) -> &str{
        &self.version
    }
}

static SINGLETON_INSTANCE: OnceLock<UniqueInformation> = OnceLock::new();


pub fn get_singleton_instance() -> &'static UniqueInformation{
    // NOTA: in this example, we are using a simple static initialization.
    // in production scenarios you can load this information from environment variables
    SINGLETON_INSTANCE.get_or_init(||{
        UniqueInformation::new("Design Patterns in Rust", "1.0.0")
    })
}
