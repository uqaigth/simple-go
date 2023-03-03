use yew::prelude::*;

use crate::go::game::Stone;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub state: Stone,
}

#[function_component(Piece)]
pub fn piece(props: &Props) -> Html {
    let piece_class = match props.state {
        Stone::Black => { classes!("piece-black") }
        Stone::White => { classes!("piece-white") }
        Stone::Chi => { classes!() }
    };

    html! {
        <div class={piece_class}></div>
    }
}

