use stylist::style;
use yew::prelude::*;

#[function_component(RealNumber)]
pub fn real_number() -> Html {
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
            <h1 class={query_h1_style}>{ "実数値の計算" }</h1>
            <h2 class={query_h2_style.clone()}>{ "HPの計算" }</h2>
            <p class={query_p_style.clone()}>
                {"
                    (( (pk_data.base_stat_h.clone()).parse::<f64>().unwrap() * 2.0 + (pk_data.individual_values_h.clone()).parse::<f64>().unwrap() + ( (pk_data.effort_values_h.clone()).parse::<f64>().unwrap() / 4.0 ).floor() ) / 2.0).floor() + 60.0
                "}
            </p>
                        <h2 class={query_h2_style}>{ "HP以外の計算 (以下の計算は、aについての計算)" }</h2>
            <p class={query_p_style}>
                {"
                    ( ((((pk_data.base_stat_a.clone()).parse::<f64>().unwrap() * 2.0 + (pk_data.individual_values_a.clone()).parse::<f64>().unwrap() + ((pk_data.effort_values_a.clone()).parse::<f64>().unwrap() / 4.0).floor()) / 2.0).floor() + 5.0) * (1.0 + ((pk_data.nature_a.clone()).parse::<f64>().unwrap() * 0.1)) ).floor()
                "}
            </p>
        </>
    }
}
