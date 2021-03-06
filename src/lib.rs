use anyhow::{anyhow, Result};
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};
use path_utils::convert_filename_extension_to_lowercase;
use uuid::Uuid;

pub async fn receive_multipart_file(mut body: actix_multipart::Multipart) -> Result<String> {
    let mut filepath_dest = String::new();
    // recibir el archivo
    // iterate over multipart stream
    while let Ok(Some(mut field)) = body.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| anyhow!("Error receiving file"))?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| anyhow!("Error receiving file"))?;
        let filepath_uuid = format!(
            "./{}{}",
            Uuid::new_v4(),
            sanitize_filename::sanitize(&filename)
        );

        let filepath = convert_filename_extension_to_lowercase(&filepath_uuid)
            .ok_or_else(|| anyhow!("Filename error"))?;
        filepath_dest = filepath.clone();
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(filepath_dest)
}

#[cfg(feature = "enablereqwest")]
pub async fn pass_post_to_server(
    url: &str,
    req: &impl serde::Serialize,
) -> impl actix_web::Responder {
    use actix_web::{http::StatusCode, Responder};

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
