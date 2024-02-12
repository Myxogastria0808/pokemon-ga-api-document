use serde::{de::DeserializeOwned, Deserialize, Serialize};
use stylist::style;
use web_sys::HtmlInputElement;
use yew::prelude::*;

///fetch関数
async fn fetch<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await;
    if let Ok(data) = response {
        if let Ok(res) = data.json::<T>().await {
            Ok(res)
        } else {
            Err(Error::DeserializeError)
        }
    } else {
        Err(Error::RequestError)
    }
}

//取得したjsonのデータをパースした時に格納するための構造体
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct MoveData {
    name: String,
    move_type: String,
    kind: String,
    power: String,
    accuracy: String,
    pp: String,
    contact: String,
    protect: String,
    range: String,
    effect: String,
    effect1: String,
    effect2: String,
    effect3: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct PkData {
    pokedex_no: String,
    form_no: String,
    rank: String,
    name: String,
    form_name: String,
    type_1: String,
    type_2: String,
    base_stat_h: String,
    base_stat_a: String,
    base_stat_b: String,
    base_stat_c: String,
    base_stat_d: String,
    base_stat_s: String,
    base_stat_all: String,
    individual_values_h: String,
    individual_values_a: String,
    individual_values_b: String,
    individual_values_c: String,
    individual_values_d: String,
    individual_values_s: String,
    effort_values_h: String,
    effort_values_a: String,
    effort_values_b: String,
    effort_values_c: String,
    effort_values_d: String,
    effort_values_s: String,
    nature_a: String,
    nature_b: String,
    nature_c: String,
    nature_d: String,
    nature_s: String,
    item: String,
    ability: String,
    tera_type: String,
    move_1: MoveData,
    move_2: MoveData,
    move_3: MoveData,
    move_4: MoveData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct JsonParce {
    data: Vec<PkData>,
}

// エラーの種類に関して、enumで定義している
#[derive(Clone, Debug, PartialEq)]
enum Error {
    RequestError,
    DeserializeError,
    // etc.
}

#[function_component(SearchPokemon)]
pub fn search_pokemon() -> Html {
    let pk_name = use_state(|| "".to_string());

    let onchange = {
        let pk_name = pk_name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            pk_name.set(input.value().parse().expect("This value is not integer"))
        })
    };

    //情報の取得
    let pk_name_clone = pk_name.clone();
    let pk_name = pk_name.clone();
    let state = yew_hooks::prelude::use_async(async move {
        fetch::<JsonParce>(format!(
            "https://pokemon-ga-api.yukiosada.work/pokemon/{}",
            *pk_name_clone
        ))
        .await
    });

    let onclick = {
        let state = state.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // You can trigger to run in callback or use_effect.
            state.run();
        })
    };

    let search_form_style = style!(
        r#"
            background-color:#000;
        "#
    )
    .expect("Failed to mount style");

    let search_div_style = style!(
        r#"
            display:flex;
            gap:43px;
            width:fit-content;
            max-width:85%;
            margin:0 auto;
            flex-wrap:wrap;
            justify-content:center;
        "#
    )
    .expect("Failed to mount style");

    let search_input_style = style!(
        r#"
            width:500px;
            height:40px;
            box-shadow: 0px 0px 20px #ff0 ;
            background-color:#000;
            border:0px;
            border-radius:10px;
            color:#fff;
            font-family: 'Itim', cursive;
            box-sizing:border-box;
            padding-left:8px;
            font-size:20px;
        "#
    )
    .expect("Failed to mount style");

    let search_button_style = style!(
        r#"
            box-shadow: 0px 0px 20px #ff0 ;
            width:120px;
            height:40px;
            background-color:#000;
            border:0px;
            border-radius:10px;
            color:#fff;
            font-family: 'Itim', cursive;
            font-size:24px;
        "#
    )
    .expect("Failed to mount style");
    let search_space_style = style!(
        r#"
            width: 100%;
            height: 100px;
            background-color: #000;
        "#
    )
    .expect("Failed to mount style");
    let search_url_style = style!(
        r#"
            color: #fff;
            font-size: 25px;
            text-align: center;
            margin-bottom: 60px;
            font-family: 'Itim', cursive;
        "#
    )
    .expect("Failed to mount style");
    let info_style = style!(
        r#"
            color: #fff;
            margin-top: 5px;
            margin-bottom: 30px;
        "#
    )
    .expect("Failed to mount style");
    let quote_style = style!(
        r#"
            color: #fff;
            margin-top: 30px;
        "#
    )
    .expect("Failed to mount style");
    let ul_color_pokemon_style = style!(
        r#"
            color: #fff;
    "#
    )
    .expect("Failed to mount style");
    let list_pokemon_style = style!(
        r#"
            list-style: none;
    "#
    )
    .expect("Failed to mount style");
    let search_pk_label_p_style = style!(
        r#"
        width: 113px;
        font-size: 18px;
    "#
    )
    .expect("Failed to mount style");
    let search_type_p_style = style!(
        r#"
            width: 150px;
            text-align: center;
            font-size: 18px;
    "#
    )
    .expect("Failed to mount style");
    let search_pk_p_style = style!(
        r#"
            font-size: 18px;
    "#
    )
    .expect("Failed to mount style");
    let search_pk_div_style = style!(
        r#"
            display: flex;
            gap: 10px;
            padding: 5px 0px;
    "#
    )
    .expect("Failed to mount style");
    let search_move_p_style = style!(
        r#"
            font-size: 18px;
            display: flex;
            padding: 5px 0px;
    "#
    )
    .expect("Failed to mount style");
    let search_pokedex_no_p_style = style!(
        r#"
        font-size: 18px;
    "#
    )
    .expect("Failed to mount style");
    let search_pokemon_name_style = style!(
        r#"
        font-size: 30px;
        padding-top: 10px;
    "#
    )
    .expect("Failed to mount style");
    let search_pk_value_p_style = style!(
        r#"
            font-size: 18px;
            width: 100px;
        "#
    )
    .expect("Failed to mount style");

    html! {
        <>
            <h1 class={search_url_style}>
                { format!("https://pokemon-ga-api.yukiosada.work/pokemon/{}", *pk_name) }
            </h1>
            <form class={search_form_style}>
                <div class={search_div_style}>
                    <input class={search_input_style} type="text" onchange={onchange} id="pk_name" placeholder="ポケモンの名前を入力" />
                    <button class={search_button_style} style="cursor:pointer" type="submit" onclick={onclick} disabled={state.loading}>{"Search"}</button>
                </div>
            </form>
            <p class={ quote_style }>{ "タイプ、テラタイプの色の引用元: https://www.pokemon.co.jp/ex/sun_moon/fight/161215_01.html" }</p>
            <p class={ info_style.clone() }>{ "※以下の情報は、上記のURLから取得できる情報の一部であることを理解の上、ご利用ください。" }</p>
            {
                if state.loading {
                    html! { <p>{ "Loading, wait a sec..." }</p> }
                } else {
                    html! {}
                }
            }
            {
                if let Some(error) = &state.error {
                    match error {
                        Error::DeserializeError => html! { <p class={ info_style }>{ "登録されているポケモンの名前を正確に入力してください。" }</p> },
                        Error::RequestError => html! {<p>{ "RequestError" }</p> },
                    }
                } else {
                    html! {}
                }
            }
            <div class={search_space_style}></div>
            {
                if let Some(pokemon) = &state.data {
                    html! {
                        <>
                            <ul class={ul_color_pokemon_style}>
                                {(pokemon.data).iter().map(|pk_data| html! {
                                    <li class={list_pokemon_style.clone()}>
                                        <p class={ search_pokedex_no_p_style.clone() }>{ format!("図鑑No.{}", pk_data.pokedex_no.clone()) }</p>
                                        <h3 class={ search_pokemon_name_style.clone() }>
                                            {
                                                format!(
                                                    "{}　{}",
                                                    pk_data.name.clone(),
                                                    match pk_data.form_name.as_str() {
                                                        "null" => "".to_string(),
                                                        _ => pk_data.form_name.clone(),
                                                    },
                                                )
                                            }
                                        </h3>
                                        <div class={ search_pk_div_style.clone() }>
                                            <p class={ search_pk_label_p_style.clone() }>{ "タイプ:" }</p>
                                            <p class={ search_type_p_style.clone() } style={
                                                match pk_data.type_1.as_str() {
                                                        "ノーマル" => "background-color: rgb(174, 174, 174);".to_string(),
                                                        "ほのお" => "background-color: rgb(255, 167, 102);".to_string(),
                                                        "みず" => "background-color: rgb(100, 197, 247);".to_string(),
                                                        "でんき" => "background-color: rgb(231, 212, 0);".to_string(),
                                                        "くさ" => "background-color: rgb(154, 195, 14);".to_string(),
                                                        "こおり" => "background-color: rgb(96, 233, 245);".to_string(),
                                                        "かくとう" => "background-color: rgb(238, 105, 105);".to_string(),
                                                        "どく" => "background-color: rgb(171, 122, 202);".to_string(),
                                                        "じめん" => "background-color: rgb(200, 168, 65);".to_string(),
                                                        "ひこう" => "background-color: rgb(100, 167, 241);".to_string(),
                                                        "エスパー" => "background-color: rgb(235, 127, 244);".to_string(),
                                                        "むし" => "background-color: rgb(81, 203, 90);".to_string(),
                                                        "いわ" => "background-color: rgb(250, 199, 39);".to_string(),
                                                        "ゴースト" => "background-color: rgb(117, 110, 180);".to_string(),
                                                        "ドラゴン" => "background-color: rgb(255, 136, 89);".to_string(),
                                                        "あく" => "background-color: rgb(104, 129, 212);".to_string(),
                                                        "はがね" => "background-color: rgb(129, 138, 164);".to_string(),
                                                        "フェアリー" => "background-color: rgb(252, 119, 153);".to_string(),
                                                        _ => "background-color: rgb(0, 0, 0); color: ragb(0, 0, 0);".to_string(),
                                                }
                                            }>{ pk_data.type_1.clone() }</p>
                                            <p class={ search_type_p_style.clone() } style={
                                                match pk_data.type_2.as_str() {
                                                        "ノーマル" => "background-color: rgb(174, 174, 174);".to_string(),
                                                        "ほのお" => "background-color: rgb(255, 167, 102);".to_string(),
                                                        "みず" => "background-color: rgb(100, 197, 247);".to_string(),
                                                        "でんき" => "background-color: rgb(231, 212, 0);".to_string(),
                                                        "くさ" => "background-color: rgb(154, 195, 14);".to_string(),
                                                        "こおり" => "background-color: rgb(96, 233, 245);".to_string(),
                                                        "かくとう" => "background-color: rgb(238, 105, 105);".to_string(),
                                                        "どく" => "background-color: rgb(171, 122, 202);".to_string(),
                                                        "じめん" => "background-color: rgb(200, 168, 65);".to_string(),
                                                        "ひこう" => "background-color: rgb(100, 167, 241);".to_string(),
                                                        "エスパー" => "background-color: rgb(235, 127, 244);".to_string(),
                                                        "むし" => "background-color: rgb(81, 203, 90);".to_string(),
                                                        "いわ" => "background-color: rgb(250, 199, 39);".to_string(),
                                                        "ゴースト" => "background-color: rgb(117, 110, 180);".to_string(),
                                                        "ドラゴン" => "background-color: rgb(255, 136, 89);".to_string(),
                                                        "あく" => "background-color: rgb(104, 129, 212);".to_string(),
                                                        "はがね" => "background-color: rgb(129, 138, 164);".to_string(),
                                                        "フェアリー" => "background-color: rgb(252, 119, 153);".to_string(),
                                                        _ => "background-color: rgb(0, 0, 0); color: rgb(0, 0, 0);".to_string(),
                                                }
                                            }>{ pk_data.type_2.clone() }</p>
                                        </div>
                                        <div class={ search_pk_div_style.clone() }>
                                            <p class={ search_pk_label_p_style.clone() }>{ "テラスタイプ:" }</p>
                                            <p class={ search_type_p_style.clone() } style={
                                                match pk_data.tera_type.as_str() {
                                                        "ノーマル" => "background-color: rgb(174, 174, 174);".to_string(),
                                                        "ほのお" => "background-color: rgb(255, 167, 102);".to_string(),
                                                        "みず" => "background-color: rgb(100, 197, 247);".to_string(),
                                                        "でんき" => "background-color: rgb(231, 212, 0);".to_string(),
                                                        "くさ" => "background-color: rgb(154, 195, 14);".to_string(),
                                                        "こおり" => "background-color: rgb(96, 233, 245);".to_string(),
                                                        "かくとう" => "background-color: rgb(238, 105, 105);".to_string(),
                                                        "どく" => "background-color: rgb(171, 122, 202);".to_string(),
                                                        "じめん" => "background-color: rgb(200, 168, 65);".to_string(),
                                                        "ひこう" => "background-color: rgb(100, 167, 241);".to_string(),
                                                        "エスパー" => "background-color: rgb(235, 127, 244);".to_string(),
                                                        "むし" => "background-color: rgb(81, 203, 90);".to_string(),
                                                        "いわ" => "background-color: rgb(250, 199, 39);".to_string(),
                                                        "ゴースト" => "background-color: rgb(117, 110, 180);".to_string(),
                                                        "ドラゴン" => "background-color: rgb(255, 136, 89);".to_string(),
                                                        "あく" => "background-color: rgb(104, 129, 212);".to_string(),
                                                        "はがね" => "background-color: rgb(129, 138, 164);".to_string(),
                                                        "フェアリー" => "background-color: rgb(252, 119, 153);".to_string(),
                                                        _ => "background-color: rgb(0, 0, 0);".to_string(),
                                                }
                                            }>{ pk_data.tera_type.clone() }</p>
                                        </div>
                                        <div class={ search_pk_div_style.clone() }>
                                            <p class={ search_pk_label_p_style.clone() }>{ "持ち物:" }</p>
                                            <p class={ search_pk_p_style.clone() }>{ pk_data.item.clone() }</p>
                                        </div>
                                        <div class={ search_pk_div_style.clone() }>
                                            <p class={ search_pk_label_p_style.clone() }>{ "特性:" }</p>
                                            <p class={ search_pk_p_style.clone() }>{ pk_data.ability.clone() }</p>
                                        </div>
                                        <div class={ search_pk_div_style.clone() }>
                                            <p class={ search_pk_label_p_style.clone() }>{ "性格:" }</p>
                                            <p class={ search_pk_p_style.clone() }>
                                                {
                                                    if pk_data.nature_a.as_str() == "1" {
                                                        if pk_data.nature_b.as_str() == "-1" {
                                                            "さみしがり".to_string()
                                                        } else if pk_data.nature_c.as_str() == "-1" {
                                                            "いじっぱり".to_string()
                                                        } else if pk_data.nature_d.as_str() == "-1" {
                                                            "やんちゃ".to_string()
                                                        } else if pk_data.nature_s.as_str() == "-1" {
                                                            "ゆうかん".to_string()
                                                        } else {
                                                            "Unexpected Error".to_string()
                                                        }
                                                    } else if pk_data.nature_a.as_str() == "-1" {
                                                        if pk_data.nature_b.as_str() == "1" {
                                                            "ずぶとい".to_string()
                                                        } else if pk_data.nature_c.as_str() == "1" {
                                                            "ひかえめ".to_string()
                                                        } else if pk_data.nature_d.as_str() == "1" {
                                                            "おだやか".to_string()
                                                        } else if pk_data.nature_s.as_str() == "1" {
                                                            "おくびょう".to_string()
                                                        } else {
                                                            "Unexpected Error".to_string()
                                                        }
                                                    } else if pk_data.nature_a.as_str() == "0" {
                                                        if pk_data.nature_b.as_str() == "1" {
                                                            if pk_data.nature_c.as_str() == "-1" {
                                                                "わんぱく".to_string()
                                                            } else if pk_data.nature_d.as_str() == "-1" {
                                                                "のうてんき".to_string()
                                                            } else if pk_data.nature_s.as_str() == "-1" {
                                                                "のんき".to_string()
                                                            } else {
                                                                "Unexpected Error".to_string()
                                                            }
                                                        } else if pk_data.nature_b.as_str() == "-1" {
                                                            if pk_data.nature_c.as_str() == "1" {
                                                                "おっとり".to_string()
                                                            } else if pk_data.nature_d.as_str() == "1" {
                                                                "おとなしい".to_string()
                                                            } else if pk_data.nature_s.as_str() == "1" {
                                                                "せっかち".to_string()
                                                            } else {
                                                                "Unexpected Error".to_string()
                                                            }
                                                        } else if pk_data.nature_b.as_str() == "0" {
                                                            if pk_data.nature_c.as_str() == "1" {
                                                                if pk_data.nature_d.as_str() == "-1" {
                                                                    "うっかりや".to_string()
                                                                } else if pk_data.nature_s.as_str() == "-1" {
                                                                    "れいせい".to_string()
                                                                } else {
                                                                    "Unexpected Error".to_string()
                                                                }
                                                            } else if pk_data.nature_c.as_str() == "-1" {
                                                                if pk_data.nature_d.as_str() == "1" {
                                                                    "しんちょう".to_string()
                                                                } else if pk_data.nature_s.as_str() == "1" {
                                                                    "ようき".to_string()
                                                                } else {
                                                                    "Unexpected Error".to_string()
                                                                }
                                                            } else if pk_data.nature_c.as_str() == "0" {
                                                                if pk_data.nature_d.as_str() == "1" {
                                                                    "なまいき".to_string()
                                                                } else if pk_data.nature_s.as_str() == "1" {
                                                                    "むじゃき".to_string()
                                                                } else if pk_data.nature_d.as_str() == "0" && pk_data.nature_s.as_str() == "0" {
                                                                    "てれや or がんばりや or すなお or きまぐれ or まじめ".to_string()
                                                                } else {
                                                                    "Unexpected Error".to_string()
                                                                }
                                                            } else {
                                                            "Unexpected Error".to_string()
                                                            }
                                                        } else {
                                                            "Unexpected Error".to_string()
                                                        }
                                                    } else {
                                                        "Unexpected Error".to_string()
                                                    }
                                                }
                                            {
                                                format!("　( {}{}{}{}{})",
                                                    {
                                                        match pk_data.nature_a.as_str() {
                                                            "1" => "攻撃: ↑ ".to_string(),
                                                            "0" => "".to_string(),
                                                            "-1" => "攻撃: ↓ ".to_string(),
                                                            _ => "".to_string(),
                                                        }
                                                    },
                                                    {
                                                        match pk_data.nature_b.as_str() {
                                                            "1" => "防御: ↑ ".to_string(),
                                                            "0" => "".to_string(),
                                                            "-1" => "防御: ↓ ".to_string(),
                                                            _ => "".to_string(),
                                                        }
                                                    },
                                                    {
                                                        match pk_data.nature_c.as_str() {
                                                            "1" => "特攻: ↑ ".to_string(),
                                                            "0" => "".to_string(),
                                                            "-1" => "特攻: ↓ ".to_string(),
                                                            _ => "".to_string(),
                                                        }
                                                    },
                                                    {
                                                        match pk_data.nature_d.as_str() {
                                                            "1" => "特防: ↑ ".to_string(),
                                                            "0" => "".to_string(),
                                                            "-1" => "特防: ↓ ".to_string(),
                                                            _ => "".to_string(),
                                                        }
                                                    },
                                                    {
                                                        match pk_data.nature_s.as_str() {
                                                            "1" => "素早さ: ↑ ".to_string(),
                                                            "0" => "".to_string(),
                                                            "-1" => "素早さ: ↓ ".to_string(),
                                                            _ => "".to_string(),
                                                        }
                                                    }
                                                )
                                            }
                                            </p>
                                        </div>
                                        <div class={ search_pk_div_style.clone() }>
                                            <p class={ search_pk_label_p_style.clone() }>{ "技:" }</p>
                                            <p class={ search_move_p_style.clone() }>{ format!("{}　", pk_data.move_1.name.clone()) }</p>
                                            <p class={ search_move_p_style.clone() }>{ format!("{}　", pk_data.move_2.name.clone()) }</p>
                                            <p class={ search_move_p_style.clone() }>{ format!("{}　", pk_data.move_3.name.clone()) }</p>
                                            <p class={ search_move_p_style.clone() }>{ format!("{}", pk_data.move_4.name.clone()) }</p>
                                        </div>

                                        <div class={ search_pk_div_style.clone() }>
                                            <p class={ search_pk_label_p_style.clone() }>{ "実数値:" }</p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("HP: {:.0}",
                                                    (( (pk_data.base_stat_h.clone()).parse::<f64>().unwrap() * 2.0 + (pk_data.individual_values_h.clone()).parse::<f64>().unwrap() + ( (pk_data.effort_values_h.clone()).parse::<f64>().unwrap() / 4.0 ).floor() ) / 2.0).floor() + 60.0
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("攻撃: {:.0}",
                                                    ( ((((pk_data.base_stat_a.clone()).parse::<f64>().unwrap() * 2.0 + (pk_data.individual_values_a.clone()).parse::<f64>().unwrap() + ((pk_data.effort_values_a.clone()).parse::<f64>().unwrap() / 4.0).floor()) / 2.0).floor() + 5.0) * (1.0 + ((pk_data.nature_a.clone()).parse::<f64>().unwrap() * 0.1)) ).floor()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("防御: {:.0}",
                                                    ( ((((pk_data.base_stat_b.clone()).parse::<f64>().unwrap() * 2.0 + (pk_data.individual_values_b.clone()).parse::<f64>().unwrap() + ((pk_data.effort_values_b.clone()).parse::<f64>().unwrap() / 4.0).floor()) / 2.0).floor() + 5.0) * (1.0 + ((pk_data.nature_b.clone()).parse::<f64>().unwrap() * 0.1)) ).floor()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("特攻: {:.0}",
                                                    ( ((((pk_data.base_stat_c.clone()).parse::<f64>().unwrap() * 2.0 + (pk_data.individual_values_c.clone()).parse::<f64>().unwrap() + ((pk_data.effort_values_c.clone()).parse::<f64>().unwrap() / 4.0).floor()) / 2.0).floor() + 5.0) * (1.0 + ((pk_data.nature_c.clone()).parse::<f64>().unwrap() * 0.1)) ).floor()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("特防: {:.0}",
                                                    ( ((((pk_data.base_stat_d.clone()).parse::<f64>().unwrap() * 2.0 + (pk_data.individual_values_d.clone()).parse::<f64>().unwrap() + ((pk_data.effort_values_d.clone()).parse::<f64>().unwrap() / 4.0).floor()) / 2.0).floor() + 5.0) * (1.0 + ((pk_data.nature_d.clone()).parse::<f64>().unwrap() * 0.1)) ).floor()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("素早さ: {:.0}",
                                                    ( ((((pk_data.base_stat_s.clone()).parse::<f64>().unwrap() * 2.0 + (pk_data.individual_values_s.clone()).parse::<f64>().unwrap() + ((pk_data.effort_values_s.clone()).parse::<f64>().unwrap() / 4.0).floor()) / 2.0).floor() + 5.0) * (1.0 + ((pk_data.nature_s.clone()).parse::<f64>().unwrap() * 0.1)) ).floor()
                                                )}
                                            </p>
                                        </div>

                                        <div class={ search_pk_div_style.clone() }>
                                            <p class={ search_pk_label_p_style.clone() }>{ "個体値:" }</p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("HP: {}",
                                                    pk_data.individual_values_h.clone()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("攻撃: {}",
                                                    pk_data.individual_values_a.clone()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("防御: {}",
                                                    pk_data.individual_values_b.clone()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("特攻: {}",
                                                    pk_data.individual_values_c.clone()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("特防: {}",
                                                    pk_data.individual_values_d.clone()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("素早さ: {}",
                                                    pk_data.individual_values_s.clone()
                                                )}
                                            </p>
                                        </div>

                                        <div class={ search_pk_div_style.clone() }>
                                            <p class={ search_pk_label_p_style.clone() }>{ "努力値:" }</p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("HP: {}",
                                                    pk_data.effort_values_h.clone()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("攻撃: {}",
                                                    pk_data.effort_values_a.clone()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("防御: {}",
                                                    pk_data.effort_values_b.clone()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("特攻: {}",
                                                    pk_data.effort_values_c.clone()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("特防: {}",
                                                    pk_data.effort_values_d.clone()
                                                )}
                                            </p>
                                            <p class={ search_pk_value_p_style.clone() }>
                                                {
                                                    format!("素早さ: {}",
                                                    pk_data.effort_values_s.clone()
                                                )}
                                            </p>
                                        </div>
                                        <br />
                                        <br />
                                        <br />
                                        <br />
                                    </li>
                                }).collect::<Html>()}
                            </ul>
                        </>
                        }
                } else {
                    html! {}
                }
            }
        </>
    }
}
