// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::collections::HashMap;
use std::sync::Arc;

use common_exception::ErrorCode;
use common_exception::Result;
use common_meta_types::CreateDatabaseReply;
use common_meta_types::MetaId;
use common_meta_types::MetaVersion;
use common_planners::CreateDatabasePlan;
use common_planners::DropDatabasePlan;

use crate::catalogs::catalog::Catalog;
use crate::catalogs::Database;
use crate::catalogs::Table;
use crate::configs::Config;
use crate::datasources::database::system::SystemDatabase;

/// System Catalog contains ... all the system databases (no surprise :)
/// Currently, this is only one database here, the "system" db.
/// "information_schema" db is supposed to held here
pub struct SystemCatalog {
    dbs: HashMap<String, Arc<dyn Database>>,
}

impl SystemCatalog {
    pub fn try_create_with_config(_conf: &Config) -> Result<Self> {
        let mut dbs = HashMap::new();
        let sys_db = Arc::new(SystemDatabase::create("system")) as Arc<dyn Database>;
        dbs.insert(sys_db.name().to_owned(), sys_db);
        Ok(Self { dbs })
    }
}

impl Catalog for SystemCatalog {
    fn get_databases(&self) -> Result<Vec<Arc<dyn Database>>> {
        let r = self
            .dbs
            .iter()
            .map(|(_, item)| item.clone())
            .collect::<Vec<_>>();
        Ok(r)
    }

    fn get_database(&self, db_name: &str) -> Result<Arc<dyn Database>> {
        let db = self
            .dbs
            .get(db_name)
            .ok_or_else(|| ErrorCode::UnknownDatabase(format!("unknown database {}", db_name)))?;
        Ok(db.clone())
    }

    fn get_table(&self, db_name: &str, table_name: &str) -> Result<Arc<dyn Table>> {
        let db = self.get_database(db_name)?;
        db.get_table(table_name)
    }

    fn get_table_by_id(
        &self,
        table_id: MetaId,
        table_version: Option<MetaVersion>,
    ) -> Result<Arc<dyn Table>> {
        for db in self.dbs.values() {
            let tbl = db.get_table_by_id(table_id, table_version);
            match tbl {
                Ok(tbl) => return Ok(tbl),
                Err(_) => continue,
            }
        }
        Err(ErrorCode::UnknownTable(format!(
            "unknown table of id {}",
            table_id
        )))
    }

    fn create_database(&self, _plan: CreateDatabasePlan) -> Result<CreateDatabaseReply> {
        Err(ErrorCode::UnImplement("Cannot create system database"))
    }

    fn drop_database(&self, _plan: DropDatabasePlan) -> Result<()> {
        Err(ErrorCode::UnImplement("Cannot drop system database"))
    }
}
