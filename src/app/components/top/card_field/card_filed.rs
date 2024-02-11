use card::Card;
use stylist::style;
use yew::prelude::*;

mod card;

#[function_component(CardField)]
pub fn card_flied() -> Html {
    let card_field_div_style = style!(
        r#"
            width:100%;
            padding-bottom:90px;
            background-color:#000;
        "#
    )
    .expect("Failed to mount style");

    let card_field_h1_style = style!(
        r#"
            text-align:center;
            color:#fff;
            font-size:30px;
            font-family: 'Itim', cursive;
            padding:60px 10px;
        "#
    )
    .expect("Failed to mount style");

    let card_field_flex_style = style!(
        r#"
            display:flex;
            gap:42px;
            width:fit-content;
            max-width:95%;
            margin:0 auto;
            flex-wrap:wrap;
            justify-content:center;
        "#
    )
    .expect("Failed to mount style");
    let card_field_span_style = style!(
        r#"
            display: inline-block;
        "#
    )
    .expect("Failed to mount style");
    html! {
        <>
            <div class={card_field_div_style}>
                <h1 class={card_field_h1_style}>
                    <span class={card_field_span_style.clone()}>{"https://"}</span>
                    <span class={card_field_span_style.clone()}>{"pokemon-ga-api"}</span>
                    <span class={card_field_span_style}>{".yukiosada.work/"}</span>
                </h1>
                <div class={card_field_flex_style}>
                    <Card
                        title="pokemon"
                        title_size=35
                        content="登録されているポケモンの詳細情報を得ることができる。"
                        url="/pokemon-ga-api-document/pokemon"
                        img="https://raw.githubusercontent.com/pokemon-GA/pokemon-ga-api-document/main/assets/img/pexels-pixabay-206359.webp"
                    />
                    <Card
                        title="latest"
                        title_size=35
                        content="計算が終了している世代の中で最新の世代の情報を取得できる。"
                        url="/pokemon-ga-api-document"
                        img="https://raw.githubusercontent.com/pokemon-GA/pokemon-ga-api-document/main/assets/img/pexels-quang-nguyen-vinh-2131801.webp"
                    />
                    <Card
                        title="generation"
                        title_size=35
                        content="計算が終了している過去の世代の情報を取得できる。"
                        url="/pokemon-ga-api-document/generation"
                        img="https://raw.githubusercontent.com/pokemon-GA/pokemon-ga-api-document/main/assets/img/pexels-konevi-1335115.webp"
                    />
                </div>
            </div>
        </>
    }
}
