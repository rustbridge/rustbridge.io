extern crate rocket;
// extern crate rocket_codegen;


fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", rocket_codegen::routes![main::index])
}

#[cfg(test)]
mod test {
   
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn home_page(){
        let client = Client::new(rocket() ).unwrap();
        let mut response = client.get("/").dispatch();
        println!("dis is my test {:#?}", response);
        assert_eq!(response.status(), Status::Ok);
        
        // assert_eq!(response, data::)
   }

}

