use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, export_schema_with_title, remove_schemas, schema_for};

use registry::msg::{ContractResponse, ContractsResponse, ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema_with_title(
        &schema_for!(ContractResponse),
        &out_dir,
        "GetContractResponse",
    );
    export_schema_with_title(
        &schema_for!(ContractsResponse),
        &out_dir,
        "GetContractsResponse",
    );
    export_schema_with_title(
        &schema_for!(ContractsResponse),
        &out_dir,
        "GetActiveContractsResponse",
    );

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
}
