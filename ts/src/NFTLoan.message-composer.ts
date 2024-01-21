/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.25.2.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { MsgExecuteContractEncodeObject } from "cosmwasm";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { ExecuteMsg, AssetInfo, Uint128, Decimal, Cw721Coin, Sg721Token, Coin, LoanTerms, InstantiateMsg, QueryMsg } from "./NFTLoan.types";
export interface NFTLoanMessage {
  contractAddress: string;
  sender: string;
  depositCollaterals: ({
    comment,
    loanPreview,
    terms,
    tokens
  }: {
    comment?: string;
    loanPreview?: AssetInfo;
    terms?: LoanTerms;
    tokens: AssetInfo[];
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  modifyCollaterals: ({
    comment,
    loanId,
    loanPreview,
    terms
  }: {
    comment?: string;
    loanId: number;
    loanPreview?: AssetInfo;
    terms?: LoanTerms;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  withdrawCollaterals: ({
    loanId
  }: {
    loanId: number;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  makeOffer: ({
    borrower,
    comment,
    loanId,
    terms
  }: {
    borrower: string;
    comment?: string;
    loanId: number;
    terms: LoanTerms;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  cancelOffer: ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  refuseOffer: ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  withdrawRefusedOffer: ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  acceptOffer: ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  acceptLoan: ({
    borrower,
    comment,
    loanId
  }: {
    borrower: string;
    comment?: string;
    loanId: number;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  repayBorrowedFunds: ({
    loanId
  }: {
    loanId: number;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  withdrawDefaultedLoan: ({
    borrower,
    loanId
  }: {
    borrower: string;
    loanId: number;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  setOwner: ({
    owner
  }: {
    owner: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  setFeeDistributor: ({
    feeDepositor
  }: {
    feeDepositor: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  setFeeRate: ({
    feeRate
  }: {
    feeRate: Decimal;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class NFTLoanMessageComposer implements NFTLoanMessage {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.depositCollaterals = this.depositCollaterals.bind(this);
    this.modifyCollaterals = this.modifyCollaterals.bind(this);
    this.withdrawCollaterals = this.withdrawCollaterals.bind(this);
    this.makeOffer = this.makeOffer.bind(this);
    this.cancelOffer = this.cancelOffer.bind(this);
    this.refuseOffer = this.refuseOffer.bind(this);
    this.withdrawRefusedOffer = this.withdrawRefusedOffer.bind(this);
    this.acceptOffer = this.acceptOffer.bind(this);
    this.acceptLoan = this.acceptLoan.bind(this);
    this.repayBorrowedFunds = this.repayBorrowedFunds.bind(this);
    this.withdrawDefaultedLoan = this.withdrawDefaultedLoan.bind(this);
    this.setOwner = this.setOwner.bind(this);
    this.setFeeDistributor = this.setFeeDistributor.bind(this);
    this.setFeeRate = this.setFeeRate.bind(this);
  }

  depositCollaterals = ({
    comment,
    loanPreview,
    terms,
    tokens
  }: {
    comment?: string;
    loanPreview?: AssetInfo;
    terms?: LoanTerms;
    tokens: AssetInfo[];
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          deposit_collaterals: {
            comment,
            loan_preview: loanPreview,
            terms,
            tokens
          }
        })),
        funds
      })
    };
  };
  modifyCollaterals = ({
    comment,
    loanId,
    loanPreview,
    terms
  }: {
    comment?: string;
    loanId: number;
    loanPreview?: AssetInfo;
    terms?: LoanTerms;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          modify_collaterals: {
            comment,
            loan_id: loanId,
            loan_preview: loanPreview,
            terms
          }
        })),
        funds
      })
    };
  };
  withdrawCollaterals = ({
    loanId
  }: {
    loanId: number;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          withdraw_collaterals: {
            loan_id: loanId
          }
        })),
        funds
      })
    };
  };
  makeOffer = ({
    borrower,
    comment,
    loanId,
    terms
  }: {
    borrower: string;
    comment?: string;
    loanId: number;
    terms: LoanTerms;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          make_offer: {
            borrower,
            comment,
            loan_id: loanId,
            terms
          }
        })),
        funds
      })
    };
  };
  cancelOffer = ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          cancel_offer: {
            global_offer_id: globalOfferId
          }
        })),
        funds
      })
    };
  };
  refuseOffer = ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          refuse_offer: {
            global_offer_id: globalOfferId
          }
        })),
        funds
      })
    };
  };
  withdrawRefusedOffer = ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          withdraw_refused_offer: {
            global_offer_id: globalOfferId
          }
        })),
        funds
      })
    };
  };
  acceptOffer = ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          accept_offer: {
            global_offer_id: globalOfferId
          }
        })),
        funds
      })
    };
  };
  acceptLoan = ({
    borrower,
    comment,
    loanId
  }: {
    borrower: string;
    comment?: string;
    loanId: number;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          accept_loan: {
            borrower,
            comment,
            loan_id: loanId
          }
        })),
        funds
      })
    };
  };
  repayBorrowedFunds = ({
    loanId
  }: {
    loanId: number;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          repay_borrowed_funds: {
            loan_id: loanId
          }
        })),
        funds
      })
    };
  };
  withdrawDefaultedLoan = ({
    borrower,
    loanId
  }: {
    borrower: string;
    loanId: number;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          withdraw_defaulted_loan: {
            borrower,
            loan_id: loanId
          }
        })),
        funds
      })
    };
  };
  setOwner = ({
    owner
  }: {
    owner: string;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          set_owner: {
            owner
          }
        })),
        funds
      })
    };
  };
  setFeeDistributor = ({
    feeDepositor
  }: {
    feeDepositor: string;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          set_fee_distributor: {
            fee_depositor: feeDepositor
          }
        })),
        funds
      })
    };
  };
  setFeeRate = ({
    feeRate
  }: {
    feeRate: Decimal;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          set_fee_rate: {
            fee_rate: feeRate
          }
        })),
        funds
      })
    };
  };
}