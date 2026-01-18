#![allow(dead_code)]


use uuid::Uuid; 


#[derive(Debug, Clone, PartialEq)]
pub enum Permissions{
    READ, 
    WRITE,
    COPY,
    DELETE,
    ALL,
}

#[derive(Debug, Clone)]
pub struct UserPrototype{
    id: Uuid,
    username: String, 
    email: String, 
    role : String, 
    permissions : Vec<Permissions>
}


impl UserPrototype {
    pub fn new(username: impl Into<String>, email: impl Into<String>, role: impl Into<String>, permissions: Vec<Permissions>)-> Self{
        let id = Uuid::new_v4(); 
        Self{
            id, 
            username:username.into(), 
            email:email.into(), 
            role:role.into(),
            permissions
        }

    }

    pub fn cloner(&self, username: impl Into<String>, email: impl Into<String>) -> Self{

        let mut  clon = self.clone(); 
        clon.id = Uuid::new_v4(); 
        clon.username = username.into(); 
        clon.email = email.into(); 
        clon 
    }


    pub fn get_id(&self) -> Uuid{
        self.id
    }
    pub fn get_username(&self) -> &str{
        &self.username
    }
    pub fn get_email(&self) -> &str{
        &self.email
    }
    pub fn get_role(&self) -> &str{
        &self.role
    }
    pub fn get_permissions(&self) -> &Vec<Permissions>{
        &self.permissions
    }


}
