use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigLink {
    // name of the link
    pub name: String,
    // the hyperlink reference
    pub href: String,
    // the title of the anchor
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct NavbarConfig {
    // links to be added to navbar
    pub links: Vec<ConfigLink>,
}

#[derive(Debug, Deserialize)]
pub struct UrchinConfig {
    // address to the database (just IP)
    pub database_address: String,
    // port of the database service
    pub database_port: u16,
    // username for database access
    pub database_user: String,
    // password to the database service
    pub database_password: String,
    // name of the database
    pub database_name: String,
    // port to use for the app
    pub webserver_port: u16,
    // port to use for the admin app
    pub admin_port: u16,
    // directory to use for storing and retrieving
    // images
    pub image_dir: String,
    // enable or disable the cache
    pub cache_enabled: bool,
    // sitekey for recaptcha
    pub recaptcha_sitekey: String,
    // secret for recaptcha
    pub recaptcha_secret: String,
    // navbar config
    pub navbar: NavbarConfig,
}


// [navbar]
// links = [
//     { name = "Home", href = "/", title = "Homepage" },
//     { name = "About", href = "/about", title = "About page" },
//     { name = "Services", href = "/services", title = "Services page" },
//     { name = "Images", href = "/images", title = "Images page" },
//     { name = "Contact", href = "/contact", title = "Contacts page" },
// ]