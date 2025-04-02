#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::helpers::CwTemplateContract;
    use crate::msg::{ContractResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json};
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn test_proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());

        let msg = InstantiateMsg {
            supra_pull_contract: "Test".to_string(),
        };
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    mod update_contract {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn test_update_contract_integrated() {
            let (mut app, cw_template_contract) = test_proper_instantiate();

            let msg = ExecuteMsg::UpdateSupraContract {
                supra_pull_contract: "Test Update".to_string(),
            };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }
    #[test]
    fn test_proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            supra_pull_contract: "Test".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetSupraPullContract {}).unwrap();
        let value: ContractResponse = from_json(&res).unwrap();
        assert_eq!("Test".to_string(), value.supra_pull_contract);
    }

    #[test]
    fn test_update_supra_contract() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            supra_pull_contract: "Test".to_string(),
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::UpdateSupraContract {
            supra_pull_contract: "Test Update".to_string(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Receive the response and unwrap() to fetch the supra oracle pull contract
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetSupraPullContract {}).unwrap();
        let value: ContractResponse = from_json(&res).unwrap();
        assert_eq!("Test Update".to_string(), value.supra_pull_contract);
    }
}
