use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    // create database and navigator
    let db = Rc::new(JiraDatabase::new("data/db.json".to_owned()));
    let mut nav = Navigator::new(db.clone());

    loop {
        clearscreen::clear().unwrap();
        // 1. get current page from navigator. If there is no current page exit the loop.
        if let Some(page) = nav.get_current_page() {
            // 2. render page
            if let Err(e) = page.draw_page() {
                println!("Error rendering page: {e}\nPress any key to continue...");
                wait_for_key_press();
            };
            // 3. get user input
            // 4. pass input to page's input handler
            // 5. if the page's input handler returns an action let the navigator process the action
            match page.handle_input(&io_utils::get_user_input().trim()) {
                Err(e) => {
                    println!("Error handling input {e}\nPress any key to continue...");
                    wait_for_key_press();
                }
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(e) = nav.handle_action(action) {
                            println!("Error handling action: {e}\nPress any key to continue...");
                            wait_for_key_press();
                        };
                    }
                }
            }
        } else {
            break;
        }
    }
}
