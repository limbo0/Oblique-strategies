#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let result = call_oblique().await.unwrap();
    Ok(())
}

async fn call_oblique() -> anyhow::Result<String> {
    let res = reqwest::get("http://www.oblique-strategies.com/index.php").await?;
    //println!("Status: {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;

    // index of opening article html element.
    let article_index_opening = body.find("<article>");

    // split using the index above.
    // we only care about the last part.
    let (_, last) = body.split_at(article_index_opening.unwrap());

    // split till the html element closing.
    let article_index_closing = last.find("</article>");
    let (article_captured, _) = last.split_at(article_index_closing.unwrap());

    let h2_opening = article_captured.find("<h2>");

    // while split add the len of the <h2> including closing and opening <>;
    let (_, last) = article_captured.split_at(h2_opening.unwrap() + 4);

    let h2_closing = last.find("</h2>");
    let (finale, _) = last.split_at(h2_closing.unwrap());

    println!("{:?}", finale);
    Ok(String::from(finale))
}
