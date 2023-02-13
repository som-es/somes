use gloo::storage::{LocalStorage, Storage};
use parking_lot::RwLock;
use yew::prelude::*;

use lazy_static::lazy_static;
use yew_router::prelude::*;
use yew_router::Switch;

use crate::routes::{switch, Route};

const TOKEN_KEY: &str = "access_token";

lazy_static! {
    /// Jwt token read from local storage.
    pub static ref TOKEN: RwLock<Option<String>> = {
        if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

/// Set jwt token to local storage.
pub fn set_jwt(token: Option<String>) {
    if let Some(t) = token.clone() {
        LocalStorage::set(TOKEN_KEY, t).expect("failed to set");
    } else {
        LocalStorage::delete(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write();
    *token_lock = token;
}

/// Get jwt token from lazy static.
pub fn get_jwt() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        //<UserContextProvider>
            <BrowserRouter>
                //<NavBar />
                // pushes footer down
                //<div class="flex-wrapper">
                    <Switch<Route> render={switch} />
                    //<Footer />
                //</div>
            </BrowserRouter>
        //</UserContextProvider>
    }
}
