mod models;
mod db;

mod ui;
mod io_utils;
mod navigator;

use std::rc::Rc;
use io_utils::*;
use navigator::*;
use db::*;

fn main() {
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    let mut navigator = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        if let Some(page) = navigator.get_current_page() {
            if let Err(error) = page.draw_page(){
                println!(
                    "Error rendering page: {error}\nPress any key to continue..."
                );
                wait_for_key_press();
            }

            let user_input = get_user_input();

            match page.handle_input(user_input.trim()) {
                Err(err) => {
                    println!("Error getting user input: {err}\nPress any key to continue...");
                    wait_for_key_press();
                },
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(err) = navigator.handle_action(action) {
                            println!("Error handling processing user input: {err}\nPress any key to continue...");
                            wait_for_key_press();
                        }
                    }
                }
            }
        }else{
            break;
        }

    }
}
