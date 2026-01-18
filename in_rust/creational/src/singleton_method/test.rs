


#[cfg(test)]
mod singleton_tests{
    use crate::singleton_method::singleton::get_singleton_instance;
    use tracing::{Level,info}; 
    use tracing_subscriber::fmt;

    fn setup_logging() {
        let _ = fmt()
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .try_init();
    }

    #[test]
    fn singleton_information_test(){
        setup_logging();
        let singleton1 = get_singleton_instance();
        let singleton2 = get_singleton_instance();
        assert!(std::ptr::eq(singleton1, singleton2)); // Verify both references point to the same instance
        assert!(singleton1.get_program_name() == singleton2.get_program_name());
        assert!(singleton1.get_version() == singleton2.get_version());


        info!("Program Name: {}", singleton1.get_program_name());
        info!("Version: {}", singleton1.get_version());
    }


}