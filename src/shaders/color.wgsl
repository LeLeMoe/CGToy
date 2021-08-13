[[stage(vertex)]]
fn main(
    [[builtin(vertex_index)]] in_vertec_index: u32,
) -> [[builtin(position)]] vec4<f32> {
    let x = f32(1 - i32(in_vertec_index)) * 0.5;
    let y = f32(i32(in_vertec_index & 1u) * 2 - 1) * 0.5;
    return vec4<f32>(x, y, 0.0, 1.0);
}

[[stage(fragment)]]
fn main() -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}