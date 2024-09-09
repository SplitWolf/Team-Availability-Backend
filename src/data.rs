use axum::async_trait;

use crate::model::*;

#[async_trait]
#[allow(dead_code)]
pub trait AvailablityStore {
    // Users
    async fn get_user_by_id(
        &self,
        user_id: i32,
    ) -> Result<Option<IdentifiableUser>, sqlx::error::Error>;
    async fn get_user_by_name(
        &self,
        user_name: String,
    ) -> Result<Option<IdentifiableUser>, sqlx::error::Error>;
    async fn add_user(&self, user: User) -> Result<IdentifiableUser, sqlx::error::Error>;
    async fn update_user(&self, user: IdentifiableUser) -> Result<IdentifiableUser, sqlx::error::Error>;
    async fn delete_user(&self, user_id: i32) -> Result<(), sqlx::error::Error>;

    // Teams
    async fn get_team_by_id(
        &self,
        team_id: i32,
    ) -> Result<Option<IdentifiableTeam>, sqlx::error::Error>;
    async fn get_team_by_name(
        &self,
        team_name: String,
    ) -> Result<Option<IdentifiableTeam>, sqlx::error::Error>;
    async fn add_team(&self, team: Team) -> Result<IdentifiableTeam, sqlx::error::Error>;
    async fn update_team(
        &self,
        team: IdentifiableTeam,
    ) -> Result<IdentifiableTeam, sqlx::error::Error>;
    async fn delete_team(&self, team_id: i32) -> Result<(), sqlx::error::Error>;

    // Players
    async fn get_player_by_id(
        &self,
        player_id: i32,
    ) -> Result<Option<IdentifiablePlayer>, sqlx::error::Error>;
    async fn get_player_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<IdentifiablePlayer>, sqlx::error::Error>;
    async fn add_player(&self, player: Player) -> Result<IdentifiablePlayer, sqlx::error::Error>;
    async fn update_player(
        &self,
        player: IdentifiablePlayer,
    ) -> Result<IdentifiablePlayer, sqlx::error::Error>;
    async fn delete_player(&self, player_id: i32) -> Result<(), sqlx::error::Error>;

    // Avail Blocks
    async fn get_available_block_by_id(
        &self,
        block_id: i32,
    ) -> Result<Option<IdentifiableAvailableBlock>, sqlx::error::Error>;
    async fn get_available_blocks_by_player_id(
        &self,
        player_id: i32,
    ) -> Result<Vec<IdentifiableAvailableBlock>, sqlx::error::Error>;
    async fn add_available_block(
        &self,
        block: AvailableBlock,
    ) -> Result<IdentifiableAvailableBlock, sqlx::error::Error>;
    async fn update_available_block(
        &self,
        block: IdentifiableAvailableBlock,
    ) -> Result<IdentifiableAvailableBlock, sqlx::error::Error>;
    async fn delete_available_block(&self, block_id: i32) -> Result<(), sqlx::error::Error>;
}

//TODO: Change updates to have thier own data type
pub struct PostgresAvailablityStore {
    pool: sqlx::PgPool,
}

impl PostgresAvailablityStore {
    pub fn new(pool: sqlx::PgPool) -> Self {
        PostgresAvailablityStore { pool }
    }
}

