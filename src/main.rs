#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let res = reqwest::get("https://www.figma.com/").await?;
    // let res = reqwest::get("https://api.figma.com/v1/activity_logs").await?;
    let res = reqwest_11::get("https://api.figma.com/").await?;
    let status = res.status();

    if status == 200 {
        println!("Success!");
    } else {
        eprintln!("Failed to retrieve data: {status}");
    }
    dbg!(res);

    let res = reqwest_12::get("https://api.figma.com/").await?;
    let status = res.status();

    if status == 200 {
        println!("Success!");
    } else {
        eprintln!("Failed to retrieve data: {status}");
    }
    dbg!(res);

    Ok(())
}

