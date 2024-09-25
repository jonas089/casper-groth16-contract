#![no_main]
use casper_contract::contract_api::{
    builtins::altbn128::{alt_bn128_pairing, Pair},
    storage,
};
use casper_types::{addressable_entity::NamedKeys, EntryPoints};
mod example;
use example::example_proof;
extern crate alloc;
use alloc::string::ToString;

#[no_mangle]
pub extern "C" fn call() {
    let proof = example_proof();
    let pair_1 = Pair {
        ax: proof.a_neg_x.into(),
        ay: proof.a_neg_y.into(),
        bax: proof.b_x_1.into(),
        bay: proof.b_y_1.into(),
        bbx: proof.b_x_2.into(),
        bby: proof.b_y_2.into(),
    };

    let pair_2 = Pair {
        ax: proof.alpha_x.into(),
        ay: proof.alpha_y.into(),
        bax: proof.beta_x_1.into(),
        bay: proof.beta_y_1.into(),
        bbx: proof.beta_x_2.into(),
        bby: proof.beta_y_2.into(),
    };

    let pair_3 = Pair {
        ax: proof.vk_x.into(),
        ay: proof.vk_y.into(),
        bax: proof.gamma_x_1.into(),
        bay: proof.gamma_y_1.into(),
        bbx: proof.gamma_x_2.into(),
        bby: proof.gamma_y_2.into(),
    };

    let pair_4 = Pair {
        ax: proof.c_x.into(),
        ay: proof.c_y.into(),
        bax: proof.delta_x_1.into(),
        bay: proof.delta_y_1.into(),
        bbx: proof.delta_x_2.into(),
        bby: proof.delta_y_2.into(),
    };

    let pairs = [pair_1, pair_2, pair_3, pair_4];
    let result = alt_bn128_pairing(&pairs).unwrap();
    assert!(result);

    let entry_points = EntryPoints::new();
    let named_keys = NamedKeys::new();
    storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("contract-hash".to_string()),
        Some("contract-package-hash".to_string()),
        None,
    );
}
