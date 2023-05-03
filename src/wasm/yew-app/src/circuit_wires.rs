pub fn wire(a_idx: i32, b_idx: i32, start_state: UnitVector2<Complex<f32>>, end_state: UnitVector2<Complex<f32>>) -> Html{
    html!{
        <div class="zero-state-box">
            {"| "}{state}{" >"}
        </div>
    }
}