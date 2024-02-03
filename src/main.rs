use app::App;
use not_found::NotFound;
use stylist::css;
use stylist::yew::Global;
use yew::prelude::*;
use yew_hooks::use_favicon;
use yew_router::prelude::*;

mod app;
mod not_found;

#[function_component(Favicon)]
pub fn favicon() -> Html {
    use_favicon(
        "https://raw.githubusercontent.com/pokemon-GA/DashApp-example/main/assets/pokemon-GA.ico"
            .to_string(),
    );

    html! {
        <>
        </>
    }
}

//現状だと、Appのルートをここに書く必要あり
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/pokemon-ga-api-document")]
    Route,
    #[at("/pokemon-ga-api-document/pokemon")]
    Pokemon,
    #[at("/pokemon-ga-api-document/generation")]
    Generation,
    #[not_found]
    #[at("/pokemon-ga-api-document/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    if let Route::NotFound = routes {
        html! {
            <>
                <NotFound />
            </>
        }
    } else {
        html! {
            <>
                <App />
            </>
        }
    }
}

#[function_component(Main)]
fn root() -> Html {
    let global_style = css!(
        r#"
            * {
                margin: 0px;
                padding: 0px;
                font-family: 'Noto Sans JP', sans-serif;
            }
            html {
                background-color: #000;
            }
            body {
                background-color: #000;
            }
        "#
    );

    html! {
        <>
            <Global css={global_style}/>
            <Favicon />
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
