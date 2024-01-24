use std::error::Error;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    // let agent = ureq::AgentBuilder::new().build();
    let agent = ureq::AgentBuilder::new()
        .tls_connector(Arc::new(native_tls::TlsConnector::new()?))
        .build();
    let body = agent
        .get("http://wttr.in/11231?format=%c%t")
        .call()?
        .into_string()?;

    println!("{}", body.replace(" ", ""));

    Ok(())
}
