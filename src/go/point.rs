use yew::prelude::*;

use crate::go::*;

// 星位
const STAR_POINTS_9: [(i8, i8); 4] = [(2, 2), (2, 6), (6, 2), (6, 6)];
const STAR_POINTS_19: [(i8, i8); 9] = [(3, 3), (3, 9), (3, 15), (9, 3), (9, 9), (9, 15), (15, 3), (15, 9), (15, 15)];

#[derive(Properties, PartialEq)]
pub struct Props {
    pub pos: (i8, i8),
    pub grid_size: i8,
    pub width: i32,
}

#[function_component(Point)]
pub fn point(props: &Props) -> Html {
    let full_width_style = format!("width: {}px; height: {}px;", props.width, props.width);

    let mut cross_class = classes!("point");
    if props.pos.0 == 0 {
        cross_class = classes!("point", "edge-top");
        if props.pos.1 == 0 {
            cross_class = classes!("point", "corner-top-left");
        }
        if props.pos.1 == props.grid_size - 1 {
            cross_class = classes!("point", "corner-top-right");
        }
    } else if props.pos.0 == props.grid_size - 1 {
        cross_class = classes!("point", "edge-bottom");
        if props.pos.1 == 0 {
            cross_class = classes!("point", "corner-bottom-left");
        }
        if props.pos.1 == props.grid_size - 1 {
            cross_class = classes!("point", "corner-bottom-right");
        }
    } else if props.pos.1 == 0 {
        cross_class = classes!("point", "edge-left");
    } else if props.pos.1 == props.grid_size - 1 {
        cross_class = classes!("point", "edge-right");
    }

    let stone = use_state_eq(|| { Stone::Chi });
    let stone_setter = stone.setter();

    let game_record: GameRecord = use_context::<GameRecord>().expect("no ctx found");
    let record_setter: UseStateSetter<GameRecord> = use_context::<UseStateSetter<GameRecord>>().expect("no ctx found");

    let onclick = use_callback(|_, _| {
        let mut new_record = game_record.clone();
        stone_setter.set(new_record.next);

        new_record.step = new_record.step + 1;
        let stone = new_record.next;
        new_record.next = if stone == Stone::Black {
            Stone::White
        } else {
            Stone::Black
        };

        // 提子
        let black = new_record.capture.get(&stone);
        new_record.steps.push(Step {
            stone,
            step: 1,
            pos: props.pos,
        });


        record_setter.set(new_record);
    }, ());

    html! {
        <>
            <div class={cross_class} style={full_width_style} {onclick}>
                if props.grid_size == 9 && STAR_POINTS_9.contains(&props.pos) {
                    <div class="star"></div>
                }
                if props.grid_size == 19 && STAR_POINTS_19.contains(&props.pos) {
                    <div class="star"></div>
                }
                if Stone::Chi != stone {
                    <Piece state={stone}/>
                }
            </div>
        </>
    }
}
