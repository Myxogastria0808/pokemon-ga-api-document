use card_field::card_filed::CardField;
use yew::prelude::*;

mod card_field;

#[function_component(Top)]
pub fn top() -> Html {
    html! {
        <>
            <CardField />
        </>
    }
}
