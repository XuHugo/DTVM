// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// mocked hostapis only used to JIT compile

#define MOCK_CHAIN_DUMMY_IMPLEMENTATION

static const std::string INTERFACE_API = "testcaseAbi";

static int32_t MyAbort(zen::runtime::Instance *Inst, int32_t Exception,
                       int32_t Len) {
  using namespace zen::common;
  char Buf[32];
  ::snprintf(Buf, sizeof(Buf), "(%d, %d)", Exception, Len);
  Inst->setExceptionByHostapi(
      getErrorWithExtraMessage(ErrorCode::EnvAbort, Buf));
  return 0;
}

static int32_t memcpy(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t wmemcpy(zen::runtime::Instance *Inst, int32_t, int32_t,
                       int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t memset(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t wmemset(zen::runtime::Instance *Inst, int32_t, int32_t,
                       int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t strlen(zen::runtime::Instance *Inst, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t wcslen(zen::runtime::Instance *Inst, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int64_t strtoll(zen::runtime::Instance *Inst, int32_t, int32_t,
                       int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int64_t strtoull(zen::runtime::Instance *Inst, int32_t, int32_t,
                        int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t __strchrnul(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t strncmp(zen::runtime::Instance *Inst, int32_t, int32_t,
                       int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t memmove(zen::runtime::Instance *Inst, int32_t, int32_t,
                       int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t wmemmove(zen::runtime::Instance *Inst, int32_t, int32_t,
                        int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t memchr(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t memcmp(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t SetStorage(zen::runtime::Instance *Inst, int32_t, int32_t,
                          int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetStorageSize(zen::runtime::Instance *Inst, int32_t, int32_t,
                              int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t DeleteStorage(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t strcmp(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetSender(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t SetReturnValue(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t Log(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t,
                   int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t print(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetBlockHash(zen::runtime::Instance *Inst, int64_t, int32_t,
                            int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int64_t GetBlockNumber(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int64_t GetBlockTimeStamp(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetStorage(zen::runtime::Instance *Inst, int32_t, int32_t,
                          int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetCode(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t,
                       int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetDataSize(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetData(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetTxHash(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int64_t GetGas(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int64_t GetValue(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetOrigin(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t CheckAccount(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t IsLocalTx(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetAccountStatus(zen::runtime::Instance *Inst, int32_t, int32_t,
                                int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetCodeHash(zen::runtime::Instance *Inst, int32_t, int32_t,
                           int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetAuthMap(zen::runtime::Instance *Inst, int32_t, int32_t,
                          int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetAuthMapInCache(zen::runtime::Instance *Inst, int32_t, int32_t,
                                 int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetBalance(zen::runtime::Instance *Inst, int32_t, int32_t,
                          int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t TransferBalance(zen::runtime::Instance *Inst, int32_t, int32_t,
                               int64_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t Result(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetSelf(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetRecoverKey(zen::runtime::Instance *Inst, int32_t, int32_t,
                             int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t ReadInterfaceName(zen::runtime::Instance *Inst, int32_t NameData,
                                 int32_t NameLenOffset) {
  uint32_t MethodSize = INTERFACE_API.size();

  int Ret = -1;
  auto instance = Inst; // VALIDATE_APP_ADDR macro use it
  if (!VALIDATE_APP_ADDR(NameData, MethodSize))
    return Ret;

  if (!VALIDATE_APP_ADDR(NameLenOffset, sizeof(uint32_t))) {
    return Ret;
  }

  uint32_t *LengthPtr = (uint32_t *)ADDR_APP_TO_NATIVE(NameLenOffset);
  *LengthPtr = MethodSize;

  char *DataPtr = (char *)ADDR_APP_TO_NATIVE(NameData);
  std::memcpy(DataPtr, INTERFACE_API.data(), MethodSize);

  return 0;
}

static int32_t ReadInterfaceNameSize(zen::runtime::Instance *Inst) {
  return INTERFACE_API.size();
}

static int32_t ReadInterfaceParams(zen::runtime::Instance *Inst, int32_t,
                                   int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t ReadInterfaceParamsSize(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t CallContract(zen::runtime::Instance *Inst, int32_t, int32_t,
                            int32_t, int32_t, int64_t, int64_t, int32_t,
                            int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t DelegateCall(zen::runtime::Instance *Inst, int32_t, int32_t,
                            int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t VerifyCommitment(zen::runtime::Instance *Inst, int32_t, int32_t,
                                int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t VerifyRange(zen::runtime::Instance *Inst, int32_t, int64_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t VerifyBalance(zen::runtime::Instance *Inst, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t Ecrecovery(zen::runtime::Instance *Inst, int32_t, int32_t,
                          int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t Digest(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t,
                      int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t VerifyRsa(zen::runtime::Instance *Inst, int32_t, int32_t,
                         int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t VerifyRsa2(zen::runtime::Instance *Inst, int32_t, int32_t,
                          int32_t, int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t VerifyMessageSM2(zen::runtime::Instance *Inst, int32_t, int32_t,
                                int32_t, int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t Base64Encode(zen::runtime::Instance *Inst, int32_t, int32_t,
                            int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t Base64Decode(zen::runtime::Instance *Inst, int32_t, int32_t,
                            int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t println(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetCallResult(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetCallResultSize(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t VerifyMessageECCK1(zen::runtime::Instance *Inst, int32_t,
                                  int32_t, int32_t, int32_t, int32_t, int32_t,
                                  int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t VerifyMessageECCR1(zen::runtime::Instance *Inst, int32_t,
                                  int32_t, int32_t, int32_t, int32_t, int32_t,
                                  int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t RangeProofVerify(zen::runtime::Instance *Inst, int32_t, int32_t,
                                int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t AddPedersenCommit(zen::runtime::Instance *Inst, int32_t, int32_t,
                                 int32_t, int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t SubPedersenCommit(zen::runtime::Instance *Inst, int32_t, int32_t,
                                 int32_t, int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t CalculatePedersenCommit(zen::runtime::Instance *Inst, int32_t,
                                       int32_t, int32_t, int32_t, int32_t,
                                       int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t PedersenCommitEqualityVerify(zen::runtime::Instance *Inst,
                                            int32_t, int32_t, int32_t, int32_t,
                                            int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t CreateContract(zen::runtime::Instance *Inst, int32_t, int32_t,
                              int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetConfidentialDeposit(zen::runtime::Instance *Inst, int32_t,
                                      int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetTransaction(zen::runtime::Instance *Inst, int32_t, int32_t,
                              int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetRelatedTransactionListSize(zen::runtime::Instance *Inst,
                                             int32_t, int32_t, int64_t,
                                             int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetRelatedTransactionList(zen::runtime::Instance *Inst, int32_t,
                                         int32_t, int64_t, int64_t, int32_t,
                                         int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t ReadBuffer(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t ReadBufferRef(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t LiftedElgamalContractHomomorphicAdd(zen::runtime::Instance *Inst,
                                                   int32_t, int32_t, int32_t,
                                                   int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t LiftedElgamalContractHomomorphicSub(zen::runtime::Instance *Inst,
                                                   int32_t, int32_t, int32_t,
                                                   int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t LiftedElgamalScalarMutiply(zen::runtime::Instance *Inst, int32_t,
                                          int32_t, int64_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t
LiftedElgamalContractZeroCheckVerify(zen::runtime::Instance *Inst, int32_t,
                                     int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t LiftedElgamalContractRangeVerify(zen::runtime::Instance *Inst,
                                                int32_t, int32_t, int32_t,
                                                int32_t, int32_t, int32_t,
                                                int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t FTraceBegin(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t FTraceEnd(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t BellmanSnarkVerify(zen::runtime::Instance *Inst, int32_t,
                                  int32_t, int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t DeployContract(zen::runtime::Instance *Inst, int32_t, int32_t,
                              int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t UpdateContract(zen::runtime::Instance *Inst, int32_t, int32_t,
                              int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t UpdateContractStatus(zen::runtime::Instance *Inst, int32_t,
                                    int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t __call_evm__(zen::runtime::Instance *Inst, int32_t, int32_t,
                            int32_t, int32_t, int32_t, int32_t, int64_t,
                            int64_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t DCGetStorageSize(zen::runtime::Instance *Inst, int32_t, int32_t,
                                int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t DCSetStorage(zen::runtime::Instance *Inst, int32_t, int32_t,
                            int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t DCDeleteStorage(zen::runtime::Instance *Inst, int32_t, int32_t,
                               int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GrayscaleDeployContract(zen::runtime::Instance *Inst, int32_t,
                                       int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GrayscaleVerification(zen::runtime::Instance *Inst, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GrayscaleVersionSwitchBack(zen::runtime::Instance *Inst,
                                          int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GrayscaleUpdateContract(zen::runtime::Instance *Inst, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t GetDigestType(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t DCSetAcl(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t,
                        int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t VerifyPrivateDKGInfo(zen::runtime::Instance *Inst, int32_t,
                                    int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t CalculateBlsPubkeyAndShares(zen::runtime::Instance *Inst,
                                           int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static void GetBlockRandomSeed(zen::runtime::Instance *Inst, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

// another chain hostapis

static void write_object(zen::runtime::Instance *Inst, int32_t, int32_t,
                         int32_t, int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static void delete_object(zen::runtime::Instance *Inst, int32_t, int32_t,
                          int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static int32_t read_object_length(zen::runtime::Instance *Inst, int32_t,
                                  int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static void read_object(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t,
                        int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static int32_t co_call(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t,
                       int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t lib_call(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t,
                        int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static void revert(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static void get_call_argpack(zen::runtime::Instance *Inst, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static int32_t get_call_argpack_length(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static void get_call_sender(zen::runtime::Instance *Inst, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static int32_t get_call_sender_length(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static void get_call_contract(zen::runtime::Instance *Inst, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static int32_t get_call_contract_length(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static void set_call_result(zen::runtime::Instance *Inst, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static void get_call_result(zen::runtime::Instance *Inst, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static int32_t get_call_result_length(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static void log(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t,
                int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}
static int64_t get_block_number(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int64_t get_block_timestamp(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t get_tx_index(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static void get_tx_sender(zen::runtime::Instance *Inst, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static int32_t get_tx_sender_length(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static void get_tx_hash(zen::runtime::Instance *Inst, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static int32_t get_tx_hash_length(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int64_t get_tx_timestamp(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int64_t get_tx_nonce(zen::runtime::Instance *Inst) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}
static void sha256(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static void sm3(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static void keccak256(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static int32_t verify_mycrypto_signature(zen::runtime::Instance *Inst, int32_t,
                                         int32_t, int32_t, int32_t, int32_t,
                                         int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static int32_t eth_secp256k1_recovery(zen::runtime::Instance *Inst, int32_t,
                                      int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static void issue_asset(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t,
                        int32_t, int32_t, int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static void burn_asset(zen::runtime::Instance *Inst, int32_t, int32_t, int32_t,
                       int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static void destroy_asset(zen::runtime::Instance *Inst, int32_t, int32_t,
                          int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static void transfer_asset(zen::runtime::Instance *Inst, int32_t, int32_t,
                           int32_t, int32_t, int32_t, int32_t, int32_t, int32_t,
                           int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static void get_asset_data(zen::runtime::Instance *Inst, int32_t, int32_t,
                           int32_t, int32_t, int32_t, int32_t, int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static int32_t get_asset_data_length(zen::runtime::Instance *Inst, int32_t,
                                     int32_t, int32_t, int32_t, int32_t,
                                     int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

static void set_asset_data(zen::runtime::Instance *Inst, int32_t, int32_t,
                           int32_t, int32_t, int32_t, int32_t, int32_t,
                           int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static void get_fungible_asset_balance(zen::runtime::Instance *Inst, int32_t,
                                       int32_t, int32_t, int32_t, int32_t,
                                       int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static void get_fungible_asset_tag(zen::runtime::Instance *Inst, int32_t,
                                   int32_t, int32_t, int32_t, int32_t,
                                   int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
}

static int32_t get_fungible_asset_tag_length(zen::runtime::Instance *Inst,
                                             int32_t, int32_t, int32_t, int32_t,
                                             int32_t) {
  MOCK_CHAIN_DUMMY_IMPLEMENTATION
  return 0;
}

#define MOCK_CHAIN_HOST_API_LIST                                               \
  NATIVE_FUNC_ENTRY(MyAbort)                                                   \
  NATIVE_FUNC_ENTRY(memcpy)                                                    \
  NATIVE_FUNC_ENTRY(wmemcpy)                                                   \
  NATIVE_FUNC_ENTRY(memset)                                                    \
  NATIVE_FUNC_ENTRY(wmemset)                                                   \
  NATIVE_FUNC_ENTRY(strlen)                                                    \
  NATIVE_FUNC_ENTRY(wcslen)                                                    \
  NATIVE_FUNC_ENTRY(strtoll)                                                   \
  NATIVE_FUNC_ENTRY(strtoull)                                                  \
  NATIVE_FUNC_ENTRY(__strchrnul)                                               \
  NATIVE_FUNC_ENTRY(strncmp)                                                   \
  NATIVE_FUNC_ENTRY(memmove)                                                   \
  NATIVE_FUNC_ENTRY(wmemmove)                                                  \
  NATIVE_FUNC_ENTRY(memchr)                                                    \
  NATIVE_FUNC_ENTRY(memcmp)                                                    \
  NATIVE_FUNC_ENTRY(SetStorage)                                                \
  NATIVE_FUNC_ENTRY(GetStorageSize)                                            \
  NATIVE_FUNC_ENTRY(DeleteStorage)                                             \
  NATIVE_FUNC_ENTRY(strcmp)                                                    \
  NATIVE_FUNC_ENTRY(GetSender)                                                 \
  NATIVE_FUNC_ENTRY(Log)                                                       \
  NATIVE_FUNC_ENTRY(print)                                                     \
  NATIVE_FUNC_ENTRY(GetBlockHash)                                              \
  NATIVE_FUNC_ENTRY(GetBlockNumber)                                            \
  NATIVE_FUNC_ENTRY(GetBlockTimeStamp)                                         \
  NATIVE_FUNC_ENTRY(GetStorage)                                                \
  NATIVE_FUNC_ENTRY(GetCode)                                                   \
  NATIVE_FUNC_ENTRY(GetDataSize)                                               \
  NATIVE_FUNC_ENTRY(GetData)                                                   \
  NATIVE_FUNC_ENTRY(GetTxHash)                                                 \
  NATIVE_FUNC_ENTRY(GetGas)                                                    \
  NATIVE_FUNC_ENTRY(GetValue)                                                  \
  NATIVE_FUNC_ENTRY(GetOrigin)                                                 \
  NATIVE_FUNC_ENTRY(CheckAccount)                                              \
  NATIVE_FUNC_ENTRY(IsLocalTx)                                                 \
  NATIVE_FUNC_ENTRY(GetAccountStatus)                                          \
  NATIVE_FUNC_ENTRY(GetCodeHash)                                               \
  NATIVE_FUNC_ENTRY(GetAuthMap)                                                \
  NATIVE_FUNC_ENTRY(GetAuthMapInCache)                                         \
  NATIVE_FUNC_ENTRY(GetBalance)                                                \
  NATIVE_FUNC_ENTRY(TransferBalance)                                           \
  NATIVE_FUNC_ENTRY(Result)                                                    \
  NATIVE_FUNC_ENTRY(GetSelf)                                                   \
  NATIVE_FUNC_ENTRY(GetRecoverKey)                                             \
  NATIVE_FUNC_ENTRY(ReadInterfaceName)                                         \
  NATIVE_FUNC_ENTRY(ReadInterfaceNameSize)                                     \
  NATIVE_FUNC_ENTRY(ReadInterfaceParams)                                       \
  NATIVE_FUNC_ENTRY(ReadInterfaceParamsSize)                                   \
  NATIVE_FUNC_ENTRY(CallContract)                                              \
  NATIVE_FUNC_ENTRY(DelegateCall)                                              \
  NATIVE_FUNC_ENTRY(VerifyCommitment)                                          \
  NATIVE_FUNC_ENTRY(VerifyRange)                                               \
  NATIVE_FUNC_ENTRY(VerifyBalance)                                             \
  NATIVE_FUNC_ENTRY(Ecrecovery)                                                \
  NATIVE_FUNC_ENTRY(Digest)                                                    \
  NATIVE_FUNC_ENTRY(VerifyRsa)                                                 \
  NATIVE_FUNC_ENTRY(VerifyRsa2)                                                \
  NATIVE_FUNC_ENTRY(VerifyMessageSM2)                                          \
  NATIVE_FUNC_ENTRY(Base64Encode)                                              \
  NATIVE_FUNC_ENTRY(Base64Decode)                                              \
  NATIVE_FUNC_ENTRY(println)                                                   \
  NATIVE_FUNC_ENTRY(SetReturnValue)                                            \
  NATIVE_FUNC_ENTRY(GetCallResult)                                             \
  NATIVE_FUNC_ENTRY(GetCallResultSize)                                         \
  NATIVE_FUNC_ENTRY(VerifyMessageECCK1)                                        \
  NATIVE_FUNC_ENTRY(VerifyMessageECCR1)                                        \
  NATIVE_FUNC_ENTRY(RangeProofVerify)                                          \
  NATIVE_FUNC_ENTRY(AddPedersenCommit)                                         \
  NATIVE_FUNC_ENTRY(SubPedersenCommit)                                         \
  NATIVE_FUNC_ENTRY(CalculatePedersenCommit)                                   \
  NATIVE_FUNC_ENTRY(PedersenCommitEqualityVerify)                              \
  NATIVE_FUNC_ENTRY(CreateContract)                                            \
  NATIVE_FUNC_ENTRY(GetConfidentialDeposit)                                    \
  NATIVE_FUNC_ENTRY(GetTransaction)                                            \
  NATIVE_FUNC_ENTRY(GetRelatedTransactionListSize)                             \
  NATIVE_FUNC_ENTRY(GetRelatedTransactionList)                                 \
  NATIVE_FUNC_ENTRY(ReadBuffer)                                                \
  NATIVE_FUNC_ENTRY(ReadBufferRef)                                             \
  NATIVE_FUNC_ENTRY(LiftedElgamalContractHomomorphicAdd)                       \
  NATIVE_FUNC_ENTRY(LiftedElgamalContractHomomorphicSub)                       \
  NATIVE_FUNC_ENTRY(LiftedElgamalScalarMutiply)                                \
  NATIVE_FUNC_ENTRY(LiftedElgamalContractZeroCheckVerify)                      \
  NATIVE_FUNC_ENTRY(LiftedElgamalContractRangeVerify)                          \
  NATIVE_FUNC_ENTRY(FTraceBegin)                                               \
  NATIVE_FUNC_ENTRY(FTraceEnd)                                                 \
  NATIVE_FUNC_ENTRY(BellmanSnarkVerify)                                        \
  NATIVE_FUNC_ENTRY(DeployContract)                                            \
  NATIVE_FUNC_ENTRY(UpdateContract)                                            \
  NATIVE_FUNC_ENTRY(UpdateContractStatus)                                      \
  NATIVE_FUNC_ENTRY(__call_evm__)                                              \
  NATIVE_FUNC_ENTRY(DCGetStorageSize)                                          \
  NATIVE_FUNC_ENTRY(DCSetStorage)                                              \
  NATIVE_FUNC_ENTRY(DCDeleteStorage)                                           \
  NATIVE_FUNC_ENTRY(GrayscaleDeployContract)                                   \
  NATIVE_FUNC_ENTRY(GrayscaleVerification)                                     \
  NATIVE_FUNC_ENTRY(GrayscaleVersionSwitchBack)                                \
  NATIVE_FUNC_ENTRY(GrayscaleUpdateContract)                                   \
  NATIVE_FUNC_ENTRY(VerifyPrivateDKGInfo)                                      \
  NATIVE_FUNC_ENTRY(GetDigestType)                                             \
  NATIVE_FUNC_ENTRY(DCSetAcl)                                                  \
  NATIVE_FUNC_ENTRY(CalculateBlsPubkeyAndShares)                               \
  NATIVE_FUNC_ENTRY(GetBlockRandomSeed)                                        \
  NATIVE_FUNC_ENTRY(write_object)                                              \
  NATIVE_FUNC_ENTRY(delete_object)                                             \
  NATIVE_FUNC_ENTRY(read_object_length)                                        \
  NATIVE_FUNC_ENTRY(read_object)                                               \
  NATIVE_FUNC_ENTRY(co_call)                                                   \
  NATIVE_FUNC_ENTRY(lib_call)                                                  \
  NATIVE_FUNC_ENTRY(revert)                                                    \
  NATIVE_FUNC_ENTRY(get_call_argpack)                                          \
  NATIVE_FUNC_ENTRY(get_call_argpack_length)                                   \
  NATIVE_FUNC_ENTRY(get_call_sender)                                           \
  NATIVE_FUNC_ENTRY(get_call_sender_length)                                    \
  NATIVE_FUNC_ENTRY(get_call_contract)                                         \
  NATIVE_FUNC_ENTRY(get_call_contract_length)                                  \
  NATIVE_FUNC_ENTRY(set_call_result)                                           \
  NATIVE_FUNC_ENTRY(get_call_result)                                           \
  NATIVE_FUNC_ENTRY(get_call_result_length)                                    \
  NATIVE_FUNC_ENTRY(log)                                                       \
  NATIVE_FUNC_ENTRY(get_block_number)                                          \
  NATIVE_FUNC_ENTRY(get_block_timestamp)                                       \
  NATIVE_FUNC_ENTRY(get_tx_index)                                              \
  NATIVE_FUNC_ENTRY(get_tx_sender)                                             \
  NATIVE_FUNC_ENTRY(get_tx_sender_length)                                      \
  NATIVE_FUNC_ENTRY(get_tx_hash)                                               \
  NATIVE_FUNC_ENTRY(get_tx_hash_length)                                        \
  NATIVE_FUNC_ENTRY(get_tx_timestamp)                                          \
  NATIVE_FUNC_ENTRY(get_tx_nonce)                                              \
  NATIVE_FUNC_ENTRY(sha256)                                                    \
  NATIVE_FUNC_ENTRY(sm3)                                                       \
  NATIVE_FUNC_ENTRY(keccak256)                                                 \
  NATIVE_FUNC_ENTRY(verify_mycrypto_signature)                                 \
  NATIVE_FUNC_ENTRY(eth_secp256k1_recovery)                                    \
  NATIVE_FUNC_ENTRY(issue_asset)                                               \
  NATIVE_FUNC_ENTRY(burn_asset)                                                \
  NATIVE_FUNC_ENTRY(destroy_asset)                                             \
  NATIVE_FUNC_ENTRY(transfer_asset)                                            \
  NATIVE_FUNC_ENTRY(get_asset_data)                                            \
  NATIVE_FUNC_ENTRY(get_asset_data_length)                                     \
  NATIVE_FUNC_ENTRY(set_asset_data)                                            \
  NATIVE_FUNC_ENTRY(get_fungible_asset_balance)                                \
  NATIVE_FUNC_ENTRY(get_fungible_asset_tag)                                    \
  NATIVE_FUNC_ENTRY(get_fungible_asset_tag_length)
