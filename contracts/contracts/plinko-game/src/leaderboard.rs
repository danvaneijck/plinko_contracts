use crate::state::LeaderboardEntry;
use cosmwasm_std::{Addr, Uint128};

const MAX_LEADERBOARD_SIZE: usize = 100;

/// Update leaderboard with new entry, maintaining sorted order
pub fn update_leaderboard(
    leaderboard: &mut Vec<LeaderboardEntry>,
    player: Addr,
    value: Uint128,
    multiplier: Option<String>,
) {
    // Remove existing entry for this player if present
    leaderboard.retain(|entry| entry.player != player);

    // Create new entry
    let new_entry = LeaderboardEntry {
        player,
        value,
        multiplier,
    };

    // Find insertion position (descending order)
    let insert_pos = leaderboard
        .iter()
        .position(|entry| entry.value < value)
        .unwrap_or(leaderboard.len());

    // Insert at correct position
    leaderboard.insert(insert_pos, new_entry);

    // Trim to max size
    if leaderboard.len() > MAX_LEADERBOARD_SIZE {
        leaderboard.truncate(MAX_LEADERBOARD_SIZE);
    }
}

/// Check if daily leaderboard needs reset (00:00 UTC)
pub fn should_reset_daily(last_reset: u64, current_time: u64) -> bool {
    const SECONDS_PER_DAY: u64 = 86400;

    // Get the UTC day for both timestamps
    let last_reset_day = last_reset / SECONDS_PER_DAY;
    let current_day = current_time / SECONDS_PER_DAY;

    current_day > last_reset_day
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::Addr;

    #[test]
    fn test_update_leaderboard_new_entry() {
        let mut leaderboard = vec![];

        update_leaderboard(
            &mut leaderboard,
            Addr::unchecked("player1"),
            Uint128::new(100),
            Some("2.0x".to_string()),
        );

        assert_eq!(leaderboard.len(), 1);
        assert_eq!(leaderboard[0].player.as_str(), "player1");
        assert_eq!(leaderboard[0].value, Uint128::new(100));
    }

    #[test]
    fn test_update_leaderboard_sorted_order() {
        let mut leaderboard = vec![];

        update_leaderboard(
            &mut leaderboard,
            Addr::unchecked("player1"),
            Uint128::new(100),
            None,
        );
        update_leaderboard(
            &mut leaderboard,
            Addr::unchecked("player2"),
            Uint128::new(200),
            None,
        );
        update_leaderboard(
            &mut leaderboard,
            Addr::unchecked("player3"),
            Uint128::new(150),
            None,
        );

        assert_eq!(leaderboard.len(), 3);
        assert_eq!(leaderboard[0].value, Uint128::new(200));
        assert_eq!(leaderboard[1].value, Uint128::new(150));
        assert_eq!(leaderboard[2].value, Uint128::new(100));
    }

    #[test]
    fn test_update_leaderboard_replace_existing() {
        let mut leaderboard = vec![];

        update_leaderboard(
            &mut leaderboard,
            Addr::unchecked("player1"),
            Uint128::new(100),
            None,
        );
        update_leaderboard(
            &mut leaderboard,
            Addr::unchecked("player1"),
            Uint128::new(200),
            None,
        );

        assert_eq!(leaderboard.len(), 1);
        assert_eq!(leaderboard[0].value, Uint128::new(200));
    }

    #[test]
    fn test_should_reset_daily_same_day() {
        let base_time = 1704067200; // 2024-01-01 00:00:00 UTC
        let later_same_day = base_time + 3600; // 1 hour later

        assert!(!should_reset_daily(base_time, later_same_day));
    }

    #[test]
    fn test_should_reset_daily_next_day() {
        let base_time = 1704067200; // 2024-01-01 00:00:00 UTC
        let next_day = base_time + 86400; // Next day

        assert!(should_reset_daily(base_time, next_day));
    }

    #[test]
    fn test_should_reset_daily_multiple_days() {
        let base_time = 1704067200; // 2024-01-01 00:00:00 UTC
        let three_days_later = base_time + (86400 * 3);

        assert!(should_reset_daily(base_time, three_days_later));
    }
}
