/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct Address Address;

typedef struct DataInputBoxes DataInputBoxes;

typedef struct ErgoBoxCandidate ErgoBoxCandidate;

typedef struct ErgoStateContext ErgoStateContext;

typedef struct Error Error;

typedef struct OutputBoxes OutputBoxes;

typedef struct SecretKey SecretKey;

typedef struct Transaction Transaction;

typedef struct UnspentBoxes UnspentBoxes;

typedef Error *ErrorPtr;

typedef Address *AddressPtr;

typedef Transaction *TransactionPtr;

typedef ErgoBoxCandidate *ErgoBoxCandidatePtr;

typedef ErgoStateContext *ErgoStateContextPtr;

typedef UnspentBoxes *UnspentBoxesPtr;

typedef DataInputBoxes *DataInputBoxesPtr;

typedef OutputBoxes *OutputBoxesPtr;

typedef SecretKey *SecretKeyPtr;

ErrorPtr ergo_wallet_address_delete(AddressPtr _address);

ErrorPtr ergo_wallet_address_from_testnet(const char *_address_str, AddressPtr *_address_out);

void ergo_wallet_delete_error(ErrorPtr error);

ErrorPtr ergo_wallet_delete_signed_tx(TransactionPtr _transaction);

void ergo_wallet_delete_string(char *ptr);

ErrorPtr ergo_wallet_ergo_box_candidate_delete(ErgoBoxCandidatePtr _ergo_box_candidate);

ErrorPtr ergo_wallet_ergo_box_candidate_new_pay_to_address(AddressPtr _recipient,
                                                           uint64_t _value,
                                                           uint32_t _creation_height,
                                                           ErgoBoxCandidatePtr *_ergo_box_candidate_out);

ErrorPtr ergo_wallet_ergo_state_context_delete(ErgoStateContextPtr _ergo_state_context);

ErrorPtr ergo_wallet_ergo_state_context_from_json(const char *_json_str,
                                                  ErgoStateContextPtr *_ergo_state_context_out);

char *ergo_wallet_error_to_string(ErrorPtr error);

ErrorPtr ergo_wallet_new_signed_tx(ErgoStateContextPtr _state_context,
                                   UnspentBoxesPtr _unspent_boxes,
                                   DataInputBoxesPtr _data_input_boxes,
                                   OutputBoxesPtr _output_boxes,
                                   AddressPtr _send_change_to,
                                   uint64_t _min_change_value,
                                   uint64_t _tx_fee_amount,
                                   SecretKeyPtr _sk,
                                   TransactionPtr *_transaction_out);

ErrorPtr ergo_wallet_output_boxes_delete(OutputBoxesPtr _output_boxes);

ErrorPtr ergo_wallet_output_boxes_new(ErgoBoxCandidatePtr _ergo_box_candidate,
                                      OutputBoxesPtr *_output_boxes_out);

ErrorPtr ergo_wallet_secret_key_delete(SecretKeyPtr _secret_key);

ErrorPtr ergo_wallet_secret_key_parse_str(const char *_secret_key_str,
                                          SecretKeyPtr *_secret_key_out);

ErrorPtr ergo_wallet_signed_tx_to_json(TransactionPtr _transaction, const char **_json_str_out);

ErrorPtr ergo_wallet_unspent_boxes_delete(UnspentBoxesPtr _unspent_boxes);

ErrorPtr ergo_wallet_unspent_boxes_from_json(const char *_json_str,
                                             UnspentBoxesPtr *_unspent_boxes_out);
