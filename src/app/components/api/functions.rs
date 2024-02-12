use archive::Archive;
use latest::Latest;
use path_param::PathParam;
use query::Query;
use real_number::RealNumber;
use search_pokemon::SearchPokemon;
use stylist::style;
use yew::prelude::*;

mod archive;
mod latest;
mod path_param;
mod query;
mod real_number;
mod search_pokemon;

#[derive(Properties, PartialEq)]
pub struct DesignContent {
    pub url: String,
    pub path: String,
    pub title: String,
}
#[function_component(Design)]
fn design(props: &DesignContent) -> Html {
    let design_div_style = style!(
        r#"
            background-color:#000;
        "#
    )
    .expect("Failed to mount style");

    let design_a_style = style!(
        r#"
            border-radius:10px;
            box-shadow: 0px 0px 20px #ff0 ;
            font-size:30px;
            color:#fff;
            text-decoration:none;
            margin:0 auto;
            display:block;
            width:fit-content;
            padding: 10px;
            font-family: 'Itim', cursive;
        "#
    )
    .expect("Failed to mount style");

    let design_h1_style = style!(
        r#"
            font-size:35px;
            color:#fff;
            font-family: 'Itim', cursive;
            padding:50px 0 50px;
            text-align:center;
        "#
    )
    .expect("Failed to mount style");

    html! {
        <div class={design_div_style}>
            <a class={design_a_style} style="hover:cursor: pointer;" href={ props.url.clone() } target="_blank" rel="noopener noreferrer">{ props.path.clone() }</a>
            <h1 class={design_h1_style}>{ props.title.clone() }</h1>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Api {
    pub api: String,
}
#[function_component(Functions)]
pub fn fnctions(props: &Api) -> Html {
    let api_types: String = props.api.clone();
    html! {
        if api_types == *"archive" {
                <Design
                    url={"https://pokemon-ga-api.yukiosada.work/docs#/default/generation_generation__generation__get"}
                    path={"/generation/{integer}"}
                    title={"過去の世代"}
                />
                <PathParam
                    title={"世代数を指定する"}
                    path={"/generation/{integer}"}
                    content={"世代数の指定は、必ずしてください。第1世代から計算が終了している最新の世代の間で、どれか1つの世代を指定してください。"}
                />
                <Query />
                <Archive />
            } else if api_types == *"latest" {
                <Design
                    url={"https://pokemon-ga-api.yukiosada.work/docs#/default/latest_generation_latest_get"}
                    path={"/latest"}
                    title={ "最新の世代" }
                />
                <Query />
                <Latest />
            } else if api_types == *"search_pokemon" {
                <Design
                    url={"https://pokemon-ga-api.yukiosada.work/docs#/default/search_pokemon_pokemon__pokemon__get"}
                    path={"/pokemon/{string}"}
                    title={ "Pokémonの検索" }
                />
                <PathParam
                    title={"ポケモンの名前を指定する"}
                    path={"/pokemon/{string}"}
                    content={"ポケモンの名前を指定してください。その際、ポケモンの名前は全角カタカナで指定してください。"}
                />
                <RealNumber />
                <SearchPokemon />
            } else {
                <p>{ "Function Props is missing!" }</p>
        }
    }
}
