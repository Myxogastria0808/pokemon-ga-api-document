use components::api::functions::Functions;
use components::top::Top;
use components::top_background::TopBackGround;
use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/pokemon-ga-api-document")]
    Root,
    #[at("/pokemon-ga-api-document/pokemon")]
    Pokemon,
    #[at("/pokemon-ga-api-document/generation")]
    Generation,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => html! {
            <>
                <Functions api={"latest"} />
            </>
        },
        Route::Pokemon => html! {
            <>
                <Functions api={"search_pokemon"} />
            </>
        },
        Route::Generation => html! {
            <>
                <Functions api={"archive"} />
            </>
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let container_style = style!(
        r#"
            width: 100%;
            max-width: 1024px;
            margin-right: auto;
            margin-left: auto;
        "#
    )
    .expect("Failed to mount style");

    html! {
        <>
            <TopBackGround />
            <div class={container_style}>
                <Top />
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </div>
        </>
    }
}
