use super::Database;
use crate::{database::HistoryToken, file::File, Operation, Position, State, History, Token, TokenInfo, POS};
use anyhow::Result;
use rusqlite::params;

impl Database {
    pub fn is_file(&mut self, name: &str) -> Result<bool> {
        Ok(self.select_file_for_name(name).is_ok())
    }

    pub fn select_file_for_name(&self, name: &str) -> Result<File> {
        Ok(self.conn.query_row(
            r#"SELECT id, name FROM files WHERE name=?"#,
            params![name],
            |row| {
                Ok(File {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            },
        )?)
    }

    pub fn select_current_history_for_file(&self, file: &File) -> Result<History> {
        let file_id = file.id;

        Ok(self.conn.query_row(
            r#"SELECT id, file_id, start_date, end_date FROM history WHERE file_id=? ORDER BY id DESC"#,
            params![file_id],
            |row| {
                let end_date = row.get(3).ok();

                Ok(History {
                    id: row.get(0)?,
                    file_id: row.get(1)?,
                    start_date: row.get(2)?,
                    end_date,
                })
            },
        )?)
    }

    pub fn select_id_for_token(&self, token: &Token) -> Result<i32> {
        Ok(self.conn.query_row(
            r#"SELECT id FROM tokens WHERE lemma=? AND pos=?"#,
            params![token.lemma, token.pos.to_int()],
            |row| row.get(0),
        )?)
    }

    pub fn select_token_for_id(&self, id: i32) -> Result<Token> {
        Ok(self.conn.query_row(
            r#"SELECT lemma, pos FROM tokens WHERE id=?"#,
            params![id],
            |row| {
                Ok(Token {
                    lemma: row.get(0)?,
                    pos: POS::to_pos(row.get(1)?),
                    text: row.get(0)?,
                })
            },
        )?)
    }

    pub fn select_history_token_for_history_id_and_token_id(
        &self,
        history_id: i32,
        token_id: i32,
    ) -> Result<HistoryToken> {
        Ok(self.conn.query_row(
            r#"SELECT history_id, token_id, total_unknown, total_seen FROM historytokens 
                WHERE history_id=? AND token_id=?"#,
            params![history_id, token_id],
            |row| {
                Ok(HistoryToken::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                ))
            },
        )?)
    }

    pub fn select_total_seen_for_history_id(&mut self, history_id: i32) -> Result<i32> {
        let mut stmt = self
            .conn
            .prepare(r#"SELECT total_seen FROM historytokens WHERE history_id=?"#)?;

        let total_seen: i32 = stmt
            .query_map(params![history_id], |row| Ok(row.get::<usize, i32>(0)))?
            .flatten()
            .map(|row| row.unwrap())
            .sum();

        Ok(total_seen)
    }

    pub fn select_total_unknown_for_history_id(&mut self, history_id: i32) -> Result<i32> {
        let mut stmt = self
            .conn
            .prepare(r#"SELECT total_unknown FROM historytokens WHERE history_id=?"#)?;

        let total_seen: i32 = stmt
            .query_map(params![history_id], |row| Ok(row.get::<usize, i32>(0)))?
            .flatten()
            .map(|row| row.unwrap())
            .sum();

        Ok(total_seen)
    }

    pub fn select_unknown_for_history(&mut self, history_id: i32) -> Result<Vec<TokenInfo>> {
        let mut stmt = self.conn.prepare(
            r#"SELECT token_id FROM historytokens 
            WHERE history_id=? AND total_unknown>0
            ORDER BY total_unknown DESC"#,
        )?;

        let unknown_ids: Vec<i32> = stmt
            .query_map(params![history_id], |row| Ok(row.get::<usize, i32>(0)))?
            .flatten()
            .map(|row| row.unwrap())
            .collect();

        let mut unknown_tokens: Vec<TokenInfo> = Vec::new();

        for id in unknown_ids {
            let history_token =
                self.select_history_token_for_history_id_and_token_id(history_id, id)?;
            let token = self.select_token_for_id(id)?;

            let unknown_token = TokenInfo {
                token,
                total_seen: history_token.total_seen,
                total_unknown: history_token.total_unknown,
            };

            unknown_tokens.push(unknown_token);
        }

        Ok(unknown_tokens)
    }

    pub fn select_newest_state_for_file_id(&mut self, file_id: i32) -> Result<State> {
        Ok(self.conn.query_row(
                r#"SELECT file_id, idx, line, operation_num, action FROM state
                    WHERE file_id=? ORDER BY operation_num DESC"# , 
                    params![file_id], 
                    |row| {
                        let position = Some(Position {
                            index: row.get(1)?,
                            line: row.get(2)?,
                        });

                        Ok(State {
                            file_id: row.get(0)?,
                            position,
                            operation_num: row.get(3)?,
                            action: Some(Operation::from_int(row.get(4)?)),
                        })
                    })?)
    }
}