#[async_trait]
impl AvailablityStore for PostgresAvailablityStore {
    //Users
    async fn get_user_by_id(
        &self,
        user_id: i32,
    ) -> Result<Option<IdentifiableUser>, sqlx::error::Error> {
        sqlx::query_as!(
            IdentifiableUser,
            //Id's are unique should only return one user
            "SELECT * FROM users WHERE id=$1",
            user_id,
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_user_by_name(
        &self,
        user_name: String,
    ) -> Result<Option<IdentifiableUser>, sqlx::error::Error> {
        sqlx::query_as!(
            IdentifiableUser,
            //Id's are unique should only return one user
            "SELECT * FROM users WHERE name=$1",
            user_name,
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn add_user(&self, user: User) -> Result<IdentifiableUser, sqlx::error::Error> {
        sqlx::query_as!(
            IdentifiableUser,
            "INSERT INTO users(name) VALUES ($1) RETURNING id, name",
            user.name
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update_user(&self, user: IdentifiableUser) -> Result<IdentifiableUser, sqlx::error::Error> {
        sqlx::query_as!(
            IdentifiableUser,
            "UPDATE users SET name=$1 WHERE id=$2 RETURNING id, name",
            user.name,
            user.id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn delete_user(&self, user_id: i32) -> Result<(), sqlx::error::Error> {
        sqlx::query!("DELETE FROM users WHERE id=$1", user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    //Teams
    async fn get_team_by_id(
        &self,
        team_id: i32,
    ) -> Result<Option<IdentifiableTeam>, sqlx::error::Error> {
        sqlx::query_as!(
            IdentifiableTeam,
            //Id's are unique should only return one user
            "SELECT * FROM teams WHERE id=$1",
            team_id,
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_team_by_name(
        &self,
        team_name: String,
    ) -> Result<Option<IdentifiableTeam>, sqlx::error::Error> {
        sqlx::query_as!(
            IdentifiableTeam,
            //Id's are unique should only return one user
            "SELECT * FROM teams WHERE name=$1",
            team_name,
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn add_team(&self, team: Team) -> Result<IdentifiableTeam, sqlx::error::Error> {
        sqlx::query_as!(
            IdentifiableTeam,
            "INSERT INTO teams(name) VALUES ($1) RETURNING id, name",
            team.name
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update_team(
        &self,
        team: IdentifiableTeam,
    ) -> Result<IdentifiableTeam, sqlx::error::Error> {
        sqlx::query_as!(
            IdentifiableTeam,
            "UPDATE users SET name=$1 WHERE id=$2 RETURNING id, name",
            team.name,
            team.id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn delete_team(&self, team_id: i32) -> Result<(), sqlx::error::Error> {
        sqlx::query!("DELETE FROM teams WHERE id=$1", team_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    //Players
    async fn get_player_by_id(
        &self,
        player_id: i32,
    ) -> Result<Option<IdentifiablePlayer>, sqlx::error::Error> {
        sqlx::query_as!(
            IdentifiablePlayer,
            //Id's are unique should only return one user
            "SELECT * FROM players WHERE id=$1",
            player_id,
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_player_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<IdentifiablePlayer>, sqlx::error::Error> {
        sqlx::query_as!(
            IdentifiablePlayer,
            //Id's are unique should only return one user
            "SELECT * FROM players WHERE user_id=$1",
            user_id,
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn add_player(&self, player: Player) -> Result<IdentifiablePlayer, sqlx::error::Error> {
        sqlx::query_as!(
            IdentifiablePlayer,
            "INSERT INTO players VALUES ($1) RETURNING id, user_id",
            player.user_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update_player(
        &self,
        player: IdentifiablePlayer,
    ) -> Result<IdentifiablePlayer, sqlx::error::Error> {
        sqlx::query_as!(
            IdentifiablePlayer,
            "UPDATE players SET user_id=$1 WHERE id=$2 RETURNING id, user_id",
            player.user_id,
            player.id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn delete_player(&self, player_id: i32) -> Result<(), sqlx::error::Error> {
        sqlx::query!("DELETE FROM users WHERE id=$1", player_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Blocks
    async fn get_available_block_by_id(
        &self,
        block_id: i32,
    ) -> Result<Option<IdentifiableAvailableBlock>, sqlx::error::Error> {
        sqlx::query_as::<_, IdentifiableAvailableBlock>(
            //Id's are unique should only return one block
            "SELECT * FROM available_blocks WHERE id=$1",
        )
        .bind(block_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_available_blocks_by_player_id(
        &self,
        player_id: i32,
    ) -> Result<Vec<IdentifiableAvailableBlock>, sqlx::error::Error> {
        sqlx::query_as::<_, IdentifiableAvailableBlock>(
            //Id's are unique should only return one player's blocks
            "SELECT * FROM available_blocks WHERE player_id=$1",
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn add_available_block(
        &self,
        block: AvailableBlock,
    ) -> Result<IdentifiableAvailableBlock, sqlx::error::Error> {
        sqlx::query_as::<_,IdentifiableAvailableBlock>(
            "INSERT INTO available_blocks (start_time, end_time, needs_waring, repeats, player_id) 
            VALUES ($1, $2, $3, $4, $5) 
            RETURNING id, start_time, end_time, needs_waring, repeats, player_id",
        )
        .bind(block.start_time)
        .bind(block.end_time)
        .bind(block.need_warning)
        .bind(block.repeats.to_string())
        .bind(block.player_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_available_block(
        &self,
        block: IdentifiableAvailableBlock,
    ) -> Result<IdentifiableAvailableBlock, sqlx::error::Error> {
        sqlx::query_as::<_,IdentifiableAvailableBlock>(
            "UPDATE available_blocks SET start_time=$1, end_time=$2, needs_waring=$3, repeats=$4, player_id=$5) 
            WHERE block_id=$6
            RETURNING id, start_time, end_time, needs_waring, repeats, player_id",
        )
        .bind(block.inner_block.start_time)
        .bind(block.inner_block.end_time)
        .bind(block.inner_block.need_warning)
        .bind(block.inner_block.repeats.to_string())
        .bind(block.inner_block.player_id)
        .bind(block.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete_available_block(&self, block_id: i32) -> Result<(), sqlx::error::Error> {
        sqlx::query!("DELETE FROM available_blocks WHERE id=$1", block_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
