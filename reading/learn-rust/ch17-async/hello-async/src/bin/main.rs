use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
	.select_first("title")
	.map(|title| title.inner_html())
}


//---  Above is the same as

/*
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
  async move {
    let text = trpl::get(url).await.text().await;
    Html::parse(&text)
      .select_first("title")
      .map(|title| title.inner_html())
  }
}

 */


fn main() {
    let args: Vec<String> = std::env::args().collect();

    // block_on async runtime:  Reference: Chap 17.1.
    //                 ^- async scheduler, (in Golang term, go routine scheduler(userspace))
    //                    - The books said:  Invisible Statemachine, (good expression)
    //
    trpl::block_on(async {
	let url = &args[1];
	match page_title(url).await {
	    Some(title) => println!("The title for {url} was --{title}--"),
	    None => println!("{url} had no title"),
	}
    })
}
