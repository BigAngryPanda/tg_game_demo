mod point;
mod shape;
mod shader;
mod log;
mod render;
mod game;
mod rand;
mod ui;
mod game_state;
mod texture;
mod scene;

use wasm_bindgen::prelude::*;

use point::*;
use game::*;

use std::rc::Rc;
use std::cell::RefCell;

fn set_input_callback(game: Rc<RefCell<Game>>) {
    let window = game.as_ref().borrow().window();

    let callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let wnd = game.as_ref().borrow().window();

        let width: f32 = wnd.inner_width().expect("Failed to get window width").as_f64().unwrap() as f32;
        let height: f32 = wnd.inner_height().expect("Failed to get window height").as_f64().unwrap() as f32;

        let e = event.dyn_into::<web_sys::MouseEvent>().expect("Failed to get mouse event");

        game.as_ref().borrow_mut().store_input(Point::from_screen_coords(e.x() as f32 / width, e.y() as f32 / height));
    }) as Box<dyn FnMut(_)>);

    window.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref()).expect("Failed to set event listener");

    callback.forget();
}

fn run_loop(game: Rc<RefCell<Game>>) {
    let draw_closure = Rc::new(RefCell::new(None));
    let draw_closure_clone = draw_closure.clone();

    let game_clone = game.clone();

    *draw_closure_clone.borrow_mut() = Some(Closure::new(move || {
        game.as_ref().borrow_mut().run();

        // Schedule ourself for another requestAnimationFrame callback.
        game.as_ref().borrow().request_next_frame(draw_closure.borrow().as_ref().unwrap())
            .expect("Failed to request new frame");
    }));

    game_clone.as_ref().borrow().request_next_frame(draw_closure_clone.borrow().as_ref().unwrap())
        .expect("Failed to request new frame");
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let game: Rc<RefCell<Game>> = Rc::new(RefCell::new(Game::new()));

    set_input_callback(game.clone());
    run_loop(game.clone());

    Ok(())
}
