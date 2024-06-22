//! Cross-Program invocations to the Metaplex Token Metadata Program.

use mpl_token_metadata::{instructions::{CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3InstructionArgs, MintNewEditionFromMasterEditionViaTokenInstructionArgs}, types::{DataV2, MintNewEditionFromMasterEditionViaTokenArgs}};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};

use crate::{Create, Metadata, NautilusAccountInfo, NautilusMut, NautilusSigner};

/// Creates a Metadata account with the Token Metadata Program.
#[allow(clippy::boxed_local)]
#[allow(clippy::too_many_arguments)]
pub fn create_metadata_v3<'a>(
    _token_metadata_program_id: &Pubkey,
    metadata: Create<'a, Metadata<'a>>,
    title: String,
    symbol: String,
    uri: String,
    mint: impl NautilusAccountInfo<'a>,
    mint_authority: impl NautilusSigner<'a>,
    update_authority: impl NautilusAccountInfo<'a>,
    payer: impl NautilusSigner<'a>,
    rent: Box<AccountInfo<'a>>,
    system_program: Box<AccountInfo<'a>>
) -> ProgramResult {
    let ctx = mpl_token_metadata::instructions::CreateMetadataAccountV3 {
        metadata: *metadata.account_info().key,
        mint: *mint.account_info().key,
        mint_authority: *mint_authority.account_info().key,
        payer: *payer.account_info().key,
        update_authority: (*update_authority.account_info().key, true),
        rent: Some(*rent.key),
        system_program: *system_program.key
    };

    invoke(
        &mpl_token_metadata::instructions::CreateMetadataAccountV3::instruction(
            &ctx,
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: title,
                    symbol,
                    uri,
                    uses: None,
                    creators: None,
                    collection: None,
                    seller_fee_basis_points: 0,
                },
                is_mutable: true,
                collection_details: None,
            }
        ),
        &[
            *metadata.account_info(),
            *mint.account_info(),
            *mint_authority.account_info(),
            *payer.account_info(),
            *rent,
        ],
    )
}

/// Creates a MasterEdition account with the Token Metadata Program.
#[allow(clippy::boxed_local)]
#[allow(clippy::too_many_arguments)]
pub fn create_master_edition_v3<'a>(
    _token_metadata_program_id: &Pubkey,
    edition: impl NautilusMut<'a>,
    mint: impl NautilusAccountInfo<'a>,
    metadata: impl NautilusAccountInfo<'a>,
    update_authority: impl NautilusSigner<'a>,
    mint_authority: impl NautilusSigner<'a>,
    payer: impl NautilusSigner<'a>,
    rent: Box<AccountInfo<'a>>,
    max_supply: Option<u64>,
    token_program: Box<AccountInfo<'a>>,
    system_program: Box<AccountInfo<'a>>
) -> ProgramResult {
    let ctx = mpl_token_metadata::instructions::CreateMasterEditionV3 {
        edition: *edition.account_info().key,
        metadata: *metadata.account_info().key,
        mint: *mint.account_info().key,
        mint_authority: *mint_authority.account_info().key,
        payer: *payer.account_info().key,
        update_authority: *update_authority.account_info().key,
        token_program: *token_program.key,
        rent: Some(*rent.key),
        system_program: *system_program.key
    };

    invoke(
        &mpl_token_metadata::instructions::CreateMasterEditionV3::instruction(
            &ctx,
            CreateMasterEditionV3InstructionArgs {
                max_supply
            }
        ),
        &[
            *edition.account_info(),
            *metadata.account_info(),
            *mint.account_info(),
            *mint_authority.account_info(),
            *payer.account_info(),
            *rent,
        ],
    )
}

/// Mints a new Edition from a MasterEdition.
#[allow(clippy::boxed_local)]
#[allow(clippy::too_many_arguments)]
pub fn mint_edition_from_master_edition<'a>(
    _token_metadata_program_id: &Pubkey,
    mint: impl NautilusMut<'a>,
    metadata: impl NautilusAccountInfo<'a>,
    edition: impl NautilusMut<'a>,
    master_edition: impl NautilusAccountInfo<'a>,
    master_edition_mint: impl NautilusAccountInfo<'a>,
    master_edition_metadata: impl NautilusAccountInfo<'a>,
    to: impl NautilusMut<'a>,
    to_authority: impl NautilusSigner<'a>,
    mint_authority: impl NautilusSigner<'a>,
    update_authority: impl NautilusSigner<'a>,
    payer: impl NautilusSigner<'a>,
    rent: Box<AccountInfo<'a>>,
    token_program: Box<AccountInfo<'a>>,
    system_program: Box<AccountInfo<'a>>,
    edition_val: u64,
) -> ProgramResult {
    let ctx = mpl_token_metadata::instructions::MintNewEditionFromMasterEditionViaToken {
        new_metadata: *metadata.key(),
        new_edition: *edition.key(),
        master_edition: *master_edition.key(),
        new_mint: *mint.key(),
        new_mint_authority: *mint_authority.key(),
        payer: *payer.key(),
        token_account_owner: *to_authority.key(),
        token_account: *to.key(),
        new_metadata_update_authority: *update_authority.key(),
        metadata: *master_edition_metadata.key(),
        edition_mark_pda: *master_edition_mint.key(),
        token_program: *token_program.key,
        rent: Some(*rent.key),
        system_program: *system_program.key
    };

    
    invoke(
        &mpl_token_metadata::instructions::MintNewEditionFromMasterEditionViaToken::instruction(
            &ctx,
            MintNewEditionFromMasterEditionViaTokenInstructionArgs {
                mint_new_edition_from_master_edition_via_token_args: MintNewEditionFromMasterEditionViaTokenArgs {
                    edition: edition_val
                }
            }
        ),
        &[
            *metadata.account_info(),
            *edition.account_info(),
            *master_edition.account_info(),
            *mint.account_info(),
            *mint_authority.account_info(),
            *payer.account_info(),
            *to_authority.account_info(),
            *to.account_info(),
            *update_authority.account_info(),
            *master_edition_metadata.account_info(),
            *master_edition_mint.account_info(),
            *rent,
        ],
    )
}
