use reqwest::Method;
use somes_common_lib::{SignUpInfo, errors::SignUpError};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::request;

#[function_component(Login)]
fn login() -> Html {

    let sign_up_info = use_state(SignUpInfo::default);

    let on_input_username = {
        let sign_up_info = sign_up_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*sign_up_info).clone();
            info.username = input.value();
            sign_up_info.set(info);
        })
    };

    let on_input_email = {
        let sign_up_info = sign_up_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*sign_up_info).clone();
            info.email = input.value();
            sign_up_info.set(info);
        })
    };

    let on_input_password = {
        let sign_up_info = sign_up_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*sign_up_info).clone();
            info.password = input.value();
            sign_up_info.set(info);
        })
    };
    
    let on_click_register = {
        let sign_up_info = sign_up_info.clone();
        Callback::from(move |e: MouseEvent| {
            let sign_up_info = sign_up_info.clone();
            wasm_bindgen_futures::spawn_local(async move {
                
                request::<_, SignUpError>(Method::POST, somes_common_lib::SIGNUP_ROUTE, (*sign_up_info).clone());
                /*if let Ok(err_msgs) = request::<UploadInfo, UploadMsgs>(
                    Method::POST,
                    "upload",
                    (*upload_info).clone(),
                ) {
                    
                }*/
            });
        })
    };

    html! {
        <>
            <input type="text" oninput={on_input_username} placeholder = "username" />
            <input type="text" oninput={on_input_email} placeholder = "email" />
            <input type="text" oninput={on_input_password} placeholder = "password" />

            <button onclick={on_click_register} class="btn btn-primary">
                    {"Register"}
            </button>
        </>
    }
}