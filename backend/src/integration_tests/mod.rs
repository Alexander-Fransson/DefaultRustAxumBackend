#[cfg(test)]
pub mod test {

    use crate::config_env::get_env_variables;
    use crate::log::tracer_config::enable_tracing;
    use crate::serve_server;
    use crate::error::Result;
    use crate::data_shapes::user::{UserForRegister, User};

    //make tests sequential

    fn run_server_sub_process() -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            enable_tracing();
            if let Err(e) = serve_server().await {
                println!("error: {}", e);
            }
        })
    }

    #[tokio::test]
    #[ignore]
    async fn router_hello_word_ok() -> Result<()> {
        
        let server = run_server_sub_process();
        
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let client = reqwest::Client::new();

        let request_url = format!("http://{}/hello_word", get_env_variables().LISTENER_URL);

        let response = client.get(request_url).send().await.unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::OK);

        server.abort();

        Ok(())
    }

    #[tokio::test]
    async fn  router_create_read_delete_ok() -> Result<()> {

        let server = run_server_sub_process();

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let client = reqwest::Client::new();

        let request_url = format!("http://{}/api/v1/user", get_env_variables().LISTENER_URL);

        let user = UserForRegister {
            username: "test_username".to_string(),
            email: "test_email".to_string(),
            password: "test_password".to_string(),
        };

        let response = client.post(&request_url)
        .json(&user) // needed reqwest -F json
        .send().await
        .unwrap();

        println!("response: {:#?}", &response.text().await.unwrap());

        assert!(false);

        // assert_eq!(response.status(), reqwest::StatusCode::OK);

        // let user_id = response.json::<User>().await.unwrap().id;

        // let response = client.get(format!("{}/{}", &request_url, user_id)).send().await.unwrap();
        // assert_eq!(response.status(), reqwest::StatusCode::OK);

        // client.delete(format!("{}/{}", &request_url, user_id)).send().await.unwrap();

        // server.abort();

        Ok(())
    }
}