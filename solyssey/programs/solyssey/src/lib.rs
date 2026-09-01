pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("A3NVCVNxojupfyZvGLfkCsHqZqCQH8GdcVuBo39kh8XQ"); // i'll change this to a custom program ID in the end 

#[program]
pub mod solyssey {
    use super::*;

    // Entry point for all adventurers .. happy hunting !!
    /* pub fn start_here(ctx: Context<>) -> Result<()> {
        
    } */
}
