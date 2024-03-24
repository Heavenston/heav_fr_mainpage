use leptos::*;

#[component]
fn App() -> impl IntoView { 
    view!{
        <div class="container">
            <div class="inner">
                <h1>"Heavenstone"</h1>
                <ul>
                    <li><a href="https://github.com/Heavenston">GitHub</a></li>
                    <li><a href="https://hfax.fr">Heav-Faxer</a></li>
                    <li><a href="https://isabellemaire.fr">isabellemaire.fr</a></li>
                </ul>
            </div>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! {
        <App />
    })
}
