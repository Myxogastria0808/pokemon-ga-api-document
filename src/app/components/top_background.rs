use stylist::style;
use yew::prelude::*;

#[function_component(TopBackGround)]
pub fn top_background() -> Html {
    let background_style = style!(
        r#"
            background-image: url(https://raw.githubusercontent.com/Myxogastria0808/pokemon-ga-api-document/main/assets/img/pexels-pixabay-36717.webp);
            background-position: center;
            background-repeat: no-repeat;
            background-size: cover;
            height: 100vh;
            width: 100%;
            position: relative;
    "#
    )
    .expect("Failed to mount style");
    let title_style = style!(
        r#"
            font-size: 48px;
            color: white;
            position: absolute;
            bottom: 50%;
            transform: translateY(50%);
            text-align:center;
            width:100%;
    "#
    )
    .expect("Failed to mount style");
    html! {
        <>
            <div class={background_style}>
                <h1 class={title_style}>{"Pokémon-GA API"}</h1>
            </div>
        </>
    }
}
