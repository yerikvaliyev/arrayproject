use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod arrayproject {
    use super::*;

    pub fn create_player_stats(_ctx: Context<CreatePlayerStats>) -> Result<()> {
        Ok(())
    }

    pub fn calculate_reward(ctx: Context<CalculateReward>, placement: u64, kills: u64, _identifier: u64) -> Result<()> {
        let stat = Stats {
            // id: player.identity,
            placement: placement as u8,
            kills: kills as u8,
            // survival_duration: game.timestamp, //change later not implemented yet
            // reward: reward,
        };
        msg!("Stat: {:?}", stat);

        let stats = &mut ctx.accounts.players_stats.load_mut()?;
        msg!("PlayersStats pubkey: {}", ctx.accounts.players_stats.key());
        msg!("Before: {}", stats.counter);
        msg!("PlayersStats before: {:?}", stats.players[0]);
        stats.append(stat);
        msg!("After: {}", stats.counter);
        msg!("PlayersStats after: {:?}", stats.players);

        Ok(())
    }


}

#[derive(Account)]
pub struct CalculateReward<'info> {
    #[account(mut)]
    pub players_stats: AccountLoader<'info, PlayersStats>,
}

#[derive(Accounts)]
pub struct CreatePlayerStats<'info> {
    #[account(zero)]
    player_stats: AccountLoader<'info, PlayersStats>,
}

#[account(zero_copy)]
pub struct PlayersStats {
    pub players: [Stats; 32], //4 + 1472
    pub counter: u64, //1
}

impl PlayersStats {
    fn append(&mut self, stat: Stats) {
        self.players[PlayersStats::index_of(self.counter)] = stat;
        self.counter = self.counter + 1;
    }

    fn index_of(counter: u64) -> usize {
        std::convert::TryInto::try_into(counter).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct Stats { //(32 + 1 + 1 + 8) * 32 = 1472
    // pub id: Pubkey, //32
    pub placement: u8, //1
    pub kills: u8, //1
    // pub survival_duration: u32, //4
    // pub reward: u64, //8
}