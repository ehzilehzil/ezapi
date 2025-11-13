#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// rustup target add wasm32-unknown-unknown
// cargo build --target wasm32-unknown-unknown --release
// 결과물이 ./target/wasm32-unknown-unknown/release/[이름].wasm
// 이를 ./lib로 복사