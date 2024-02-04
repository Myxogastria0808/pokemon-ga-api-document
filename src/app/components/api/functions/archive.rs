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
struct Party {
    name: String,
    form_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Ranking {
    rank: usize,
    score: f64,
    party: Vec<Party>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct JsonParce {
    generation: usize,
    ranking: Vec<Ranking>,
}

// エラーの種類に関して、enumで定義している
#[derive(Clone, Debug, PartialEq)]
enum Error {
    RequestError,
    DeserializeError,
    // etc.
}

#[function_component(Archive)]
pub fn archive() -> Html {
    let generation = use_state(|| 1);

    let onchange = {
        let generation = generation.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            generation.set(input.value().parse().expect("This value is not integer"));
        })
    };

    //情報の取得
    let generation_clone = generation.clone();
    let pre_state = yew_hooks::prelude::use_async(async {
        fetch::<JsonParce>(
            "https://pokemon-ga-api.yukiosada.work/latest?higher=1&lower=1".to_string(),
        )
        .await
    });
    let now_state = yew_hooks::prelude::use_async(async {
        fetch::<JsonParce>(
            "https://pokemon-ga-api.yukiosada.work/latest?higher=1&lower=1".to_string(),
        )
        .await
    });
    let state = yew_hooks::prelude::use_async(async move {
        fetch::<JsonParce>(format!(
            "https://pokemon-ga-api.yukiosada.work/generation/{}?higher=1&lower=10",
            *generation_clone
        ))
        .await
    });

