use actix_web::{http::StatusCode, Responder};

pub async fn pass_post_to_server(url: &str, req: &impl serde::Serialize) -> impl Responder {
    let client = reqwest::Client::new();
    let res = client.post(url).json(req).send().await;

    match res {
        Ok(response) => {
            let status = response.status();
            response.text().await.unwrap().with_status(status)
        }
        _ => "{\"status\": \"Internal Error\"}"
            .to_string()
            .with_status(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
