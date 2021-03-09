use std::convert::TryInto;

use async_trait::async_trait;
use sqlx::PgPool;

use crate::polling::dto::{NewRun, Run, RunId};
use crate::polling::errors::ServiceResult;
use crate::polling::run_repository::RunRepository;

#[derive(Clone, Debug)]
pub struct PostgresRunRepository {
    db_pool: PgPool,
}

impl PostgresRunRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl RunRepository for PostgresRunRepository {
    async fn generate_run_id(&self) -> RunId {
        RunId::new_v4()
    }

    async fn save_run(&self, run: &NewRun) -> ServiceResult<()> {
        sqlx::query!(
            r#"
            insert into run (run_id)
            values ($1)
            "#,
            run.id
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    async fn update_run(&self, run: &Run) -> ServiceResult<()> {
        let query_result = sqlx::query!(
            r#"
            update run set status_id = $1,
                           run_successful_responses = $2,
                           run_value_sum = $3
            where run_id = $4
            "#,
            run.status as i16,
            run.successful_responses_count as i64,
            run.sum as i64,
            run.id,
        )
        .execute(&self.db_pool)
        .await?;

        assert_eq!(
            1,
            query_result.rows_affected(),
            "DAO method 'finish_run' updated 0 rows, tried to finish run with id: {}",
            run.id
        );

        Ok(())
    }

    async fn get_run_by_id(&self, run_id: RunId) -> ServiceResult<Run> {
        let row = sqlx::query!(
            r#"
            select r.status_id,
                   r.run_successful_responses,
                   r.run_value_sum
            from run r
            where r.run_id = $1;
            "#,
            run_id
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(Run {
            id: run_id,
            status: row.status_id.try_into()?,
            successful_responses_count: row.run_successful_responses as u64,
            sum: row.run_value_sum as u64,
        })
    }
}
