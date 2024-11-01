use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ControldBody {
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ControldStatus {
    body: ControldBody,
    success: bool,
}

const CONTROLD_URL: &str = "https://comss.verify.controld.com/utility/reload-config";

async fn get_controld_status() -> Result<Response, reqwest::Error> {
    reqwest::get(CONTROLD_URL).await
}

async fn unwrap_controld_status(body: Result<Response, reqwest::Error>) -> ControldStatus {
    match body {
        Ok(r) => {
            if r.status().is_success() {
                r.json::<ControldStatus>().await.unwrap()
            } else {
                ControldStatus {
                    body: ControldBody {
                        status: "dns error".to_owned(),
                    },
                    success: false,
                }
            }
        }
        _ => ControldStatus {
            body: ControldBody {
                status: "dns error".to_owned(),
            },
            success: false,
        },
    }
}

fn is_controld_up(controld_status: ControldStatus) -> bool {
    controld_status.success
}

pub async fn get_status_of_controld() -> bool {
    is_controld_up(unwrap_controld_status(get_controld_status().await).await)
}
