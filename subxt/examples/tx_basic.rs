#[subxt::subxt(runtime_metadata_path = "../TRIMMED_METADATA.scale")]
pub mod runtime {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let call = runtime::Call::System(runtime::system::Call::remark { remark: vec![] });
    let sudo_set_balance = runtime::tx().sudo().sudo(call);
    Ok(())
}
