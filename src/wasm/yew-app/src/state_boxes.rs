use nalgebra::{UnitVector2, Complex};
use yew::prelude::*;
pub fn discrete_state_box(state: i32) -> Html{
    html!{
        <div class="zero-state-box">
            {"| "}{state}{" >"}
        </div>
    }
}

pub fn quantized_state_box(state: UnitVector2<Complex<f32>>, a_idx: i32, b_idx: i32) -> Html{
    html!{
        <div class="zero-state-box">
            {"| "}{"ψ"}<sub>{format!("{}{}", a_idx,b_idx)}</sub>{" >"}
        </div>
    }
}