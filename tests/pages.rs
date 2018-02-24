extern crate rocket;

#[cfg(test)]
mod test {
   
    //use data::*;
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn home_page(){
        let client = Client::new(rocket::ignite()).unwrap();
        let mut response = client.get("/").dispatch();
        println!("dis is my test {:#?}", response);
        assert_eq!(response.status(), Status::Ok);
        
        // assert_eq!(response, data::)
   }

}

