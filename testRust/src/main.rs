mod service;
mod model;
mod convertion;
use crate::service::image_service::run_main_logic;

fn main() {
    run_main_logic("ppsteam.jpg");
    run_main_logic("ppsteamPNG.png");
}
