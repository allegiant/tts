#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;

use crate::core::layout;


fn main() {
    layout::init();
}









