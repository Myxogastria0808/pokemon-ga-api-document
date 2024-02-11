use stylist::style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PathParamContent {
    pub title: String,
    pub path: String,
    pub content: String,
}
#[function_component(PathParam)]
pub fn path_param(props: &PathParamContent) -> Html {
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
            <h1 class={query_h1_style}>{ props.title.clone() }</h1>
            <h2 class={query_h2_style}>{ props.path.clone() }</h2>
            <p class={query_p_style}>
                { props.content.clone() }
            </p>
        </>
    }
}
