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
            color: white;
            margin: 30px 0;
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
                    <button class={search_button_style} style="cursor:pointer" type="submit" onclick={onclick} disabled={state.loading}>{"search"}</button>
                </div>
            </form>
            <p class={ info_style }>{ "※以下の情報は、上記のURLから取得できる情報の一部であることを理解の上、ご利用ください。" }</p>
            {
                if state.loading {
                    html! { <p>{ "Loading, wait a sec..." }</p> }
                } else {
                    html! {}
                }
            }
            <div class={search_space_style}></div>
            {
                if let Some(pokemon) = &state.data {
                    html! {
                        <>
                            <ul>
                                {(pokemon.data).iter().map(|pk_data| html! {
                                    <li>
                                        <p>{ "pokedex_no: " }<b>{ pk_data.pokedex_no.clone() }</b><b>{ pk_data.form_no.clone() }</b></p>
                                        <p>{ "name: " }<b>{ pk_data.name.clone() }</b></p>
                                        <p>{ "form_name: " }<b>{ pk_data.form_name.clone() }</b></p>
                                        <p>{ "type_1: " }<b>{ pk_data.type_1.clone() }</b></p>
                                        <p>{ "type_2: " }<b>{ pk_data.type_2.clone() }</b></p>
                                        <p>{ "base_stat_h: " }<b>{ pk_data.base_stat_h.clone() }</b></p>
                                        <p>{ "base_stat_a: " }<b>{ pk_data.base_stat_a.clone() }</b></p>
                                        <p>{ "base_stat_b: " }<b>{ pk_data.base_stat_b.clone() }</b></p>
                                        <p>{ "base_stat_c: " }<b>{ pk_data.base_stat_c.clone() }</b></p>
                                        <p>{ "base_stat_d: " }<b>{ pk_data.base_stat_d.clone() }</b></p>
                                        <p>{ "base_stat_s: " }<b>{ pk_data.base_stat_s.clone() }</b></p>
                                        <p>{ "base_stat_all: " }<b>{ pk_data.base_stat_all.clone() }</b></p>
                                        <p>{ "individual_values_h: " }<b>{ pk_data.individual_values_h.clone() }</b></p>
                                        <p>{ "individual_values_a: " }<b>{ pk_data.individual_values_a.clone() }</b></p>
                                        <p>{ "individual_values_b: " }<b>{ pk_data.individual_values_b.clone() }</b></p>
                                        <p>{ "individual_values_c: " }<b>{ pk_data.individual_values_c.clone() }</b></p>
                                        <p>{ "individual_values_d: " }<b>{ pk_data.individual_values_d.clone() }</b></p>
                                        <p>{ "individual_values_s: " }<b>{ pk_data.individual_values_s.clone() }</b></p>
                                        <p>{ "effort_values_h: " }<b>{ pk_data.effort_values_h.clone() }</b></p>
                                        <p>{ "effort_values_a: " }<b>{ pk_data.effort_values_a.clone() }</b></p>
                                        <p>{ "effort_values_b: " }<b>{ pk_data.effort_values_b.clone() }</b></p>
                                        <p>{ "effort_values_c: " }<b>{ pk_data.effort_values_c.clone() }</b></p>
                                        <p>{ "effort_values_d: " }<b>{ pk_data.effort_values_d.clone() }</b></p>
                                        <p>{ "effort_values_s: " }<b>{ pk_data.effort_values_s.clone() }</b></p>
                                        <p>{ "nature_a: " }<b>{ pk_data.nature_a.clone() }</b></p>
                                        <p>{ "nature_b: " }<b>{ pk_data.nature_b.clone() }</b></p>
                                        <p>{ "nature_c: " }<b>{ pk_data.nature_c.clone() }</b></p>
                                        <p>{ "nature_d: " }<b>{ pk_data.nature_d.clone() }</b></p>
                                        <p>{ "nature_s: " }<b>{ pk_data.nature_s.clone() }</b></p>
                                        <p>{ "item: " }<b>{ pk_data.item.clone() }</b></p>
                                        <p>{ "ability: " }<b>{ pk_data.ability.clone() }</b></p>
                                        <p>{ "tera_type: " }<b>{ pk_data.tera_type.clone() }</b></p>
                                        <br />
                                        <br />
                                        <p>{ "move_1 name: " }<b>{ pk_data.move_1.name.clone() }</b></p>
                                        <br />
                                        <p>{ "move_2 name: " }<b>{ pk_data.move_2.name.clone() }</b></p>
                                        <br />
                                        <p>{ "move_3 name: " }<b>{ pk_data.move_3.name.clone() }</b></p>
                                        <br />
                                        <p>{ "move_4 name: " }<b>{ pk_data.move_4.name.clone() }</b></p>
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
            <p>
                {
                    if let Some(error) = &state.error {
                        match error {
                            Error::DeserializeError => html! { <p>{ "登録されているポケモンの名前を正確に入力してください。" }</p> },
                            Error::RequestError => html! { "RequestError" },
                        }
                    } else {
                        html! {}
                    }
                }
            </p>
        </>
    }
}
