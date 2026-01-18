


#[cfg(test)]
mod prototype_tests{
    use crate::prototype_method::prototype::{Permissions,UserPrototype};
    use tracing::{Level,info}; 
    use tracing_subscriber::fmt; 


    fn setup_logging() {
        let _ = fmt()
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .try_init();
    }

    #[test]
    fn prototyping_user_test(){
        setup_logging();
        let admin_user = UserPrototype::new(
           "manuel",
           "manuel@gmail.com",
            "admin",
            vec![Permissions::ALL] 
        );


        let admin_user2 = admin_user.cloner("alfonso", "alfonso@gmail.com");

        assert!(admin_user.get_username() != admin_user2.get_username());
        assert!(admin_user.get_id() != admin_user2.get_id());
        assert!(admin_user.get_email() != admin_user2.get_email());
        assert!(admin_user.get_role() == admin_user2.get_role());
        assert!(admin_user.get_permissions() == admin_user2.get_permissions());
        info!("Original User: {:?}", admin_user);
        info!("Cloned User: {:?}", admin_user2);


    }



}