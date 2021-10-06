use super::Database;
use anyhow::Result;

impl Database {
    pub fn initialize(&self) -> Result<()> {
        self.create_files()?;
        self.create_tokens()?;
        self.create_state()?;
        self.create_history()?;
        self.create_history_tokens()?;
        self.create_file_tokens()?;

        Ok(())
    }

    pub fn create_files(&self) -> Result<()> {
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                eof INTEGER NOT NULL
                );
            "#,
            [],
        )?;

        Ok(())
    }

    pub fn create_tokens(&self) -> Result<()> {
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS tokens (
                id INTEGER PRIMARY KEY,
                lemma TEXT NOT NULL,
                pos TEXT NOT NULL,
                CONSTRAINT uq_token UNIQUE(lemma, pos)
                );
            "#,
            [],
        )?;

        Ok(())
    }

    pub fn create_state(&self) -> Result<()> {
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS state (
                id INTEGER PRIMARY KEY,
                file_id INTEGER NOT NULL,
                idx INTEGER NOT NULL,
                line INTEGER NOT NULL,
                operation INTEGER NOT NULL,
                total INTEGER NOT NULL,
                unknown INTEGER NOT NULL,
                action INTEGER NOT NULL,
                FOREIGN KEY (file_id) REFERENCES files(id)
                );
            "#,
            [],
        )?;

        Ok(())
    }

    pub fn create_history(&self) -> Result<()> {
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY,
                date TEXT NOT NULL,
                file_id INTEGER NOT NULL,
                total INTEGER NOT NULL,
                unknown INTEGER NOT NULL,
                FOREIGN KEY (file_id) REFERENCES files(id)
                );
            "#,
            [],
        )?;

        Ok(())
    }

    pub fn create_history_tokens(&self) -> Result<()> {
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS historytokens (
                history_id INTEGER NOT NULL,
                token_id INTEGER NOT NULL,
                unknown INTEGER NOT NULL,
                total INTEGER NOT NULL,
                PRIMARY KEY (history_id, token_id)
                );
            "#,
            [],
        )?;

        Ok(())
    }

    pub fn create_file_tokens(&self) -> Result<()> {
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS filetokens (
                file_id INTEGER NOT NULL,
                token_id INTEGER NOT NULL,
                unknown INTEGER NOT NULL,
                total INTEGER NOT NULL,
                PRIMARY KEY (file_id, token_id)
                );
            "#,
            [],
        )?;

        Ok(())
    }
}
