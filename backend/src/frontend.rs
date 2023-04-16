use moon::*;

pub async fn frontend() -> Frontend {
    Frontend::new()
        .title("Libredu(Beta)")
        .append_to_head("<link href='/_api/public/fontawesome/css/all.css' rel='stylesheet'>")
    //.append_to_head(include_str!("../favicon.html")) // realfavicongenerator.net
}
