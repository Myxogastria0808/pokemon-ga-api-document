use stylist::style;
use yew::prelude::*;

#[function_component(Query)]
pub fn card() -> Html {
    let query_h1_style = style!(
        r#"
            text-align:center;
            color:#fff;
            font-size:30px;
            font-family: 'Itim', cursive;
            padding: 10px;
        "#
    )
    .expect("Failed to mount style");
    let query_h2_style = style!(
        r#"
            text-align:center;
            color:#fff;
            font-size:25px;
            font-family: 'Itim', cursive;
            padding:10px 10px 30px 10px;
        "#
    )
    .expect("Failed to mount style");

    let query_p_style = style!(
        r#"
            color: #fff;
            padding:0 30px;
            margin-bottom:100px;
        "#
    )
    .expect("Failed to mount style");

    html! {
        <>
            <h1 id="api" class={query_h1_style}>{ "取得するパーティ数を絞る" }</h1>
            <h2 class={query_h2_style}>{ "?higher={higher}&lower={lower}" }</h2>
            <p class={query_p_style}>
                {"
                    / と /generation/{generation} には、上位何位(higher)から下位何位(lower)までのパーティ数を取得するという指定ができる。
                    何も指定しない場合は、1位から10位まで(?higher=1&lower=10)を取得するようになっている。
                "}
            </p>
        </>
    }
}
