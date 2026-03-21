use color_eyre::eyre::{Ok, Result};

fn main() -> Result<()> {
    println!("Todo App using Ratatui crate!");
    color_eyre::install()?;

    let terminal = ratatui::init();
    ratatui::restore();

    Ok(())
}