    let onclick = {
        // let pre_state = pre_state.clone();
        let state = state.clone();
        let now_state = now_state.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // You can trigger to run in callback or use_effect.
            // pre_state.run();
            state.run();
            now_state.run();
        })
    };

    let pre_state_clone = pre_state.clone();
    use_effect_with((), move |_| {
        pre_state.run();
    });
    let pre_state = pre_state_clone.clone();

    let search_space_style = style!(
        r#"
            width: 100%;
            height: 100px;
            background-color: #000;
        "#
    )
    .expect("Failed to mount style");
    let archive_div_style = style!(
        r#"
            display:flex;
            gap:20px;
            max-width:90%;
            margin:0 auto;
            flex-wrap:wrap;
            justify-content:center;
        "#
    )
    .expect("Failed to mount style");
    let archive_input_style = style!(
        r#"
            width: 300px;
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
            margin-bottom:20px;
        "#
    )
    .expect("Failed to mount style");
    let archive_button_style = style!(
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
            margin-bottom:20px;
        "#
    )
    .expect("Failed to mount style");
    let archive_url_style = style!(
        r#"
            color: #fff;
            font-size: 25px;
            text-align: center;
            margin-bottom: 30px;
            font-family: 'Itim', cursive;
        "#
    )
    .expect("Failed to mount style");
    let archive_latest_generation_style = style!(
        r#"
            color: #fff;
            font-size: 27px;
            text-align: center;
            margin-bottom: 15px;
        "#
    )
    .expect("Failed to mount style");
    let latest_ranking_style = style!(
        r#"
            color: #fff;
            font-size: 27px;
            text-align: center;
            margin-bottom: 15px;
        "#
    )
    .expect("Failed to mount style");
    let latest_ranking_space_style = style!(
        r#"
            width: 100%;
            height: 30px;
        "#
    )
    .expect("Failed to mount style");
    let latest_li_style = style!(
        r#"
            list-style: none;
        "#
    )
    .expect("Failed to mount style");
    let latest_rank_style = style!(
        r#"
            position: absolute;
            top: 0;
            left: 50%;

            color: #fff;
            font-size: 25px;
            padding: 0 1em;
            margin: 0;
            background-color: #000;
            transform: translateY(-50%) translateX(-50%);
        "#
    )
    .expect("Failed to mount style");
    let latest_score_style = style!(
        r#"
            font-size: 18px;
        "#
    )
    .expect("Failed to mount style");
    let latest_rank_party_title_style = style!(
        r#"
            text-align: center;
            font-size: 22px;
        "#
    )
    .expect("Failed to mount style");
    let latest_rank_party_style = style!(
        r#"
            text-align: center;
            font-size: 20px;
        "#
    )
    .expect("Failed to mount style");
    let latest_rank_party_div_style = style!(
        r#"
            height: 40px;
            width: 100%;
        "#
    )
    .expect("Failed to mount style");
    let caption_box_style = style!(
        r#"
            position: relative;
            width: 90%;
            margin-top: 1em;
            margin-right: auto;
            margin-left: auto;
            border: 1px solid white;
            border-radius: 10px;
            padding: 30px 10px;
            box-sizing:border-box;
        "#
    )
    .expect("Failed to mount style");
    let party_member_style = style!(
        r#"
            display: inline-block;
            margin-left: 13px;
        "#
    )
    .expect("Failed to mount style");
    let string_color_style = style!(
        r#"
            color: #fff;
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
            {
                if let Some(max_generation) = &pre_state.data {
                    if let Some(now_max_generation) = &now_state.data {
                        html! {
                            <>
                                <h1 class={archive_latest_generation_style.clone()}>{ "計算が終了している最新の世代" }</h1>
                                <h1 class={archive_latest_generation_style}>
                                    { (now_max_generation.generation).to_string().clone() }
                                </h1>
                                <h1 class={archive_url_style}>
                                    { format!("https://pokemon-ga-api.yukiosada.work/generation/{}?higher=1&lower=10", *generation.clone()) }
                                </h1>
                                <form>
                                    <div class={archive_div_style}>
                                        <input class={archive_input_style} type="numeric" inputmode="numeric" onchange={onchange} min="1" max={(now_max_generation.generation).to_string()} step="1" placeholder="世代数を入力" />
                                        <button class={archive_button_style} type="submit" style="cursor:pointer" onclick={onclick}>{"Search"}</button>
                                    </div>
                                </form>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                <h1 class={archive_latest_generation_style.clone()}>{ "計算が終了している最新の世代" }</h1>
                                <h1 class={archive_latest_generation_style}>
                                    { (max_generation.generation).to_string().clone() }
                                </h1>
                                <h1 class={archive_url_style}>
                                    { format!("https://pokemon-ga-api.yukiosada.work/generation/{}?higher=1&lower=10", *generation.clone()) }
                                </h1>
                                <form>
                                    <div class={archive_div_style}>
                                        <input class={archive_input_style} type="number" onchange={onchange} id="generation" min="1" max={(max_generation.generation).to_string()} step="1" placeholder="世代数を入力" />
                                        <button class={archive_button_style} type="submit" style="cursor:pointer" onclick={onclick}>{"Search"}</button>
                                    </div>
                                </form>
                            </>
                        }
                    }
                } else {
                    html! { <p>{ "Reponse Error" }</p>}
                }
            }
            {
                html! {<p class={info_style}>{ "※以下の情報は、上記のURLから取得できる情報の一部であることを理解の上、ご利用ください。" }</p>}
            }
            {
                if state.loading {
                    html! {<p>{ "Loading, wait a sec..." }</p> }
                } else {
                    html! {}
                }
            }
            <div class={search_space_style}></div>
            <div class={string_color_style}>
                {
                    if let Some(max_generation) = &pre_state.data {
                        if let Some(error) = &state.error {
                            match error {
                                Error::DeserializeError => html! { <p>{ format!("世代数は、1 から {} の間で入力してください。", max_generation.generation) }</p> },
                                Error::RequestError => html! { <p>{ "RequestError" }</p> },
                            }
                        } else {
                            html! {}
                        }
                    } else { html! { <p>{ "RequestError" }</p> } }
                }
                {
                    if let Some(pokemon) = &state.data {
                        html! {
                            <>
                                <h1 class={latest_ranking_style.clone()}>{ "ランキング" }</h1>
                                <div class={latest_ranking_space_style}></div>

                                <ul>
                                    {(pokemon.ranking).iter().map(|data| html! {
                                        <>
                                            <div class={caption_box_style.clone()}>
                                                <li class={latest_li_style.clone()}>
                                                    <h2 class={latest_rank_style.clone()}>{ format!("{}位", data.rank) }</h2>
                                                    <h2 class={latest_score_style.clone()}>{ format!("得点: {}点", data.score) }</h2>
                                                    <h3 class={latest_rank_party_title_style.clone()}>{"＜パーティ＞"}</h3>
                                                    <h3 class={latest_rank_party_style.clone()}>
                                                        {(data.party).iter().map(|member| html! {
                                                            <span class={party_member_style.clone()}>
                                                                { format!("{}", member.name.clone()) }
                                                                {
                                                                    if member.form_name.clone() == "null" {
                                                                        {"".to_string()}
                                                                    } else {
                                                                        { format!(" {}", member.form_name.clone()) }
                                                                    }
                                                                }
                                                            </span>
                                                        }).collect::<Html>()}
                                                    </h3>
                                                </li>
                                            </div>
                                            <div class={latest_rank_party_div_style.clone()}></div>
                                        </>
                                    }).collect::<Html>()}
                                </ul>
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </>
    }
}
