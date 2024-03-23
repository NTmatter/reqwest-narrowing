#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let res = reqwest_11::get("https://api.figma.com/").await?;
    let status = res.status();

    if status == 200 {
        println!("Success!");
    } else {
        eprintln!("Failed to retrieve data: {status}");
    }

    Ok(())
}

#[tokio::test]
async fn figma_root_11() {
    let res = reqwest_11::get("https://figma.com/").await.unwrap();
    assert_eq!(res.status(), 200, "Response failed: {res:#?}");
}

#[tokio::test]
async fn figma_root_12() {
    let res = reqwest_12::get("https://figma.com/").await.unwrap();
    assert_eq!(res.status(), 200, "Response failed: {res:#?}");
}

#[tokio::test]
async fn figma_www_root_11() {
    let res = reqwest_11::get("https://www.figma.com/").await.unwrap();
    assert_eq!(res.status(), 200, "Response failed: {res:#?}");
}

#[tokio::test]
async fn figma_www_root_12() {
    let res = reqwest_12::get("https://www.figma.com/").await.unwrap();
    assert_eq!(res.status(), 200, "Response failed: {res:#?}");
}

#[tokio::test]
async fn figma_api_root_11() {
    let res = reqwest_11::get("https://api.figma.com/").await.unwrap();
    assert_eq!(res.status(), 200, "Response failed: {res:#?}");
}

#[tokio::test]
async fn figma_api_root_12() {
    let res = reqwest_12::get("https://api.figma.com/").await.unwrap();
    assert_eq!(res.status(), 200, "Response failed: {res:#?}");
}

#[tokio::test]
async fn figma_api_me_11() {
    let res = reqwest_11::get("https://api.figma.com/v1/me")
        .await
        .unwrap();
    assert_eq!(res.status(), 403, "Expected 403 Forbidden: {res:#?}");
}

#[tokio::test]
async fn figma_api_api_me_12() {
    let res = reqwest_12::get("https://api.figma.com/v1/me")
        .await
        .unwrap();
    assert_eq!(res.status(), 403, "Expected 403 Forbidden: {res:#?}");
}
