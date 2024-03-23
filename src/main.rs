#[tokio::main]
async fn main() {
    println!("It Works! (on my machine)");
}

#[tokio::test]
async fn figma_root_11() {
    let res = reqwest_11::get("https://figma.com/").await.unwrap();
    assert_eq!(res.status(), 200, "Expected 200 Success: {res:#?}");
}

#[tokio::test]
async fn figma_root_12() {
    let res = reqwest_12::get("https://figma.com/").await.unwrap();
    assert_eq!(res.status(), 200, "Expected 200 Success: {res:#?}");
}

#[tokio::test]
async fn figma_www_root_11() {
    let res = reqwest_11::get("https://www.figma.com/").await.unwrap();
    assert_eq!(res.status(), 200, "Expected 200 Success: {res:#?}");
}

#[tokio::test]
async fn figma_www_root_12() {
    let res = reqwest_12::get("https://www.figma.com/").await.unwrap();
    assert_eq!(res.status(), 200, "Expected 200 Success: {res:#?}");
}

#[tokio::test]
async fn figma_api_root_11() {
    let res = reqwest_11::get("https://api.figma.com/").await.unwrap();
    assert_eq!(res.status(), 200, "Expected 200 Success: {res:#?}");
}

#[tokio::test]
async fn figma_api_root_12() {
    let res = reqwest_12::get("https://api.figma.com/").await.unwrap();
    assert_eq!(res.status(), 200, "Expected 200 Success: {res:#?}");
}

#[tokio::test]
async fn figma_api_me_11() {
    let res = reqwest_11::get("https://api.figma.com/v1/me")
        .await
        .unwrap();
    assert_eq!(res.status(), 403, "Expected 403 Forbidden: {res:#?}");
}

#[tokio::test]
async fn figma_api_me_12() {
    let res = reqwest_12::get("https://api.figma.com/v1/me")
        .await
        .unwrap();
    assert_eq!(res.status(), 403, "Expected 403 Forbidden: {res:#?}");
}


#[tokio::test]
async fn figma_api_root_http1_12() {
    let client = reqwest_12::Client::builder().http1_only().build().unwrap();
    let res = client.get("https://api.figma.com/")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200, "Expected 403 Forbidden: {res:#?}");
}


#[tokio::test]
async fn figma_api_root_http1_11() {
    let client = reqwest_11::Client::builder().http1_only().build().unwrap();
    let res = client.get("https://api.figma.com/")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200, "Expected 403 Forbidden: {res:#?}");
}
