mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const SKU_MASK: u64 = 0xFFFFFFFC00000000;
const ID_MASK: u64 = 0x3FFFFFFFF;

#[wasm_bindgen]
pub struct WasmItem(u64);

#[wasm_bindgen]
impl WasmItem {
    pub fn new(s: String) -> Option<WasmItem> {
        let mut parts = s.split('-');
        let sku = parts.next()?.parse::<u64>().ok()?;
        let item_id = parts.next()?.parse::<u64>().ok()?;
        Some(WasmItem(item_id | (sku << 34)))
    }

    pub fn toString(&self) -> String {
        format!("{}-{:<010}", self.sku(), self.item_id())
    }

    pub fn sku(&self) -> u64 {
        (self.0 & SKU_MASK) >> 34
    }

    pub fn item_id(&self) -> u64 {
        self.0 & ID_MASK
    }
}

#[test]
fn test_item() {
    assert_eq!(
        "10-0000000001",
        WasmItem::new("10-0000000001".to_owned())
            .unwrap()
            .toString()
    );

    assert_eq!(
        "999999-9999999999",
        WasmItem::new("999999-9999999999".to_owned())
            .unwrap()
            .toString()
    );
}
