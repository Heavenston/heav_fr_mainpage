use leptos::*;

#[component]
fn App() -> impl IntoView { 
    view!{
        <div class="container">
            <div class="left">
                <h1>"Heavenstone"</h1>
                <ul>
                    <li>
                        <a href="https://github.com/Heavenston/heav_fr_mainpage">GitHub</a>
                    </li>
                </ul>
            </div>
            <div class="right">
                <section>
                    <h2>"Web"</h2>
                    <ul>
                        <li><a href="https://hfax.fr">Heav-Faxer</a></li>
                        <li><a href="https://isabellemaire.fr">isabellemaire.fr</a></li>
                        <li><a href="https://adhd.heav.fr">ADHD Bubbles</a></li>
                    </ul>
                </section>
                <section>
                    <h2>"Autre"</h2>
                    <ul>
                        <li><a href="#">Un autre truc</a></li>
                        <li><a href="#">Encore un autre</a></li>
                        <li><a href="#">Peut etre different?</a></li>
                    </ul>
                </section>
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
