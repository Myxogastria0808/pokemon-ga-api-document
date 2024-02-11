use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/pokemon-ga-api-document")]
    Root,
    #[at("/pokemon-ga-api-document/pokemon")]
    Pokemon,
    #[at("/pokemon-ga-api-document/generation")]
    Generation,
}

impl Route {
    fn rooter(url: &String) -> Result<Route, String> {
        if url == "/pokemon-ga-api-document" {
            Ok(Route::Root)
        } else if url == "/pokemon-ga-api-document/pokemon" {
            Ok(Route::Pokemon)
        } else if url == "/pokemon-ga-api-document/generation" {
            Ok(Route::Generation)
        } else {
            Err(String::from("Rooting error"))
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CardContent {
    pub title: String,
    pub title_size: usize,
    pub content: String,
    pub url: String,
    pub img: String,
}
#[function_component(Card)]
pub fn card(props: &CardContent) -> Html {
    let card_style = style!(
        r#"
            background-position: center;
            background-repeat: no-repeat;
            background-size: cover;
            position: relative;
            width: 240px;
            height: 400px;
            box-sizing:border-box;
            padding:0 9px;
    "#
    )
    .expect("Failed to mount style");

    let card_h1_style = style!(
        r#"
            font-family: 'Itim', cursive;
            color: #fff;
            padding-top:15px;
            padding-bottom:25px;
            text-align:center;
    "#
    )
    .expect("Failed to mount style");

    let card_p_style = style!(
        r#"
            font-size:20px;
            color:#fff;
        "#
    )
    .expect("Failed to mount style");

    let card_button_style = style!(
        r#"
            width:174px;
            height:40px;
            background-color:#000;
            border-radius:10px;
            box-shadow: 0px 0px 20px #fff ;
            border:0px;
            box-sizing:border-box;
            display:block;
            margin: 40px auto 0;
            color:#fff;
            line-height:40px;
            font-size:23px;
            text-align:center;
            font-family: 'Itim', cursive;
        "#
    )
    .expect("Failed to mount style");

    let navigator = use_navigator().unwrap();
    if &props.url == "/pokemon-ga-api-document"
        || &props.url == "/pokemon-ga-api-document/pokemon"
        || &props.url == "/pokemon-ga-api-document/generation"
    {
        let url = props.url.clone();
        let onclick = Callback::from(move |_| {
            navigator.push(match &Route::rooter(&url) {
                Ok(o_) => o_,
                Err(e_) => panic!("{}", e_),
            })
        });
        html! {
            <>
                <div
                    class={card_style}
                    style={ format!("background-image: url({})", props.img.clone()) }
                >
                    <h1 class={card_h1_style} style={ format!("font-size: {}px", props.title_size.clone()) }>{ props.title.clone() }</h1>
                    <p class={card_p_style}>{ props.content.clone() }</p>
                    <button class={card_button_style} style="cursor:pointer;" onclick={onclick}>{ "Detail" }</button>
                </div>
            </>
        }
    } else {
        html! {
            <></>
        }
    }
}
