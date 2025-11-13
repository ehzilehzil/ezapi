// rustup target add wasm32-unknown-unknown
// cargo build --target wasm32-unknown-unknown --release
// 결과물이 ./target/wasm32-unknown-unknown/release/[이름].wasm
// 이를 ./lib로 복사

use linfa::traits::Transformer;
use linfa_clustering::KMeans;
use ndarray::Array2;


struct Data {
    name: String,
    addr: String,
    lng: f64,
    lat: f64,
    g: String,
}

#[unsafe(no_mangle)]
pub fn get_kmeans(data: &mut Vec<Data>) {

}

fn test() {
    
}



#[cfg(test)]
mod test {
    // use super::*;

    #[test]
    fn test() {
        super::test();
    }
}