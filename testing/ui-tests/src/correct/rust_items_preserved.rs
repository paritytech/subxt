#[subxt::subxt(runtime_metadata_path = "../../../../artifacts/polkadot_metadata_tiny.scale")]
pub mod node_runtime {
    pub struct SomeStruct;
    pub enum SomeEnum {
        A,
        B,
    }
    pub trait SomeTrait {
        fn some_func(&self) -> u32;
    }
    impl SomeTrait for SomeStruct {
        fn some_func(&self) -> u32 {
            1
        }
    }
    impl SomeTrait for SomeEnum {
        fn some_func(&self) -> u32 {
            2
        }
    }
}

fn main() {
    use node_runtime::SomeTrait;

    let unit = node_runtime::SomeStruct;
    assert_eq!(unit.some_func(), 1);
    let enumeration = node_runtime::SomeEnum::A;
    assert_eq!(enumeration.some_func(), 2);
}
