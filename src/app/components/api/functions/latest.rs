use serde::{de::DeserializeOwned, Deserialize, Serialize};
use stylist::style;
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

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
enum Error {
    RequestError,
    DeserializeError,
    // etc.
}

//propsの構造体
#[derive(Properties, PartialEq)]
pub struct UrlProps {
    pub name: String,
}

#[function_component(Latest)]
pub fn latest() -> Html {
    //情報の取得
    let state = yew_hooks::prelude::use_async(async move {
        fetch::<JsonParce>(
            "https://pokemon-ga-api.yukiosada.work/latest?higher=1&lower=10".to_string(),
        )
        .await
    });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            // You can trigger to run in callback or use_effect.
            state.run();
        })
    };

    let latest_click_style = style! {
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
            margin:0 auto 10px;
            display:block;
        "#
    }
    .expect("Failed to mount style");

    let latest_div_style = style! {
        r#"
            background-color: #000;
            color: #fff;
            padding: 10px;
        "#
    }
    .expect("Failed to mount style");
    let latest_space_style = style!(
        r#"
            width: 100%;
            height: 100px;
            background-color: #000;
        "#
    )
    .expect("Failed to mount style");
    let latest_url_style = style!(
        r#"
            color: #fff;
            font-size: 25px;
            text-align: center;
            margin-bottom: 60px;
            font-family: 'Itim', cursive;
        "#
    )
    .expect("Failed to mount style");
    let latest_latest_generation_style = style!(
        r#"
            color: #fff;
            font-size: 27px;
            text-align: center;
            margin-bottom: 15px;
        "#
    )
    .expect("Failed to mount style");
    let latest_latest_generation_space_style = style!(
        r#"
            width: 100%;
            height: 60px;
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
    let info_style = style!(
        r#"
            color: white;
            margin: 30px 0;
        "#
    )
    .expect("Failed to mount style");

    html! {
        <div class={latest_div_style}>
            <h1 class={latest_url_style}>
                { "https://pokemon-ga-api.yukiosada.work/latest?higher=1&lower=10" }
            </h1>
            <button onclick={onclick} disabled={state.loading} style="cursor:pointer" class={latest_click_style}>{ "Click" }</button>
            <p class={ info_style }>{ "※以下の情報は、上記のURLから取得できる情報の一部であることを理解の上、ご利用ください。" }</p>
            {
                if state.loading {
                    html! { <p> { "Loading, wait a sec..." }</p> }
                } else {
                    html! {}
                }
            }
            <div class={latest_space_style}></div>
            <p>
                {
                    if let Some(error) = &state.error {
                        match error {
                            Error::DeserializeError => html! { "DeserializeError" },
                            Error::RequestError => html! { "RequestError" },
                        }
                    } else {
                        html! {}
                    }
                }
            </p>
            {
                if let Some(pokemon) = &state.data {
                    html! {
                        <>
                            <h1 class={latest_latest_generation_style.clone()}>{ "計算が終了している最新の世代" }</h1>
                            <h1 class={latest_latest_generation_style.clone()}>{ &pokemon.generation }</h1>
                            <div class={latest_latest_generation_space_style.clone()}></div>
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
    }
}
