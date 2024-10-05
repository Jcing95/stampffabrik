use leptos;
use leptos::ev::MouseEvent;
use leptos::logging::log;
use validator::Validate;

use crate::app::model::user::{RegisterRequest};
use crate::app::server::auth::{register};

stylance::import_style!(style, "auth.module.scss");

#[leptos::component]
pub fn RegisterForm(
    set_show_register_modal: leptos::WriteSignal<bool>,
) -> impl leptos::IntoView {
    let (email, set_email) = leptos::create_signal(String::new());
    let (password, set_password) = leptos::create_signal(String::new());

    let (error_message, set_error_message) = leptos::create_signal(String::new());
    let (if_error, set_if_error) = leptos::create_signal(false);

    let on_back_pressed = move | _ | {
        set_show_register_modal(false);
    };

    let on_register = move |_| {
        let register_request = RegisterRequest::new(email(), password());
        let is_valid = register_request.validate();

        match is_valid {
            Ok(_) => {
                leptos::spawn_local(async move {
                    let register_result = register(register_request).await;

                    match register_result {
                        Ok(user) => {
                            log!{"success"};
                            set_show_register_modal(false);                            
                        }
                        Err(e) => {
                            log!("Error adding {:?}", e);
                            set_error_message(format! {"Error adding {:?}", e});
                        }
                    }
                });
            }
            Err(_) => {
                set_if_error(true);
                set_error_message(String::from("All fields are required!"));
            }
        }
    };

    let on_enter = move |e: leptos::ev::KeyboardEvent| {
        if e.key() == "Enter" {
            on_register(MouseEvent::new("").unwrap());
        }
    };

    log!{"creating component now!"};

    leptos::view! {
        <div class=style::back on:click=on_back_pressed>
            <div class=style::container on:click=|e: leptos::ev::MouseEvent| e.stop_propagation()>
                <input type="email" placeholder="E-Mail"
                    value=email
                    on:input=move |e| {
                        set_email(leptos::event_target_value(&e));
                    }
                    class=style::input
                />
                <input type="password" placeholder="Passwort"
                    value=password
                    on:input=move |e| {
                        set_password(leptos::event_target_value(&e));
                    }
                    on:keydown = on_enter
                    class=style::input
                />
                <span class=style::error_label>
                    <leptos::Show when = move || { if_error() }>
                        {error_message()}
                    </leptos::Show>
                </span>
                <button on:click=on_register class=style::button>Registrieren</button>
            </div>
        </div>
    }
}
