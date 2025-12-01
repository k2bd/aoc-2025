use pyo3_stub_gen::Result;
use rs::stub_info;

fn main() -> Result<()> {
    let stub = stub_info()?;
    stub.generate()?;
    Ok(())
}
