use gloo_net::http::Request;


pub async fn fetch<Fn>(path: &str, fun: Fn)
where
    Fn: FnOnce(String),
{
    if let Ok(response) = Request::get(path).send().await {
        if !response.ok() {
    
        } else {
            let response = response.text().await.expect("Decode");
            fun(response)
        }
    }
}

pub async fn post<Fn, T>(path: &str, data: T, fun: Fn) 
where
    Fn: FnOnce(String) ,
    T: serde::Serialize,
{
    if let Ok(request) = Request::post(path).json(&data) {
        if let Ok(response) = request.send().await {
            if !response.ok() {
             
            } else {
                let response = response.text().await.expect("Decode");
                fun(response)
            }
        }
    } 
}
