use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

// this example is basic on substrate2.0 not rc4 before.
// so some word is changed but this example still can be used
// you can find it on https://substrate.dev/docs/zh-CN/tutorials/build-a-dapp/


//
#[test]
fn create_claim_works(){
    new_test_ext().execute_with(||{
        let proof:Vec<u8> = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1),proof.clone()));

        assert_eq!(Proofs::<Test>::get(&proof),(1,frame_system::Module::<Test>::block_number()));

    })
}
//  创建存证的测试用例
/// The proof has already been claimed.
#[test]
fn create_claim_failed_when_claim_already_exist(){
    new_test_ext().execute_with(||{
        let proof:Vec<u8> = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1),proof.clone());

        assert_noop!(
           PoeModule::create_claim(Origin::signed(1),proof.clone()),
           Error::<Test>::ProofAlreadyClaimed
        );
    })
}

//  撤销存证的测试用例
#[test]
fn revoke_claim_works(){
    new_test_ext().execute_with(||{
        let proof:Vec<u8> = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1),proof.clone());

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1),proof.clone()));

    })
}

/// The proof does not exist, so it cannot be revoked.
#[test]
fn revoke_claim_failed_when_claim_is_not_exist(){
    new_test_ext().execute_with(||{
        let proof:Vec<u8> = vec![0,1];


        assert_noop!(
           PoeModule::revoke_claim(Origin::signed(1),proof.clone()),
           Error::<Test>::NoSuchProof,
        );
    })
}

/// The proof is claimed by another account, so caller can't revoke it.
#[test]
fn claim_but_another_account_can_not_revoked(){

    new_test_ext().execute_with(||{
        let proof:Vec<u8> = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1),proof.clone()));

        assert_noop!(
           PoeModule::revoke_claim(Origin::signed(2),proof.clone()),
           Error::<Test>::NotProofOwner,
        );
    })
}

//转移存证的测试用例
//转移流程为账户1创建存证后，删除账户1的存证信息填入所有权【可以删除存证的】2账户地址和当前区块号
#[test]
fn transfer_claim(){
    new_test_ext().execute_with(||{
        let proof:Vec<u8> = vec![0,1];
        let account_id:u64 = 1;
        let _ = PoeModule::create_claim(Origin::signed(1),proof.clone());
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1),proof.clone(),account_id));
    })
}

//长度检查，溢出返回错误
#[test]
fn claim_legend(){
    new_test_ext().execute_with(||{
        let proof:Vec<u8> = vec![0,1,2,3,4,5];

        assert_noop!(
           PoeModule::create_claim_check_legend(Origin::signed(1),proof.clone()),
           Error::<Test>::Exceedsrange,
        );
    })
}


