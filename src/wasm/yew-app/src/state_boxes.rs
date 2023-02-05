use yew::prelude::*;
use nalgebra::Complex;
pub fn discrete_state_box(state: i32) -> Html{
    html!{
        <div class="zero-state-box">
            {"| "}{state}{" >"}
        </div>
    }
}

pub fn quantized_state_box(state: Complex<f32>, a_idx: i32, b_idx: i32) -> Html{
    html!{
        <div class="zero-state-box">
            {"| "}{"ψ"}{" >"}
        </div>
    }
}