use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use std::path::PathBuf;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    for i in 2..182 {
        let img = format!("https://atariarchives.org/basicgames/pages/page{i}.gif");
        let response = reqwest::get(img).await?;

        let mut dest = {
            let fname = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() {None} else {Some(name)})
                .unwrap_or("tmp.bin");

            println!("file to download: {}", fname);
            let fname: PathBuf = ["./","pages",fname].iter().collect();
            println!("will be located under: '{:?}'", fname);
            File::create(fname)?
        };
        let mut content = std::io::Cursor::new(response.bytes().await?);
        copy(&mut content, &mut dest)?;
    };
    Ok(())
}
