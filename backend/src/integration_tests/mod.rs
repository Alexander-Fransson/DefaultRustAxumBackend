#[cfg(test)]
pub mod test {

    use serial_test::serial;

    use crate::config_env::get_env_variables;
    //use crate::log::tracer_config::enable_tracing;
    use crate::serve_server;
    use crate::error::Result;
    use crate::views::user::{User, UserForLogin, UserForRegister};

    //make tests sequential


    fn run_server_sub_process() -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            //enable_tracing();
            if let Err(e) = serve_server().await {
                println!("error: {}", e);
            }
        })
    }


    #[serial]
    #[tokio::test]
    async fn main_response_mapper_ok() -> Result<()> {
        let server = run_server_sub_process();
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        // failed login

        let client = reqwest::Client::new();

        let request_url = format!("http://{}/auth/login", get_env_variables().LISTENER_URL);

        let login_params = UserForLogin {
            email: "test@email.com".to_string(),
            password: "wrong password".to_string()
        };

        let bad_login_body = serde_json::to_string(&login_params).unwrap();

        let response = client
        .post(request_url)
        .header("Content-Type", "application/json") 
        .body(bad_login_body)
        .send().await.unwrap()
        ;

        let text_response = response.text().await.unwrap();

        print!("text_response: {}",text_response);

        assert!(&text_response.contains("ENTITY_NOT_FOUND"));

        server.abort();

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn router_hello_word_ok() -> Result<()> {
        
        let server = run_server_sub_process();
        
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let client = reqwest::Client::new();

        let request_url = format!("http://{}/hello_word", get_env_variables().LISTENER_URL);

        let response = client.get(request_url).send().await.unwrap();

        println!("response: {:#?}", response);

        assert_eq!(response.status(), reqwest::StatusCode::OK);

        server.abort();

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn  router_create_read_delete_ok() -> Result<()> {

        let server = run_server_sub_process();

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let client = reqwest::Client::new();

        let request_url = format!("http://{}/api/v1/user/", get_env_variables().LISTENER_URL);

        let user = UserForRegister {
            name: "test_username".to_string(),
            email: "test_email".to_string(),
            password: "test_password".to_string(),
        };

        let response = client.post(&request_url)
        .json(&user) // needed reqwest -F json
        .send().await
        .unwrap();

        assert_eq!(response.status(), reqwest::StatusCode::OK);

        let user_id = response.json::<User>().await.unwrap().id;

        let response = client.get(format!("{}{}", &request_url, user_id)).send().await.unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::OK);

        client.delete(format!("{}{}", &request_url, user_id)).send().await.unwrap();

        server.abort();

        Ok(())
    }
}